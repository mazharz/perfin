#[derive(Debug)]
pub struct Account {
    name: String,
    subaccount: Option<Box<Account>>,
}

impl Account {
    pub fn add(name: String) -> Result<Box<Account>, &'static str> {
        if name.len() <= 0 {
            return Err("Account name can't be empty");
        }

        let mut accounts = name.split(":").into_iter();
        let root = accounts
            .next()
            .ok_or("Account name can't be empty!")
            .expect("Account name can't be empty!")
            .to_string();
        let mut root = Some(Box::new(Account {
            name: root,
            subaccount: None,
        }));

        let mut anchor = &mut root;
        for acc in accounts {
            let sub = Account {
                name: acc.to_string(),
                subaccount: None,
            };
            match anchor {
                Some(acc) => {
                    acc.subaccount = Some(Box::new(sub));
                    anchor = &mut acc.subaccount;
                }
                None => break,
            }
        }

        match root {
            Some(r) => return Ok(r),
            None => return Err("Couldn't create account."),
        }
    }

    pub fn format_string(self: &Box<Self>) -> String {
        let mut result = String::from("");

        let mut anchor = self;
        loop {
            result.push_str(&anchor.name);
            match &anchor.subaccount {
                Some(next) => {
                    result.push_str(":");
                    anchor = &next;
                }
                None => break,
            }
        }

        result
    }
}
