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

        // array of strings
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
        let mut refe = &mut root;
        for acc in accounts {
            let sub = Account {
                name: acc.to_string(),
                subaccount: None,
            };
            match refe {
                Some(acc) => {
                    acc.subaccount = Some(Box::new(sub));
                    refe = &mut acc.subaccount;
                }
                None => break,
            }
        }

        match root {
            Some(r) => return Ok(r),
            None => return Err("Couldn't create account."),
        }
    }
}
