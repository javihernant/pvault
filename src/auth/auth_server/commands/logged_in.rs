use crate::auth::{auth_server::AuthServer, account::Account};

use super::CommandError;

impl AuthServer {
    pub fn unban(&self, user:&str) -> Result<(), CommandError> {
        if !self.is_user_logged() {
            return Err(CommandError::UserNotLogged);
        }
        if self.account.as_ref().unwrap().is_admin() {
            let max_retries = self.config.max_retries;
            let mut acc: Account = match Account::fetch(user, &self.db_conn) {
                Ok(acc) => {
                    acc
                },
                Err(_) => return Err(CommandError::ExecutionError("Could not find that user!")),
            };
            acc.reset_retries(max_retries);
            acc.unban();
            if let Err(_) = acc.write_update(&self.db_conn) {
                Err(CommandError::ExecutionError("Error updating db"))
            } else {
                Ok(())
            }
        }
        else {
            Err(CommandError::NotAuthorized)
        }
        
    }

    pub fn show_stats(&self, user: &Option<String>) -> Result<(), CommandError> {
        if !self.is_user_logged() {
            return Err(CommandError::UserNotLogged);
        }
        let caller_is_admin = self.account.as_ref().unwrap().is_admin();
        if let Some(user) = user.as_deref() {
            let acc = Account::fetch(user, &self.db_conn)?;
            acc.show_stats(caller_is_admin);
        } else {
            self.account.as_ref().unwrap().show_stats(true);
        }
        Ok(())
    }
}