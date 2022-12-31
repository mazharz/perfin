use chrono::{DateTime, Local, NaiveDate};

#[derive(Debug)]
pub struct Entry {
    id: usize,
    description: String,
    date: NaiveDate,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    // TODO:
    // category: Category,
    // account: Account,
    // debit: usize,
    // credit: usize,
    // currency: Currency
}

impl Entry {
    pub fn add(description: String, date: String) -> Result<Entry, &'static str> {
        if description.len() <= 0 {
            return Err("description can't be empty.");
        }

        if date.len() <= 0 {
            return Err("date can't be empty.");
        }
        let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d").expect("Date is invalid.");

        let created_at = Local::now();
        let updated_at = Local::now();

        // TODO: must generate random id here
        Ok(Entry {
            id: 1,
            description,
            date,
            created_at,
            updated_at,
        })
    }
}
