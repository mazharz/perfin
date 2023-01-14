use dotenv::dotenv;
use std::env;

pub fn var(key: String) -> Option<String> {
    dotenv().ok();

    env::var(key).ok()
}
