use pvault::account::{LoginError, Account};
use pvault::account_input::LoginInput;
fn main() {
    let mut account = Account::new();

    let account = loop {
        let creds = LoginInput::new();

        if let Err(e) = account.connect(&creds) {
            match e {
                LoginError::LockedAccount => break None,
                LoginError::WrongCredentials => {
                    eprintln!("Login error");
                    continue;
                }
            }
        }
        break Some(account);
    };
    
    if let Some(acc) = account {
        println!("{}",acc.data());
    }
    else {
        println!("Couldnt connect!");
    }
    
}
