use crate::env;

pub struct Config {
    pub transaction_id_length: String,
    pub journal_file_path: String,
}

impl Config {
    pub fn read() -> Config {
        let mut fallback_file_path = env::var(String::from("HOME")).unwrap_or(String::from(""));
        fallback_file_path.push_str("/main.journal");

        Config {
            transaction_id_length: env::var("TRANSACTION_ID_LENGTH".to_string())
                .unwrap_or(String::from("10")),
            journal_file_path: env::var("JOURNAL_FILE_PATH".to_string())
                .unwrap_or(fallback_file_path),
        }
    }
}
