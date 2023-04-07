use std::{path::PathBuf};
use crate::{account_input::LoginInput,};
use sqlite::{self,Connection};

use super::{AuthConfig, account::{self, Account}};


pub struct AuthServer {
    db_conn: Connection,
    config: AuthConfig,
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

        //TODO: Read config from a json
        let config = AuthConfig {
            db_path: PathBuf::from("./accounts.db"),
            retries_limit: 3,
        };
        let conn = match sqlite::open(&config.db_path) {
            Ok(c) => c,
            Err(e) => return Err(Error::DbError(e)),
        };

        Ok(AuthServer { db_conn: conn, config })
    }

    pub fn authenticate(&self, creds: &LoginInput) -> Result<Account, Error> {
        let mut acc = match Account::fetch(creds.get_user(), &self.db_conn) {
            Ok(acc) => {
                acc
            },
            Err(_) => return Err(Error::LoginError),
        };

        if let Err(e) = acc.is_pass_valid(creds.get_pass()) {
            match e {
                account::Error::TooManyRetries => {
                    return Err(Error::LockedAccount)
                },
                account::Error::WrongPassword => {
                    acc.log_fail_attempt();
                    acc.write_update(&self.db_conn);
                    return Err(Error::LoginError)
                },
                account::Error::UserNotFound => {
                    return Err(Error::LoginError)
                }
            }
        } else {
            Ok(acc)
        }
    }
}



