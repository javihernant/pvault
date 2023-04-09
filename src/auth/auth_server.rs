use std::{path::PathBuf};
use crate::{account_input::LoginInput,};
use sqlite::{self,Connection};

use super::{AuthConfig, account::{self, Account, LoginError}};

pub mod commands;
pub struct AuthServer {
    db_conn: Connection,
    config: AuthConfig,
    account: Option<Account>,
}

#[derive(Debug)]
pub enum ServerError {
    DbError(sqlite::Error),
    ConfigError,
}


impl AuthServer {
    pub fn try_new() -> Result<AuthServer, ServerError> {

        //TODO: Read config from a toml
        let config = AuthConfig {
            db_path: PathBuf::from("./accounts.db"),
            max_retries: 3,
        };
        let conn = match sqlite::open(&config.db_path) {
            Ok(c) => c,
            Err(e) => return Err(ServerError::DbError(e)),
        };

        Ok(AuthServer { db_conn: conn, config, account:None})
    }

    pub fn run_session(&mut self) {
        //Run common commands
        //Run logged commands
        //Run not-logged commands (Signup, Login)
        self.login().unwrap();
        // todo!()
    }

    fn authenticate(&mut self, creds: &LoginInput) -> Result<(), LoginError> {
        let mut acc = Account::fetch(creds.user(), &self.db_conn)?;

        if let Err(e) = acc.is_pass_valid(creds.pass()) {
            match e {
                account::LoginError::WrongPassword => {
                    acc.log_fail_attempt();
                    acc.write_update(&self.db_conn).unwrap();
                    Err(e)
                },
                _ => return Err(e),
            }
        } else {
            self.account = Some(acc);
            Ok(())
        }
    }

    // fn execute(&mut self, command:&AccountCommand) -> Result<(), commands::CommandError>{
    //     if self.account.is_some() {
    //         match command {
    //             AccountCommand::Unban(user) => Ok(self.unban(user)?),
                
    //         }
    //     }
    //     else {
    //         Err(commands::CommandError::NoUserLogged)
    //     }
        
    // }
}



