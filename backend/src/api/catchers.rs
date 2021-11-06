use rocket::Catcher;
use serde_json::{json, Value};

#[catch(404)]
async fn not_found() -> Value {
    json!({"message": "Resource was not found!"})
}

#[catch(default)]
async fn default() -> Value {
    json!({"message": "Api couldn't handle the request!"})
}

pub fn api_catchers() -> Vec<Catcher> {
    catchers![not_found, default]
}
