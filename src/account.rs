use crate::{account_input::LoginInput, auth::{AuthError, AuthServer}};

pub struct Account {
    user: String,
    online: bool,
    data: String,
    auth_server: AuthServer,
}

pub enum LoginError {
    LockedAccount,
    WrongCredentials
}

impl Account {
    pub fn new() -> Account {
        Account {
            user: "default".to_string(),
            online:false,
            data: "This is my deepest darkest secret".to_string(),
            auth_server: AuthServer::new(),
        }
    }

    pub fn connect(&mut self, creds: &LoginInput) -> Result<(), LoginError> {

        match self.auth_server.authenticate(creds) {
            Err(AuthError::TooManyRetries(_)) => Err(LoginError::LockedAccount),
            Err(_) => Err(LoginError::WrongCredentials),
            Ok(_) => {
                self.user = creds.user().to_owned();
                self.online = true;
                Ok(())
            }
        }

    }

    pub fn data(&self) -> &str {
        &self.data
    }
}