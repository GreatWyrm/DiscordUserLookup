use std::collections::HashSet;

use reqwest::blocking::Client;
use reqwest::header::HeaderValue;
use serde::Deserialize;

mod cache;
use crate::cache::CachedUserList;

// See https://docs.discord.com/developers/resources/user#user-resource
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct DiscordUser {
    id: String, // Techically a u64, but represented as a string
    username: String,
    global_name: Option<String>,
    avatar: Option<String>,
    bot: Option<bool>,
    system: Option<bool>,
    mfa_enabled: Option<bool>,
    banner: Option<String>,
    accent_color: Option<i32>,
    locale: Option<String>,
    verified: Option<bool>,
    email: Option<String>,
    flags: Option<i32>,
    premium_type: Option<i32>,
    public_flags: Option<i32>,
    // TODO: Avatar Decoration Data
    // TODO: Collectibles
    // TODO: Prumary Guild
    discriminator: String,
}

enum PremiumType {
    None,
    NitroClassic,
    Nitro,
    NitroBasic,
}

#[allow(dead_code)]
impl DiscordUser {
    pub fn get_id_as_u64(self) -> Option<u64> {
        match self.id.parse::<u64>() {
            Ok(value) => Some(value),
            Err(_e) => None,
        }
    }

    pub fn get_premium_type(self) -> Option<PremiumType> {
        match self.premium_type {
            Some(value) => match value {
                1 => Some(PremiumType::NitroClassic),
                2 => Some(PremiumType::Nitro),
                3 => Some(PremiumType::NitroBasic),
                _ => Some(PremiumType::None),
            },
            None => None,
        }
    }
}

// Looks up the username first in the cache list, and if not, contacts the API and saves the result
fn lookup_username_with_cache(
    cached_list: &mut CachedUserList,
    bot_token: &str,
    user_id: u64,
) -> Result<String, reqwest::Error> {
    let cache_lookup = cached_list.lookup_user(user_id);

    match cache_lookup {
        Some(name) => {
            println!("Hit cache for {}", name);
            return Ok(name);
        }
        None => {
            let lookup_result = lookup_user(bot_token, user_id);
            match lookup_result {
                Ok(user) => {
                    println!("Made API call for {}", &user.username);
                    return Ok(user.username);
                }
                Err(e) => Err(e),
            }
        }
    }
}

// Always calls the API
fn lookup_user(bot_token: &str, user_id: u64) -> Result<DiscordUser, reqwest::Error> {
    // Create sensitive header value
    let mut auth_value = HeaderValue::from_str(format!("Bot {}", bot_token).as_str()).unwrap();
    auth_value.set_sensitive(true);

    let client = Client::new();
    let request = client
        .get(format!("https://discord.com/api/v9/users/{}", user_id))
        .header(reqwest::header::AUTHORIZATION, auth_value);

    let response = request.send();

    match response {
        Ok(result) => result.json::<DiscordUser>(),
        Err(err) => Err(err),
    }
}

fn parse_cli_ids(bot_token: &str, input: Vec<&String>) -> () {
    for arg in input.iter() {
        // Try parse to u64
        let result = arg.parse::<u64>();
        match result {
            Ok(value) => {
                let discord_user = lookup_user(&bot_token, value);
                match discord_user {
                    Ok(user) => println!(
                        "ID: {}, Username: {}, Global Name: {:?}",
                        user.id, user.username, user.global_name
                    ),
                    Err(e) => println!("Failed to retrieve user: {}", e),
                }
            }
            Err(e) => {
                println!("Failed to parse {} as a u64. {}", arg, e);
            }
        }
        // Sleep to avoid API limits
        std::thread::sleep(std::time::Duration::from_millis(3));
    }
}

fn parse_file(cached_list: &mut CachedUserList, bot_token: &str, file_name: String) -> () {
    let file_contents_result = std::fs::read_to_string(file_name);
    let mut usernames: HashSet<String> = HashSet::new();
    match file_contents_result {
        Ok(file_contents) => {
            let lines: Vec<&str> = file_contents.lines().collect();
            // Delimiter is a ';', strangely enough
            for line in lines {
                let tokens = line.split(";");
                for token in tokens {
                    // Try parse as number
                    let result = token.parse::<u64>();
                    match result {
                        Ok(value) => {
                            if value < 10000000000000000 {
                                // Ids are a 19 digit number, discard anything below that
                                continue;
                            }
                            let result = lookup_username_with_cache(cached_list, &bot_token, value);
                            match result {
                                Ok(username) => {
                                    usernames.insert(username.clone());
                                    cached_list.save_to_cache(value, &username);
                                }
                                Err(e) => println!("Failed to retrieve user: {}", e),
                            }
                        }
                        Err(_e) => {}
                    };
                }
            }
        }
        Err(e) => println!("Failed to read file, error: {}", e),
    }

    for username in usernames {
        println!("{}", username);
    }
}

fn main() {
    let mut cached_list: CachedUserList = CachedUserList::new();
    let no_token = "NO_TOKEN".to_string();
    // Get Bot Token from either env var or file
    let bot_token = match std::env::var("BOT_TOKEN") {
        Ok(val) => val,
        Err(_e) => std::fs::read_to_string("BOT_TOKEN.txt").unwrap_or(no_token.clone()),
    };
    if bot_token == no_token {
        println!("Failed to find bot token!");
        return;
    }

    // Parse command line arguments
    let cli_args: Vec<String> = std::env::args().collect();
    if cli_args.len() > 2 {
        match cli_args[1].as_str() {
            "--file" => parse_file(&mut cached_list, &bot_token, cli_args[2].clone()),
            "--lookup" => parse_cli_ids(&bot_token, cli_args.iter().skip(2).collect()),
            _ => println!("Unknown option {}", cli_args[1]),
        }
    } else {
        println!(
            "Requires more arguments (got {}, need more than 2)",
            cli_args.len()
        );
    }
    cached_list.save_to_file();
}
