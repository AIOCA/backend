use crate::http::{HttpMethod, HttpRequest};
use crate::requests::{StartEndCoordinates, YourNavigationResponse};
use rocket_contrib::json::{Json, JsonValue};
use serde_json::from_str;

///http://www.yournavigation.org/api/1.0/gosmore.php?flat=28.6337465&flon=77.35780799999999&tlat=28.6388179&tlon=77.3606496&format=geojson
/// huhu
#[post("/commute/route", data = "<data>")]
pub fn calculate_path(data: Json<StartEndCoordinates>) -> Result<Json<JsonValue>, Json<JsonValue>> {
    let request = HttpRequest{
        url:format!("http://www.yournavigation.org/api/1.0/gosmore.php?flat={}&flon={}&tlat={}&tlon={}&format=geojson",data.start_lat,data.start_long,data.stop_lat,data.stop_long),
        body:None,
        method:HttpMethod::Get,
    };
    match request.make_request() {
        Ok(response) => {
            let response: YourNavigationResponse = from_str(&response).unwrap();
            Ok(Json(json!({
                "Ok": true,
                "waypoint":response.coordinates,
                "distance":response.properties.distance,
            })))
        }
        Err(err) => Ok(Json(json!({
            "Ok": false,
            "message":err.to_string(),
        }))),
    }
}
