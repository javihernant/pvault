use std::{error::Error, str::FromStr};

use crate::auth::account::LoginError;


#[derive(Debug)]
pub enum CommandError {
    UnknownCommand,
    NotEnoughPrivilege,
    ExecutionError(&'static str),
    LoginError(LoginError)
}

impl From<LoginError> for CommandError {
    fn from(error: LoginError) -> Self {
        CommandError::LoginError(error)
    }
}

#[derive(Debug)]
pub enum Command<'a> {
    Login,
    Register,
    Unban (&'a str)
}

impl FromStr for Command<'_> {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let args:Vec<&str> = s.split(" ").collect();
        match args[0] {
            "/login" => Ok(Self::Login),
            _ => Err(CommandError::UnknownCommand),
        }
    }
}

mod logged_in;
mod anonymous;
mod common;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_command() {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let comm = Command::from_str(input.as_str()).unwrap();
        println!("{:?}", comm);
    }
}
