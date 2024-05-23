
use super::User;
use uuid::Uuid;

pub struct UserBuilder {
    uid: String,
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    active: Option<bool>,
}

impl UserBuilder {
    pub fn new() -> Self {
        // Create uid
        let uid = Uuid::default().to_string();

        // Return UserBuilder
        UserBuilder {
            uid: uid,
            email: None,
            first_name: None,
            last_name: None,
            active: None,
        }
    }

    pub fn uid(&mut self, uid: String) -> Self {
        self.uid = uid;
        self
    }
    
    pub fn email(&mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    pub fn first_name(&mut self, first_name: String) -> Self {
        self.first_name = Some(first_name);
        self
    }

    pub fn last_name(&mut self, last_name: String) -> Self {
        self.last_name = Some(last_name);
        self
    }

    pub fn active(&mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    pub fn build(self) -> User {
        User {
            uid: self.uid.clone(),
            email: self.email.clone().unwrap_or_default(),
            first_name: self.first_name.clone().unwrap_or_default(),
            last_name: self.last_name.clone().unwrap_or_default(),
            active: self.active.clone().unwrap_or_default(),
        }
    }
}