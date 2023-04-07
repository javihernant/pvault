use std::{path::PathBuf, io};

pub mod auth_server;
pub mod account;

struct AuthConfig {
    db_path: PathBuf,
    retries_limit: usize,
    // pass_reqs: PassConfig
}

struct PassConfig {
    min_len: usize,
    min_special_chars: usize,
    min_nums: usize,
    min_lower_case: usize,
    min_upper_case: usize,
}

impl AuthConfig {
    fn read() -> Result<AuthConfig, io::Error> {
        todo!()
    }
}

