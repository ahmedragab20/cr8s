use crate::models::{NewRustacean, UpdateRustacean};
use crate::repositories::RustaceansRepository;
use crate::DBConnection;
use crate::*;
use cr8s::throw_interval_error;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket_db_pools::Connection;

#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(
    mut cnn: Connection<DBConnection>,
) -> Result<Json<Value>, Custom<Json<Value>>> {
    RustaceansRepository::retrieve_by_limit(&mut cnn, 100)
        .await
        .map(|r| Json(json!(r)))
        .map_err(throw_interval_error)
}

#[rocket::get("/rustaceans/<id>")]
pub async fn get_rustacean_by_id(
    mut connection: Connection<DBConnection>,
    id: i32,
) -> Result<Json<Value>, Custom<Json<Value>>> {
    RustaceansRepository::find_by_id(&mut connection, id)
        .await
        .map(|rustacean| {
            if let Some(r) = rustacean {
                Ok(Json(json!(r)))
            } else {
                Err(Custom(
                    Status::NotFound,
                    Json(json!({
                        "error": "Rustacean not found",
                        "details": "No rustacean found with this id"
                    })),
                ))
            }
        })
        .map_err(throw_interval_error)?
}

#[rocket::post("/rustaceans", data = "<rustacean>")]
pub async fn create_rustacean<'a>(
    mut cnn: Connection<DBConnection>,
    rustacean: Json<NewRustacean<'a>>,
) -> Result<Json<Value>, Custom<Json<Value>>> {
    RustaceansRepository::create(&mut cnn, rustacean.into_inner())
        .await
        .map(|r| Json(json!(r)))
        .map_err(throw_interval_error)
}

#[rocket::put("/rustaceans/<id>", data = "<updated_data>")]
pub async fn update_rustacean<'a>(
    mut cnn: Connection<DBConnection>,
    id: i32,
    updated_data: Json<UpdateRustacean<'a>>,
) -> Result<Json<Value>, Custom<Json<Value>>> {
    RustaceansRepository::update(&mut cnn, id, updated_data.into_inner())
        .await
        .map(|r| Json(json!(r)))
        .map_err(throw_interval_error)
}

#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rusacean(
    mut cnn: Connection<DBConnection>,
    id: i32,
) -> Result<Json<Value>, Custom<Json<Value>>> {
    RustaceansRepository::delete(&mut cnn, id)
        .await
        .map(|r| Json(json!(r)))
        .map_err(throw_interval_error)
}
