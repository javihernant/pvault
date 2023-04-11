use crate::auth::{auth_server::AuthServer, account::Account};

use super::CommandError;

pub enum AccountCommand {
    Unban (String),
}
impl AuthServer {
    pub fn unban(&self, user:&str) -> Result<(), CommandError> {

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
}