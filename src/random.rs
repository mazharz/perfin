pub mod random {
    use rand::{distributions::Alphanumeric, thread_rng, Rng};

    use crate::config;

    pub fn generate_string(length: Option<usize>) -> String {
        let config_length =
            config::env("TRANSACTION_ID_LENGTH".to_string()).unwrap_or("10".to_string());
        let config_length = config_length
            .parse::<usize>()
            .expect("Couldn't parse config length into number.");

        let length = length.unwrap_or(config_length);
        let random: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();
        random
    }
}
