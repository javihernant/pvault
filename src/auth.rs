use std::fmt::Display;

use crate::credentials::Credentials;

pub enum AuthError {
    InvalidUser,
    InvalidPassword,
}

impl Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidUser => write!(f, "Invalid User"),
            Self::InvalidPassword => write!(f, "Invalid Password"),
        }
    }
}

pub fn authenticate(creds:Credentials) -> Result<Credentials, AuthError> {
    if creds.user() == "Javi" {
        if creds.pass() != "mypass" {
            return Err(AuthError::InvalidPassword);
        }
        Ok(creds)
    }else {
        Err(AuthError::InvalidUser)
    }
}