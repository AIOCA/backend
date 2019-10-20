pub mod login;
pub mod register;

use serde_derive::Deserialize;

///LoginForm
///used as structure for serialising login request forms
#[derive(FromForm, Deserialize, Debug)]
pub struct LoginForm {
    pub name: String,
    pub password: String,
}

///AuthRequestPayload
///A general type structure for requests that requires authentication
#[derive(Deserialize)]
pub struct AuthRequestPayload<T> {
    pub token: String,
    pub data: T,
}
