use dotenv::dotenv;
use std::env;

pub fn env(key: String) -> Option<String> {
    dotenv().ok();

    env::var(key).ok()
}
