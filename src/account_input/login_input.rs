
#[derive(Debug)]
pub struct LoginInput {
    user: String,
    password: String,
}

impl LoginInput {
    pub fn new() -> LoginInput {
        let user = super::read_user();
        let password = super::read_pass();

        LoginInput { 
            user,
            password
        }
    }

    pub fn user(&self) -> &str {
        &self.user
    }

    pub fn pass(&self) -> &str {
        &self.password
    }
}

//TODO: This is for debuging purposes for now.
impl From<[&str; 2]> for LoginInput {

    fn from(value: [&str; 2]) -> Self {
        
        LoginInput {
            user: value[0].to_string(),
            password: value[1].to_string(),
        }
    }
}