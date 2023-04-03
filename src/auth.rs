use std::fmt::Display;
use crate::{account_input::LoginInput,};
use std::collections::HashMap;

///Possible errors that the authentication server can return.
/// For security reasons, this info should only be available to the developer. 
/// You can return a less-informative error to your user.
pub enum AuthError {
    InvalidUser,
    InvalidPassword (usize),
    TooManyRetries (usize),
}

pub struct AuthServer {
    creds: HashMap<String, String>,
    retries_left: HashMap<String, usize>,
    config: AuthConfig,
}
struct AuthConfig {
    retries_limit: usize,
}

// impl AuthConfig {
//     fn new(retries_limit: usize) -> AuthConfig {
//         AuthConfig { retries_limit }
//     }
// }

impl AuthServer {
    pub fn new() -> AuthServer {
        let mut creds = HashMap::new();
        creds.insert(String::from("javi"), String::from("pass"));
        creds.insert(String::from("pepe"), String::from("pass"));
        creds.insert(String::from("fran"), String::from("pass"));

        let config = AuthConfig {
            retries_limit: 3,
        };

        let retries_left = creds
            .keys()
            .cloned()
            .map(|k| (k, config.retries_limit))
            .collect::<HashMap<String, usize>>();

        AuthServer { creds, retries_left, config }
    }

    pub fn authenticate(&mut self, creds: &LoginInput) -> Result<(), AuthError> {
        let (in_user, in_pass)  = {
            (creds.user(), creds.pass())
        };
        if let Some(pass) = self.creds.get(in_user) {
            if in_pass == pass {
                Ok(())
            } else {
                let retries_left = self.retries_left.get_mut(in_user).expect("Should contain the user as a key");
                if *retries_left > 0 {
                    *retries_left -= 1;
                    Err(AuthError::InvalidPassword(*retries_left))
                }
                else {
                    Err(AuthError::TooManyRetries(self.config.retries_limit))
                }
            }
        } else {
            Err(AuthError::InvalidUser)
        }
    }
}

impl Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidUser => write!(f, "Invalid User"),
            Self::InvalidPassword(left) => write!(f, "Wrong Password; {left} retries left"),
            Self::TooManyRetries(total) => write!(f, "Too many login retries; Only {total} allowed"),
        }
    }
}

