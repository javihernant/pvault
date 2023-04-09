use std::error::Error;

use crate::auth::account::LoginError;


#[derive(Debug)]
pub enum CommandError {
    CommandNotFound,
    NotEnoughPrivilege,
    ExecutionError(&'static str),
    LoginError(LoginError)
}

impl From<LoginError> for CommandError {
    fn from(error: LoginError) -> Self {
        CommandError::LoginError(error)
    }
}

mod logged_in;
mod anonymous;
mod common;
