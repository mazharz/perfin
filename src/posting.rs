// TODO: make name into account which handles subaccounts
#[derive(Debug)]
pub struct Posting {
    name: String,
    currency: String,
    amount: i64,
}

impl Posting {
    pub fn add(name: String, amount: String) -> Result<Posting, &'static str> {
        let mut amount = amount.chars();
        let currency = amount.next().expect("Couldn't extract prefix from amount.");
        let amount = amount
            .as_str()
            .parse::<i64>()
            .expect("Couldn't convert amount to number.");

        Ok(Posting {
            name,
            currency: currency.to_string(),
            amount,
        })
    }
}
