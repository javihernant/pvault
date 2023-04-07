use pvault::auth::auth_server::{self, AuthServer};
use pvault::account_input::LoginInput;

fn main() {
    let mut auth_serv = AuthServer::try_new().unwrap();
    loop {
        let creds = LoginInput::new();
        match auth_serv.authenticate(&creds) {
            Ok(_) => break,
            Err(auth_server::Error::LoginError) => {
                println!("Ups, wrong credentials!");
                continue;
            }
            Err(auth_server::Error::LockedAccount) => {
                println!("Shit, I've locked the account!");
                break;
            }
            _ => {}
            
        }
    };
    
}

mod tests {
    use pvault::{account_input::LoginInput, auth::auth_server::AuthServer};

    #[test]
    fn test_not_possible_to_write_to_db_without_being_admin() {
        // let mut auth_serv = AuthServer::try_new().unwrap();
        // let creds = LoginInput::try_from(["javi", "pass"]).unwrap();
        // let mut acc = auth_serv.authenticate(&creds).unwrap();
        // acc.log_fail_attempt();
        // acc.write_update(&auth_serv.db_conn);
    }

    #[test]
    fn test_unban_user_with_an_admin_account() {
        use pvault::auth::auth_server::commands::AccountCommand;
        let mut auth_serv = AuthServer::try_new().unwrap();
        let creds = LoginInput::try_from(["admin", "pass"]).unwrap();
        auth_serv.authenticate(&creds).unwrap();
        let command = AccountCommand::Unban(String::from("javi"));
        auth_serv.execute(&command).unwrap();
    }
}
