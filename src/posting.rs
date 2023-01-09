pub struct Posting {
    name: String,
    currency: String,
    amount: u128,
}

impl Posting {
    pub fn add(name: String, amount: String) -> Result<Posting, &'static str> {
        return Err("shit");
        // TODO: create Posting and use in Transaction
    }
}
