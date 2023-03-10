use chrono::{DateTime, Local, NaiveDate};

use crate::posting::Posting;
use crate::random::random;

#[derive(Debug)]
pub struct Transaction {
    id: String,
    description: String,
    date: NaiveDate,
    postings: Vec<Posting>,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl Transaction {
    pub fn add(
        description: String,
        date: String,
        postings: Vec<String>,
    ) -> Result<Transaction, &'static str> {
        let id = random::generate_string(None);

        if date.len() <= 0 {
            return Err("date can't be empty.");
        }
        let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d").expect("Date is invalid.");

        if postings.len() % 2 != 0 {
            return Err("Must be key value pairs.");
        }

        let mut posting_objects: Vec<Posting> = vec![];
        for i in 0..postings.len() / 2 {
            let name = &postings[i * 2];
            let amount = &postings[(i + 1) * 2 - 1];
            let posting = Posting::add(name.to_string(), amount.to_string())
                .expect("Couldn't create posting object.");
            posting_objects.push(posting);
        }

        let created_at = Local::now();
        let updated_at = Local::now();

        Ok(Transaction {
            id,
            description,
            date,
            postings: posting_objects,
            created_at,
            updated_at,
        })
    }

    pub fn format_string(&self) -> String {
        let mut result = "".to_string();

        let date = self.date.format("%Y-%m-%d").to_string();
        result.push_str(&date);

        result.push_str(" ");

        let description = self.description.to_string();
        result.push_str(&description);

        result.push_str("\n    ");

        for posting in &self.postings {
            result.push_str(&posting.format_string());
            result.push_str("\n    ");
        }

        result
    }
}
