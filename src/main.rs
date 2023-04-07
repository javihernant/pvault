use pvault::auth;
use pvault::auth::auth_server::{self, AuthServer};
use pvault::account_input::LoginInput;

fn main() {
    let auth_serv = AuthServer::try_new().unwrap();
    let acc = loop {
        let creds = LoginInput::new();
        match auth_serv.authenticate(&creds) {
            Ok(acc) => break Some(acc),
            Err(auth_server::Error::LoginError) => {
                println!("Ups, wrong credentials!");
                continue;
            }
            Err(auth_server::Error::LockedAccount) => {
                println!("Shit, I've locked the account!");
                break None;
            }
            _ => {}
            
        }
    };

    println!("{:?}", acc);
    
}
