use crate::credentials::{self, Credentials};

pub struct Account {
    creds: Credentials,
    data: String,
}

#[derive(Debug)]
pub enum AccountError {
    TooManyAuths
}

impl Account {
    pub fn access() -> Result<Account, AccountError> {
        let mut i = 0;
        let creds = loop {
            match Credentials::try_new() {
                Ok(creds) => break creds,
                Err(e) => {
                    eprintln!("Wrong credentials:{e}");
                    if i==2 {
                        return Err(AccountError::TooManyAuths);
                    }   
                }
            }
            i += 1;
        };
        
        Ok(
            Account {
                creds,
                data: "This is my deepest darkest secret".to_string()
            }
        )
    }

    pub fn data(&self) -> &str {
        &self.data
    }
}