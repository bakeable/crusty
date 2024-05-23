use diesel::table;

table! {
    users (uid) {
        uid -> Varchar,
        email -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        active -> Bool,
    }
}