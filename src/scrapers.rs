use crate::config::{DUMPOR_URL, USER_AGENT_SCRAPER};
use color_eyre::Result;
use reqwest::Client;
use scraper::{Html, Selector};
use tracing::debug;

pub async fn search_candidates(client: &Client, name: &str) -> Result<Vec<String>> {
    let url = format!("{}{}", DUMPOR_URL, name.replace(" ", "+"));
    
    let res = client
        .get(&url)
        .header("User-Agent", USER_AGENT_SCRAPER)
        .send()
        .await?;

    let body = res.text().await?;
    let document = Html::parse_document(&body);
    
    let selector = Selector::parse("a.profile-name-link").unwrap();
    
    let mut usernames = Vec::new();
    for element in document.select(&selector) {
        if let Some(text) = element.text().next() {
            usernames.push(text.to_string());
        }
    }
    
    debug!("Found {} candidates", usernames.len());
    Ok(usernames)
          }
