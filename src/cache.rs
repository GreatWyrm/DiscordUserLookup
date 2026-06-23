use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;

#[derive(Serialize, Deserialize, Debug)]
pub struct CachedUserList {
    users: HashMap<u64, String>,
}

static CACHE_FILE_PATH: &str = "user_cache.json";

impl CachedUserList {
    pub fn new() -> Self {
        let file_contents_result = std::fs::read_to_string(CACHE_FILE_PATH);
        match file_contents_result {
            Ok(str) => {
                let parse_result = serde_json::from_str(&str);
                match parse_result {
                    Ok(user_list) => Self { users: user_list },
                    Err(e) => {
                        println!("Failed to deseralize user list, {}", e);
                        Self {
                            users: HashMap::new(),
                        }
                    }
                }
            }
            Err(e) => {
                println!("Failed to load cache list, {}", e);
                Self {
                    users: HashMap::new(),
                }
            }
        }
    }

    pub fn save_to_file(self) -> () {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(CACHE_FILE_PATH);
        match file {
            Ok(fd) => {
                let _ = serde_json::to_writer_pretty(fd, &self.users);
            }
            Err(e) => println!("Failed to save cache list, {}", e),
        };
    }

    pub fn lookup_user(&self, id: u64) -> Option<String> {
        self.users.get(&id).cloned()
    }

    pub fn save_to_cache(&mut self, id: u64, username: &str) {
        self.users.insert(id, username.to_string());
    }
}
