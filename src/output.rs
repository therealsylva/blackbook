use crate::models::FinalResult;
use colored::Colorize;

pub fn print_output(result: &FinalResult, target_name: &str, target_email: &str, target_phone: &str, format_json: bool) -> bool {
    if format_json {
        println!("{}", serde_json::to_string(result).unwrap());
        return false; 
    }

    let mut name_f = 0;
    let mut email_f = 0;
    let mut phone_f = 0;

    println!("Information about      : {}", result.username);
    
    if result.full_name.to_lowercase() == target_name.to_lowercase() {
        println!("{} Full Name              : {} {}", "[+]".green(), result.full_name, "\u{2713}");
        name_f = 1;
    } else {
        println!("Full Name              : {}", result.full_name);
    }

    println!("User ID                : {}", result.user_id);
    println!("Verified               : {}", result.is_verified);
    println!("Is private Account     : {}", result.is_private);
    println!("Followers              : {}", result.followers);
    println!("Following              : {}", result.following);
    println!("Number of posts        : {}", result.posts);
    println!("External URL           : {}", result.external_url);
    println!("Biography              : {}", result.bio);

    if let Some(details) = &result.details {
        if let Some(pub_email) = &details.public_email {
            if !pub_email.is_empty() && check_email(pub_email, target_email) {
                println!("{} Public email           : {} {}", "[+]".green(), pub_email, "\u{2713}");
                email_f = 1;
            } else {
                println!("Public email           : {}", pub_email);
            }
        }

        if let Some(pub_phone) = &details.public_phone {
            if !pub_phone.is_empty() && check_phone(pub_phone, target_phone) {
                println!("{} Public phone number    : {} {}", "[+]".green(), pub_phone, "\u{2713}");
                phone_f = 1;
            } else {
                println!("Public phone           : {}", pub_phone);
            }
        }

        if let Some(obs_email) = &details.obfuscated_email {
            if !obs_email.is_empty() && check_email(obs_email, target_email) {
                println!("{} Obfuscated email       : {} {}", "[+]".green(), obs_email, "\u{2713}");
                email_f = 1;
            } else {
                println!("Obfuscated email       : {}", obs_email);
            }
        }

        if let Some(obs_phone) = &details.obfuscated_phone {
            if !obs_phone.is_empty() && check_phone(obs_phone, target_phone) {
                println!("{} Obfuscated phone       : {} {}", "[+]".green(), obs_phone, "\u{2713}");
                phone_f = 1;
            } else {
                println!("Obfuscated phone       : {}", obs_phone);
            }
        }
    }

    println!("Profile Picture        : {}", result.profile_pic);

    let score = name_f + email_f + phone_f;
    let stop_search = score == 3;

    if score == 3 {
        println!("{} Profile ID {} match level: {}", "[*]".cyan(), result.user_id, "HIGH".green().bold());
    } else if score == 2 {
        println!("{} Profile ID {} match level: {}", "[*]".cyan(), result.user_id, "MEDIUM".yellow());
    } else if score == 1 {
        println!("{} Profile ID {} match level: {}", "[*]".cyan(), result.user_id, "LOW".red());
    }

    println!("{}", "-".repeat(30));
    
    stop_search
}

fn check_email(public: &str, target: &str) -> bool {
    if target.is_empty() || public.is_empty() { return false; }
    
    let pub_parts: Vec<&str> = public.split('@').collect();
    let tgt_parts: Vec<&str> = target.split('@').collect();
    
    if pub_parts.len() != 2 || tgt_parts.len() != 2 { return false; }
    
    let pub_local = pub_parts[0];
    let tgt_local = tgt_parts[0];
    
    let pub_domain = pub_parts[1];
    let tgt_domain = tgt_parts[1];
    
    if pub_domain != tgt_domain { return false; }
    if pub_local.chars().next() != tgt_local.chars().next() { return false; }
    if pub_local.chars().last() != tgt_local.chars().last() { return false; }
    
    true
}

fn check_phone(public: &str, target: &str) -> bool {
    if target.is_empty() || public.is_empty() { return false; }
    
    let parts: Vec<&str> = public.split_whitespace().collect();
    if parts.is_empty() { return false; }
    
    let first = parts[0];
    let last = &public[public.len().saturating_sub(2)..];
    
    let target_parts: Vec<&str> = target.split_whitespace().collect();
    let target_first = target_parts.get(0).unwrap_or(&target);
    let target_last = &target[target.len().saturating_sub(2)..];

    first == *target_first && last == target_last
          }
