use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};

/// Throws an interval server response with `InternalServerError` status
pub fn throw_interval_error(e: Error) -> Custom<Json<Value>> {
    rocket::error!("Server Error");
    Custom(
        Status::InternalServerError,
        Json(json!({
            "error": "Internal Server Error",
            "details": e.to_string()
        })),
    )
}
