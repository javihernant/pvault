use std::{path::PathBuf};
use crate::{account_input::LoginInput,};
use sqlite::{self,Connection};

use self::commands::AccountCommand;

use super::{AuthConfig, account::{self, Account}};

pub mod commands;
pub struct AuthServer {
    db_conn: Connection,
    config: AuthConfig,
    account: Option<Account>,
}

#[derive(Debug)]
pub enum Error {
    DbError(sqlite::Error),
    ConfigError,
    LoginError,
    LockedAccount,
}

impl AuthServer {
    pub fn try_new() -> Result<AuthServer, Error> {

        //TODO: Read config from a toml
        let config = AuthConfig {
            db_path: PathBuf::from("./accounts.db"),
            max_retries: 3,
        };
        let conn = match sqlite::open(&config.db_path) {
            Ok(c) => c,
            Err(e) => return Err(Error::DbError(e)),
        };

        Ok(AuthServer { db_conn: conn, config, account:None})
    }

    pub fn authenticate(&mut self, creds: &LoginInput) -> Result<(), Error> {
        let mut acc = match Account::fetch(creds.get_user(), &self.db_conn) {
            Ok(acc) => {
                acc
            },
            Err(_) => return Err(Error::LoginError),
        };

        if let Err(e) = acc.is_pass_valid(creds.get_pass()) {
            match e {
                account::LoginError::BannedAccount => {
                    return Err(Error::LockedAccount)
                },
                account::LoginError::WrongPassword => {
                    acc.log_fail_attempt();
                    acc.write_update(&self.db_conn).unwrap();
                    if acc.retries_left() > 0 {
                        return Err(Error::LoginError)
                    } else {
                        return Err(Error::LockedAccount)
                    }
                    
                },
                account::LoginError::UserNotFound => {
                    unreachable!();
                }
            }
        } else {
            self.account = Some(acc);
            Ok(())
        }
    }

    pub fn execute(&mut self, command:&AccountCommand) -> Result<(), commands::CommandError>{
        if self.account.is_some() {
            match command {
                AccountCommand::Unban(user) => Ok(self.unban(user)?),
    
            }
        }
        else {
            Err(commands::CommandError::NoUserLogged)
        }
        
    }
}



