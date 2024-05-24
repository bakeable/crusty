
use super::User;

pub struct UserBuilder {
    pub id: i32,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub active: Option<bool>,
}

impl UserBuilder {
    pub fn new() -> Self {
        UserBuilder {
            id: 0,
            email: None,
            first_name: None,
            last_name: None,
            active: None,
        }
    }

    pub fn id(&mut self, id: i32) -> &mut UserBuilder {
        self.id = id;
        self
    }
    
    pub fn email(&mut self, email: &str) -> &mut UserBuilder  {
        self.email = Some(String::from(email));
        self
    }
    
    pub fn first_name(&mut self, first_name: &str) -> &mut UserBuilder  {
        self.first_name = Some(String::from(first_name));
        self
    }

    pub fn last_name(&mut self, last_name: &str) -> &mut UserBuilder  {
        self.last_name = Some(String::from(last_name));
        self
    }

    pub fn active(&mut self, active: bool) -> &mut UserBuilder  {
        self.active = Some(active);
        self
    }

    pub fn build(&self) -> User {
        User {
            id: 0,
            email: self.email.clone().unwrap_or_default(),
            first_name: self.first_name.clone().unwrap_or_default(),
            last_name: self.last_name.clone().unwrap_or_default(),
            active: self.active.clone().unwrap_or_default(),
        }
    }
}