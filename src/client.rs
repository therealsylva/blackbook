 use crate::config::{API_URL, BASE_URL, IG_SIG_KEY, SIG_KEY_VERSION, USER_AGENT_LOOKUP, USER_AGENT_WEB};
 use crate::models::{LookupResponse, UserProfile};
 use color_eyre::Result;
 use governor::{Quota, RateLimiter};
 use governor::state::{InMemoryState, NotKeyed};
 use governor::clock::DefaultClock;
 use hmac::{Hmac, Mac};
 use nonzero_ext::nonzero;
 use reqwest::Client;
 use sha2::Sha256;
 use std::sync::Arc;
 use tokio::time::{sleep, Duration};
 use tracing::warn;
 use urlencoding::encode;

 type HmacSha256 = Hmac<Sha256>;

 pub struct InstagramClient {
     client: Client,
     rate_limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
 }

impl InstagramClient {
    pub async fn new(session_id: &str) -> Result<Self> {
        let client = Client::builder()
            .user_agent(USER_AGENT_WEB)
            .cookie_store(true)
            .build()?;

        let rate_limiter = Arc::new(RateLimiter::direct(Quota::per_second(nonzero!(1u32))));

        let ig = Self { client, rate_limiter };
        
        ig.validate_session(session_id).await?;
        ig.set_cookie(session_id).await;
        
        Ok(ig)
    }

    async fn set_cookie(&self, session_id: &str) {
        let cookie_header = format!("sessionid={}", session_id);
        let url: reqwest::Url = BASE_URL.parse().unwrap();
        self.client.get(url).header("Cookie", cookie_header).send().await.ok();
    }

    async fn validate_session(&self, session_id: &str) -> Result<()> {
        self.set_cookie(session_id).await;
        let res = self.client
            .get(&format!("{}/accounts/current_user/?__a=1", BASE_URL))
            .send()
            .await?;

        if !res.status().is_success() {
            Err(color_eyre::eyre::eyre!("Session ID validation failed. Check credentials."))
        } else {
            Ok(())
        }
    }

    pub async fn get_full_info(&self, username: &str) -> Result<Option<UserProfile>> {
        self.wait_for_quota().await;
        
        let url = format!("{}/{}//?__a=1", BASE_URL, username);
        
        let res = self.client.get(&url).send().await;
        
        match res {
            Ok(r) => {
                if r.status().is_success() {
                    let json: serde_json::Value = r.json().await?;
                    if let Some(logging_page_id) = json.get("logging_page_id").and_then(|v| v.as_str()) {
                        let id = logging_page_id.trim_start_matches("profilePage_");
                        return self.fetch_user_details(id).await;
                    }
                }
                Ok(None)
            }
            Err(e) => {
                warn!("Request failed for {}: {}", username, e);
                Ok(None)
            }
        }
    }

    async fn fetch_user_details(&self, user_id: &str) -> Result<Option<UserProfile>> {
        self.wait_for_quota().await;

        let url = format!("{}/users/{}/info/", API_URL, user_id);
        
        let res = self.client.get(&url).send().await?;

        if res.status().is_success() {
            let json: serde_json::Value = res.json().await?;
            if let Some(user) = json.get("user") {
                let profile: UserProfile = serde_json::from_value(user.clone())?;
                return Ok(Some(profile));
            }
        }
        Ok(None)
    }

    pub async fn advanced_lookup(&self, query: &str) -> Result<LookupResponse> {
        let data = serde_json::json!({
            "login_attempt_count": "0",
            "directly_sign_in": "true",
            "source": "default",
            "q": query,
            "ig_sig_key_version": SIG_KEY_VERSION
        });

        let data_str = data.to_string();
        let signature = generate_signature(&data_str, IG_SIG_KEY, SIG_KEY_VERSION);

        let client = Client::new(); 
        
        let mut retry_count = 0;
        loop {
            let res = client
                .post(&format!("{}/users/lookup/", API_URL))
                .header("User-Agent", USER_AGENT_LOOKUP)
                .header("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8")
                .body(signature.clone())
                .send()
                .await?;

            if res.status() == 429 {
                if retry_count >= 3 {
                    return Err(color_eyre::eyre::eyre!("Rate limit exceeded after retries"));
                }
                let wait_time = 2u64.pow(retry_count + 1);
                warn!("Rate limited. Sleeping for {}s...", wait_time);
                sleep(Duration::from_secs(wait_time)).await;
                retry_count += 1;
            } else {
                let resp_text = res.text().await?;
                let result: LookupResponse = serde_json::from_str(&resp_text).unwrap_or_default();
                return Ok(result);
            }
        }
    }

    async fn wait_for_quota(&self) {
        self.rate_limiter.until_ready().await;
    }
}

fn generate_signature(data: &str, key: &str, version: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(key.as_bytes()).unwrap();
    mac.update(data.as_bytes());
    let result = mac.finalize();
    let hex_hash = hex::encode(result.into_bytes());
    
    format!(
        "ig_sig_key_version={}&signed_body={}.{}",
        version,
        hex_hash,
        encode(data)
    )
      }
