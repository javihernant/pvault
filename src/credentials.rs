use std::io::{self, Write};
use crate::auth::{self, AuthError};

#[derive(Debug)]
pub struct Credentials {
    user: String,
    password: String,
}

impl Credentials {
    fn read_user() -> String {
        print!("User: ");
        io::stdout().flush().unwrap();
        let mut user = String::new();
        io::stdin().read_line(&mut user).expect("Failed to read line");
        user.trim().to_string()
    }

    fn read_pass() -> String {
        print!("Password: ");
        io::stdout().flush().unwrap();
        let mut password = String::new();
        io::stdin().read_line(&mut password).expect("Failed to read line");
        password.trim().to_string()
    }

    pub fn try_new() -> Result<Credentials, AuthError> {
        let user = Self::read_user();
        let password = Self::read_pass();

        let creds = Credentials { 
            user,
            password
        };
        auth::authenticate(creds)
    }

    pub fn user(&self) -> &str {
        &self.user
    }

    pub fn pass(&self) -> &str {
        &self.password
    }
}