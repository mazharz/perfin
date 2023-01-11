use crate::account::Account;

// TODO: make name into account which handles subaccounts
#[derive(Debug)]
pub struct Posting {
    account: Box<Account>,
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

        let account = Account::add(name).expect("Couldn't create account struct.");

        Ok(Posting {
            account,
            currency: currency.to_string(),
            amount,
        })
    }
}
