//use crate::auth::auth_user;
use crate::requests::EstimatePayload;
use rand::seq::SliceRandom;
use rocket_contrib::json::{Json, JsonValue};

#[post("/commute/estimate", data = "<data>")]
pub fn estimate(data: Json<EstimatePayload>) -> Result<Json<JsonValue>, Json<JsonValue>> {
    let mut products = [
        ("Micro", 8),
        ("Mini", 10),
        ("Prime Sedan", 50),
        ("Prime SUV", 16),
        ("UberGo", 27),
        ("Pool", 27),
        ("Go Executive", 25),
        ("Premier", 30),
        ("Premier Executive", 30),
        ("UberXL", 38),
        ("Go Intercity", 100),
        ("UberGo", 64),
        ("Premier", 50),
        ("Premier Intercity", 125),
        ("XL Intercity", 184),
    ];
    products.shuffle(&mut rand::thread_rng());
    let distance = (((data.start_lat - data.stop_lat) * 100.0).powi(2)
        + ((data.start_long - data.stop_long) * 100.0).powi(2))
    .sqrt();
    let products: Vec<(&str, f64)> = products
        .iter()
        .map(|p| (p.0, f64::from(p.1) * distance))
        .collect();

    print!("distance {}", distance);

    Ok(Json(json!({
        "Ok": true,
        "products":products,
    })))
}
