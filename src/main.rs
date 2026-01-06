mod client;
mod config;
mod models;
mod output;
mod scrapers;
mod validators;

use clap::Parser;
use color_eyre::eyre::Result;
use colored::Colorize;
use dotenv::dotenv;
use models::FinalResult;
use reqwest::Client;
use std::env;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use validators::validate_input;

#[derive(Parser, Debug)]
#[command(author = "blackeko5", version, about = "Blackbook: Advanced Instagram OSINT Tool", long_about = None)]
struct Args {
    #[clap(short, long)]
    name: String,

    #[clap(short, long)]
    email: String,

    #[clap(short, long)]
    phone: String,

    #[clap(short, long, default_value = "0")]
    timeout: u64,

    #[clap(long, default_value = "false")]
    json: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::WARN)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let args = Args::parse();
    let session_id = env::var("SESSION_ID").expect("SESSION_ID must be set in environment variables");

    let banner = r#"
                _ _   _                
  _  _ ___ ___ (_) |_( )___  _ __  ___ 
 | || / -_|_-< | |  _|/(_-< | '  \/ -_)
  \_, \___/__/ |_|\__| /__/ |_|_|_\___|
  |__/                                 
    "#;

    if !args.json {
        println!("{}", banner.bright_cyan());
        println!("\tTwitter: @blackeko5\n");
    }

    let target = models::UserTarget {
        name: args.name.clone(),
        email: args.email.clone(),
        phone: args.phone.clone(),
    };

    validate_input(&target)?;

    let http_client = Client::new();
    let ig_client = client::InstagramClient::new(&session_id).await?;

    let candidates = scrapers::search_candidates(&http_client, &target.name).await?;

    if candidates.is_empty() {
        if !args.json {
            eprintln!("{} No candidates found.", "[!]".red());
        }
        return Ok(());
    }

    for username in candidates {
        let clean_username = username.trim_start_matches('@');

        let (profile_res, lookup_res) = tokio::join!(
            ig_client.get_full_info(clean_username),
            ig_client.advanced_lookup(clean_username)
        );

        if let Ok(Some(profile)) = profile_res {
            let details = Some(models::LookupDetails {
                public_email: if profile.public_email.is_empty() { None } else { Some(profile.public_email.clone()) },
                public_phone: if profile.public_phone_number.is_empty() { None } else { Some(profile.public_phone_number.clone()) },
                obfuscated_email: lookup_res.as_ref().ok().and_then(|r| r.obfuscated_email.clone()),
                obfuscated_phone: lookup_res.as_ref().ok().and_then(|r| r.obfuscated_phone.clone()),
            });

            let result = FinalResult {
                username: profile.username.clone(),
                user_id: profile.user_id,
                full_name: profile.full_name,
                is_verified: profile.is_verified,
                is_private: profile.is_private,
                followers: profile.follower_count,
                following: profile.following_count,
                posts: profile.media_count,
                bio: profile.biography,
                external_url: profile.external_url,
                profile_pic: profile.hd_profile_pic_url_info.url,
                match_score: 0,
                details,
            };

            let stop = output::print_output(&result, &target.name, &target.email, &target.phone, args.json);
            
            if stop {
                break;
            }
        }

        if args.timeout > 0 {
            tokio::time::sleep(std::time::Duration::from_secs(args.timeout)).await;
        }
    }

    Ok(())
      }
