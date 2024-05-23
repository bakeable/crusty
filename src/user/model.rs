use serde::{Serialize, Deserialize};
use diesel::prelude::*;
    
#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Insertable, Queryable, Selectable, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    uid: String,
    email: String,
    first_name: String,
    last_name: String,
    active: bool,
}