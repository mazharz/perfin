use chrono::{DateTime, Local, NaiveDate};
use uuid::Uuid;

#[derive(Debug)]
pub struct Transaction {
    id: String,
    description: String,
    date: NaiveDate,
    postings: Vec<String>,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl Transaction {
    pub fn add(
        description: String,
        date: String,
        postings: Vec<String>,
    ) -> Result<Transaction, &'static str> {
        let id = Uuid::new_v4().to_string();

        if date.len() <= 0 {
            return Err("date can't be empty.");
        }
        let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d").expect("Date is invalid.");

        if postings.len() % 2 != 0 {
            return Err("Must be key value pairs.");
        }
        // TODO: create Vec<Posting> here

        // let mut debit = 0;
        // let mut credit = 0;
        // let mut amount = amount.chars();
        // let prefix = amount.next().expect("Couldn't extract prefix from amount.");

        // TODO: figure out how to ergonomically send -394 without having to prepend a -- to it
        // TODO: refactor
        // if prefix == '+' || prefix == '-' {
        //     let whole = amount
        //         .as_str()
        //         .parse::<u128>()
        //         .expect("Couldn't convert amount to number.");
        //     if prefix == '+' {
        //         debit = whole;
        //     } else {
        //         credit = whole;
        //     }
        // } else {
        //     let mut whole = String::from(prefix);
        //     whole.push_str(amount.as_str());
        //     let whole = whole
        //         .parse::<u128>()
        //         .expect("Couldn't convert amount to number.");
        //     debit = whole;
        // }

        let created_at = Local::now();
        let updated_at = Local::now();

        Ok(Transaction {
            id,
            description,
            date,
            postings,
            created_at,
            updated_at,
        })
    }
}
