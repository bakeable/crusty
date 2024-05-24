use serde::{Serialize, Deserialize};
use diesel::prelude::*;
    
#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Identifiable, Insertable, Queryable, Selectable, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub active: bool,
}