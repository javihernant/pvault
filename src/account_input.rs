pub mod login_input;
//mod signup;

use std::io::{self, Write};

pub use login_input::LoginInput;

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

