use diesel::prelude::*;
use dotenv::dotenv;
use std::env;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    
    // Load the DATABASE_URL environment variable
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    log::debug!("Connecting to {}", database_url);
    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    log::debug!("Connected to {}", database_url);

    connection
}