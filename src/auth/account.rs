use std::fmt::Display;

use sqlite::{State, Connection};

#[derive(Debug)]
pub struct Account {
    id: usize,
    username: String,
    password: String,
    data: String,
    banned: bool,
    retries_left: usize,
}

#[derive(Debug)]
pub enum Error {
    UserNotFound,
    WrongPassword,
    TooManyRetries,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UserNotFound => write!(f, "Invalid User"),
            Self::WrongPassword => write!(f, "Wrong Password"),
            Self::TooManyRetries => write!(f, "Too many login retries"),
        }
    }
}

impl Account {
    pub fn fetch(user: &str, db_conn: &Connection) -> Result<Account, Error> {
        let query = "SELECT * FROM users WHERE username = ?";
        let mut statement = db_conn.prepare(query).unwrap();
        statement.bind((1, user)).unwrap();

        if let Ok(State::Row) = statement.next() {
            let id = statement.read::<i64, _>("id").unwrap() as usize;
            let username = statement.read::<String, _>("username").unwrap();
            let password = statement.read::<String, _>("password").unwrap();
            let data = statement.read::<String, _>("data").unwrap();
            let banned = statement.read::<i64, _>("banned").unwrap() != 0;
            let retries_left = statement.read::<i64, _>("retries_left").unwrap() as usize;

            let acc = Account {
                id,
                username,
                password,
                data,
                banned,
                retries_left
            };
            Ok(acc)
        } else {
            Err(Error::UserNotFound)
        }
    }

    pub fn is_pass_valid(&self, pass: &str) -> Result<(), Error> {
        if pass == self.password {
            Ok(())
        }
        else
        {
            if self.retries_left > 0 {
                Err(Error::WrongPassword)
            } else {
                Err(Error::TooManyRetries)
            }
        }
    }

    pub fn log_fail_attempt(&mut self) {
        if self.retries_left > 0 {
            self.retries_left -= 1;
            if self.retries_left == 0 {
                self.banned = true;
            }
        }
    }

    pub fn write_update(&self, db_conn: &Connection) {
        let query = format!("UPDATE users
SET username = '{}',
    password = '{}',
    data = '{}',
    banned = {},
    retries_left = {}
WHERE
    id = {}", self.username, self.password, self.data, if self.banned {1} else {0}, self.retries_left, self.id);

        db_conn.execute(query).unwrap();

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlite;
    
    fn create_db_conn() -> Connection {
        sqlite::open("./accounts.db").unwrap()
    }

    #[test]
    fn test_is_pass_valid() {
        let conn = create_db_conn();
        let acc = Account::fetch("javi", &conn).unwrap();
        acc.is_pass_valid("pass").unwrap();
    }

    #[test]
    fn test_update_acc() {
        let conn = create_db_conn();
        let mut acc = Account::fetch("javi", &conn).unwrap();
        acc.log_fail_attempt();
        acc.write_update(&conn);
    }
}