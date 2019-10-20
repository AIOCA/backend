use crate::database::models::{Admin,User};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use rocket_contrib::json::{Json, JsonValue};
use std::env;

#[database("aioc")]
pub struct DB(MysqlConnection);

pub fn db_connect() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn check_admin(conn: &DB, user_fetched: &User) -> bool {
    Admin::belonging_to(user_fetched)
        .first::<Admin>(conn as &MysqlConnection)
        .is_ok()
}

pub fn fetch_user(conn: &DB, username: &str) -> Result<User, Json<JsonValue>> {
    use crate::database::schema::users::dsl::*;
    match users
        .filter(user_name.eq(&username.to_ascii_lowercase()))
        .first::<User>(conn as &MysqlConnection)
    {
        Ok(u) => Ok(u),
        Err(err) => Err(Json(json!({
            "Ok":false,
            "message":err.to_string()
        }))),
    }
}

pub fn register_user(conn: &DB, mut doc: User) -> Result<(), Json<JsonValue>> {
    use crate::database::schema::users;
    doc.user_name = doc.user_name.to_ascii_lowercase();

    match diesel::insert_into(users::table)
        .values(&doc)
        .execute(conn as &MysqlConnection)
    {
        Ok(_) => Ok(()),
        Err(err) => Err(Json(json!({
            "Ok":false,
            "message":err.to_string()
        }))),
    }
}