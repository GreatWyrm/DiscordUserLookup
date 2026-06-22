use reqwest::blocking::Client;
use reqwest::header::HeaderValue;
use serde::Deserialize;


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
            Some(value) => {
                match value {
                    1 => Some(PremiumType::NitroClassic),
                    2 => Some(PremiumType::Nitro),
                    3 => Some(PremiumType::NitroBasic),
                    _ => Some(PremiumType::None),
                }
            },
            None => None,
        }
    }
}

fn lookup_user(bot_token: &str, user_id: u64) -> () {
    // Create sensitive header value
    let mut auth_value = HeaderValue::from_str(format!("Bot {}", bot_token).as_str()).unwrap();
    auth_value.set_sensitive(true);

    let client = Client::new();
    let request = client.get(format!("https://discord.com/api/v9/users/{}", user_id))
        .header(reqwest::header::AUTHORIZATION, auth_value);

    let response = request.send().unwrap();


    match response.status() {
        reqwest::StatusCode::OK => {
            println!("Success!");
        },
        _ => println!("Unhandled Status Code {}", response.status()),
    };
    let retrieved_user = response.json::<DiscordUser>().unwrap();
    println!("ID: {}, Username: {}, Global Username: {:?}", retrieved_user.id, retrieved_user.username, retrieved_user.global_name);
    //println!("{}",response.text().unwrap());
    //println!("{:?}", response.json::<DiscordUser>());
} 

fn main() {
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
    for arg in cli_args.into_iter().skip(1) {
        // Try parse to u64
        let result = arg.parse::<u64>();
        match result {
            Ok(value) => {
                lookup_user(&bot_token, value);
            },
            Err(e) => {
                println!("Failed to parse {} as a u64. {}", arg, e);
            },
        }
        // Sleep to avoid API limits
        std::thread::sleep(std::time::Duration::from_millis(3));
    }
}
