pub mod random {
    use rand::{distributions::Alphanumeric, thread_rng, Rng};

    use crate::config::Config;

    pub fn generate_string(length: Option<usize>) -> String {
        let config_length = Config::read().transaction_id_length;
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
