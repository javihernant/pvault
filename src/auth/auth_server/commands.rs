use std::{error::Error, str::FromStr};

use crate::auth::account::LoginError;


#[derive(Debug)]
pub enum CommandError {
    UnknownCommand,
    NotAuthorized,
    ExecutionError(&'static str),
    LoginError(LoginError),
    UserNotLogged
}

impl From<LoginError> for CommandError {
    fn from(error: LoginError) -> Self {
        CommandError::LoginError(error)
    }
}

#[derive(Debug)]
pub enum Command {
    Login,
    Register,
    Unban (String)
}

impl FromStr for Command {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let args:Vec<&str> = s.split_ascii_whitespace().collect();
        match args[0] {
            "/login" => Ok(Self::Login),
            "/unban" => {
                if args.len() < 2 {
                    Err(CommandError::ExecutionError("Not enough arguments"))
                } else {
                    Ok(Command::Unban(args[1].to_string()))
                }
            }
            _ => Err(CommandError::UnknownCommand),
        }
    }
}

impl Command {
    
    pub fn try_read() -> Result<Command, CommandError> {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        Ok(Command::from_str(input.as_str())?)
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

        println!("{:?}", comm);
    }
}
