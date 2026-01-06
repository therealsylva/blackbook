use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct UserTarget {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3))]
    pub phone: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProfilePic {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserProfile {
    pub username: String,
    pub full_name: String,
    #[serde(alias = "pk")]
    pub user_id: u64,
    #[serde(default)]
    pub is_private: bool,
    #[serde(default)]
    pub is_verified: bool,
    #[serde(default)]
    pub follower_count: u64,
    #[serde(default)]
    pub following_count: u64,
    #[serde(default)]
    pub external_url: String,
    #[serde(default)]
    pub biography: String,
    #[serde(default)]
    pub public_email: String,
    #[serde(default)]
    pub public_phone_number: String,
    #[serde(default)]
    pub media_count: u64,
    pub hd_profile_pic_url_info: ProfilePic,
}

#[derive(Debug, Deserialize, Default)]
pub struct LookupResponse {
    #[serde(default)]
    #[allow(dead_code)]
    pub message: Option<String>,
    #[serde(default)]
    pub obfuscated_email: Option<String>,
    #[serde(default)]
    pub obfuscated_phone: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct FinalResult {
    pub username: String,
    pub user_id: u64,
    pub full_name: String,
    pub is_verified: bool,
    pub is_private: bool,
    pub followers: u64,
    pub following: u64,
    pub posts: u64,
    pub bio: String,
    pub external_url: String,
    pub profile_pic: String,
    pub match_score: u8,
    pub details: Option<LookupDetails>,
}

#[derive(Debug, Serialize, Clone)]
pub struct LookupDetails {
    pub public_email: Option<String>,
    pub public_phone: Option<String>,
    pub obfuscated_email: Option<String>,
    pub obfuscated_phone: Option<String>,
  }
