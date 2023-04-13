use std::fmt::Display;
use chrono::naive::{NaiveDate, NaiveDateTime};

use sqlite::{State, Connection};

#[derive(Debug, Copy, Clone)]
enum Status {
    Online,
    Offline,
    Idle,
    Invisible
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Offline => write!(f, "offline"),
            Status::Online => write!(f, "online"),
            Status::Idle => write!(f, "idle"),
            Status::Invisible => write!(f, "invisible"),

        }
    }
}

impl Status {
    fn code(&self) -> i32 {
        match self {
            Status::Offline => 0,
            Status::Online => 1,
            Status::Idle => 2,
            Status::Invisible => 3,
        }
    }
}
#[derive(Debug)]
struct InvalidStatus;

impl TryFrom<i32> for Status {
    type Error = InvalidStatus;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Status::Offline),
            1 => Ok(Status::Online),
            2 => Ok(Status::Idle),
            3 => Ok(Status::Invisible),
            _ => Err(InvalidStatus)
        }
    }
}

#[derive(Debug)]
pub struct Account {
    id: usize,
    username: String,
    password: String,
    motto: String,
    banned: bool,
    retries_left: usize,
    rank: i32,
    email: String,
    last_on: NaiveDateTime,
    status: Status,
    signup_date: NaiveDateTime,
}

#[derive(Debug)]
pub enum LoginError {
    UserNotFound,
    WrongPassword,
    BannedAccount,
}

impl Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UserNotFound => write!(f, "invalid User"),
            Self::WrongPassword => write!(f, "wrong Password"),
            Self::BannedAccount => write!(f, "this account is banned"),
        }
    }
}

impl Account {
    pub fn fetch(user: &str, db_conn: &Connection) -> Result<Account, LoginError> {
        let query = "SELECT * FROM users WHERE username = ?";
        let mut statement = db_conn.prepare(query).unwrap();
        statement.bind((1, user)).unwrap();

        if let Ok(State::Row) = statement.next() {
            let id = statement.read::<i64, _>("id").unwrap() as usize;
            let username = statement.read::<String, _>("username").unwrap();
            let password = statement.read::<String, _>("password").unwrap();
            let motto = statement.read::<String, _>("motto").unwrap();
            let banned = statement.read::<i64, _>("banned").unwrap() != 0;
            let retries_left = statement.read::<i64, _>("retries_left").unwrap() as usize;
            let rank = statement.read::<i64, _>("rank").unwrap() as i32;
            let email = statement.read::<String, _>("email").unwrap();
            let last_on = statement.read::<String, _>("last_on").unwrap();
            let signup_date = statement.read::<String, _>("signup_date").unwrap();
            let status = statement.read::<i64, _>("status").unwrap() as i32;

            let parse_from_str = NaiveDateTime::parse_from_str;
            let acc = Account {
                id,
                username,
                password,
                motto,
                banned,
                retries_left,
                rank,
                email,
                last_on: parse_from_str(last_on.as_str(), "%Y-%m-%d %H:%M:%S").unwrap(),
                status: Status::try_from(status).unwrap(),
                signup_date: parse_from_str(signup_date.as_str(), "%Y-%m-%d %H:%M:%S").unwrap(),
            };
            Ok(acc)
        } else {
            Err(LoginError::UserNotFound)
        }
    }

    pub fn is_pass_valid(&self, pass: &str) -> Result<(), LoginError> {
        if pass == self.password && !self.banned {
            Ok(())
        }
        else
        {
            if !self.banned {
                Err(LoginError::WrongPassword)
            } else {
                Err(LoginError::BannedAccount)
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

    pub fn write_update(&self, db_conn: &Connection) -> Result<(), sqlite::Error>{
        let query = format!("UPDATE users
SET username = '{}',
    password = '{}',
    motto = '{}',
    banned = {},
    retries_left = {},
    rank = {},
    email = '{}',
    last_on = '{}',
    status = {},
    signup_date = '{}'
WHERE
    id = {}", self.username, self.password, self.motto, if self.banned {1} else {0}, self.retries_left, self.rank, self.email, self.last_on, self.status.code(), self.signup_date, self.id);

        Ok(db_conn.execute(query)?)
    }

    pub fn reset_retries(&mut self, value: usize){
        self.retries_left = value;
    }

    pub fn retries_left(&self) -> usize{
        self.retries_left
    }

    pub fn unban(&mut self) {
        self.banned = false;
    }

    pub fn is_admin(&self) -> bool {
        self.rank > 0
    }

    pub fn show_stats(&self, caller_is_admin: bool) {
        println!("Username: {}", self.username);
        println!("Motto: {}", self.motto);
        println!("Last on: {}", self.last_on);
        let status = match self.status {
            Status::Invisible if !caller_is_admin => Status::Offline,
            status => status,
        };
        println!("Status: {}", status);

        if caller_is_admin {
            println!("===Extra info:===");
            println!("Banned: {}", self.banned);
            println!("Rank: {}", self.rank);
            println!("Email: {}", self.email);
            println!("Signed up on: {}", self.signup_date)

        }
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
        acc.status = 2;
        acc.write_update(&conn).unwrap();
    }

    #[test]
    fn test_show_stats() {
        let conn = create_db_conn();
        let acc = Account::fetch("javi", &conn).unwrap();
        acc.show_stats();
    }
}