pub mod estimate;
pub mod login;
pub mod register;
pub mod route;

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

///StartEndCoordinates
///used as structure for serialising requests with coordinates
#[derive(Deserialize)]
pub struct StartEndCoordinates {
    pub start_lat: f64,
    pub start_long: f64,
    pub stop_lat: f64,
    pub stop_long: f64,
}

#[derive(Deserialize)]
struct YourNavigationProperties {
    distance:String,
}

#[derive(Deserialize)]
pub struct YourNavigationResponse {
    coordinates: Vec<Vec<f64>>,
    properties:YourNavigationProperties,
}


#[derive(Deserialize)]
pub struct EstimateData {
    distance:f64,
}
