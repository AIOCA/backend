#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

pub mod auth;
pub mod database;
pub mod http;
pub mod requests;

use crate::auth::cors::CORS;
use crate::database::core::DB;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    rocket::ignite()
        .mount(
            "/",
            routes![
                requests::register::user_register,
                requests::login::user_login,
                requests::estimate::estimate,
                requests::route::calculate_path,
            ],
        )
        .attach(DB::fairing())
        .attach(CORS())
        .launch();
}
