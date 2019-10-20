use crate::database::schema::{admins,users};
use serde_derive::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, FromForm, Insertable, Debug, Deserialize, Serialize)]
#[primary_key(user_id)]
#[table_name = "users"]
pub struct User {
    pub user_id: Option<i32>,
    pub user_name: String,
    pub password: String,
    pub email: String,
    pub display_name: String,
    pub address: String,
}

#[derive(Identifiable, Queryable, FromForm, Associations, Insertable)]
#[belongs_to(User, foreign_key = "user_id")]
#[primary_key(user_id)]
#[table_name = "admins"]
pub struct Admin {
    user_id: Option<i32>,
}