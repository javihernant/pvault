use crate::{auth::auth_server::AuthServer, account_input::LoginInput};

use super::CommandError;

impl AuthServer {
    pub fn login(&mut self) -> Result<(), CommandError> {
        let creds = LoginInput::new();
        Ok(self.authenticate(&creds)?)  
    }
}