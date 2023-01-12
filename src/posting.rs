use crate::account::Account;

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

    pub fn format_string(&self) -> String {
        let mut result = String::from("");

        let account = &self.account.format_string();
        result.push_str(account);

        result.push_str("    ");

        result.push_str(&self.currency);

        result.push_str(&self.amount.to_string());

        result
    }
}
