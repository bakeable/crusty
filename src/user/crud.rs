use diesel::pg::PgConnection;
use crate::user::model::User;
use crate::schema::users;

use diesel::prelude::*;

impl User {
    pub fn create(conn: &mut PgConnection, user: User) -> QueryResult<User> {
        diesel::insert_into(users::table).values(user).get_result(conn)
    }

    pub fn read(conn: &mut PgConnection, user_id: i32) -> QueryResult<User> {
        users::table.find(user_id).first(conn)
    }

    pub fn update(conn: &mut PgConnection, user_id: i32, user: User) -> QueryResult<User> {
        diesel::update(users::table.find(user_id)).set(user).get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, user_id: i32) -> QueryResult<usize> {
        diesel::delete(users::table.find(user_id)).execute(conn)
    }
}