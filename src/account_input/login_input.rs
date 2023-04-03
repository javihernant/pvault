use std::io::{self, Write};

#[derive(Debug)]
pub struct LoginInput {
    user: String,
    password: String,
}

impl LoginInput {
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

    pub fn new() -> LoginInput {
        let user = Self::read_user();
        let password = Self::read_pass();

        LoginInput { 
            user,
            password
        }
    }

    pub fn user(&self) -> &str {
        &self.user
    }

    pub fn pass(&self) -> &str {
        &self.password
    }
}