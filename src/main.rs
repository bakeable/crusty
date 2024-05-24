extern crate diesel;
extern crate dotenv;
use crate::user::UserBuilder;
use crate::user::User;

pub mod schema;
pub mod user;
pub mod database;

fn main() {
    // Establish a connection to the database
    let mut connection = database::establish_connection();

    // Example usage
    let first_user = UserBuilder::new()
        .first_name("Robin")
        .last_name("Bakker")
        .email("robin@bakeable.nl")
        .active(true)
        .build();

    match User::create(&mut connection, first_user) {
        Ok(user) => println!("User created: {:?}", user),
        Err(err) => println!("Error creating user: {}", err),
    }

    // Example usage
    let mut second_user = UserBuilder::new()
        .first_name("Robin")
        .last_name("Bakker")
        .email("robin@bakeable.nl")
        .active(true)
        .build();

    match User::create(&mut connection, second_user.clone()) {
        Ok(user) => println!("User created: {:?}", user),
        Err(err) => println!("Error creating user: {}", err),
    }


    match User::read(&mut connection, 1) {
        Ok(user) => println!("User read: {:?}", user),
        Err(err) => println!("Error reading user: {}", err),
    }

    // Update the user
    second_user.active = false;
    match User::update(&mut connection, 1, second_user) {
        Ok(user) => println!("User updated: {:?}", user),
        Err(err) => println!("Error updating user: {}", err),
    }

    // Read the user
    match User::read(&mut connection, 1) {
        Ok(user) => println!("User read: {:?}", user),
        Err(err) => println!("Error reading user: {}", err),
    }

    match User::delete(&mut connection, 1) {
        Ok(user) => println!("User deleted: {:?}", user),
        Err(err) => println!("Error deleting user: {}", err),
    }
}