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

    pub fn get_user(&self) -> &str {
        &self.user
    }

    pub fn get_pass(&self) -> &str {
        &self.password
    }
}

//TODO: This is for debuging purposes for now.
impl From<[&str; 2]> for LoginInput {

    fn from(value: [&str; 2]) -> Self {
        
        LoginInput {
            user: value[0].to_string(),
            password: value[1].to_string(),
        }
    }
}