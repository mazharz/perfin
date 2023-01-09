pub mod date {
    use chrono::Local;

    pub fn today_date() -> String {
        Local::now().format("%Y-%m-%d").to_string()
    }
}
