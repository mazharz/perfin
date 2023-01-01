use chrono::{DateTime, Local, NaiveDate};
use uuid::Uuid;

#[derive(Debug)]
pub struct Entry {
    id: String,
    description: String,
    date: NaiveDate,
    debit: u128,
    credit: u128,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    // TODO:
    // category: Category,
    // account: Account,
    // currency: Currency
}

impl Entry {
    pub fn add(description: String, date: String, amount: String) -> Result<Entry, &'static str> {
        let id = Uuid::new_v4().to_string();

        if description.len() <= 0 {
            return Err("description can't be empty.");
        }

        if date.len() <= 0 {
            return Err("date can't be empty.");
        }
        let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d").expect("Date is invalid.");

        if amount.len() <= 0 {
            return Err("amount can't be empty.");
        }

        let mut debit = 0;
        let mut credit = 0;
        let mut amount = amount.chars();
        let prefix = amount.next().expect("Couldn't extract prefix from amount.");

        // TODO: figure out how to ergonomically send -394 without having to prepend a -- to it
        // TODO: refactor
        if prefix == '+' || prefix == '-' {
            let whole = amount
                .as_str()
                .parse::<u128>()
                .expect("Couldn't convert amount to number.");
            if prefix == '+' {
                debit = whole;
            } else {
                credit = whole;
            }
        } else {
            let mut whole = String::from(prefix);
            whole.push_str(amount.as_str());
            let whole = whole
                .parse::<u128>()
                .expect("Couldn't convert amount to number.");
            debit = whole;
        }

        let created_at = Local::now();
        let updated_at = Local::now();

        Ok(Entry {
            id,
            description,
            date,
            debit,
            credit,
            created_at,
            updated_at,
        })
    }
}
