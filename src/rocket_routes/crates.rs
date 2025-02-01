use crate::models::crates::NewCrate;
use crate::repositories::crates::CratesRepository;
use crate::DBConnection;
use crate::*;
use cr8s::throw_interval_error;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket_db_pools::Connection;

#[rocket::get("/crates")]
pub async fn get_crates(
    mut cnn: Connection<DBConnection>,
) -> Result<Json<Value>, Custom<Json<Value>>> {
    CratesRepository::retrieve_by_limit(&mut cnn, 100)
        .await
        .map(|crates| Json(json!(crates)))
        .map_err(throw_interval_error)
}

#[rocket::get("/crates/<id>")]
pub async fn get_crate_by_id(
    mut connection: Connection<DBConnection>,
    id: i32,
) -> Result<Json<Value>, Custom<Json<Value>>> {
    CratesRepository::find_by_id(&mut connection, id)
        .await
        .map(|a_crate| {
            if let Some(c) = a_crate {
                Ok(Json(json!(c)))
            } else {
                Err(Custom(
                    Status::NotFound,
                    Json(json!({
                        "error": "Crate not found",
                        "details": "No crate found with this id"
                    })),
                ))
            }
        })
        .map_err(throw_interval_error)?
}

#[rocket::get("/crates/since/<hours>")]
pub async fn get_crates_since(
    mut connection: Connection<DBConnection>,
    hours: i32,
) -> Result<Json<Value>, Custom<Json<Value>>> {
    CratesRepository::find_since(&mut connection, hours)
        .await
        .map(|crates| Json(json!(crates)))
        .map_err(throw_interval_error)
}

#[rocket::post("/crates", data = "<a_crate>")]
pub async fn create_crate<'a>(
    mut cnn: Connection<DBConnection>,
    a_crate: Json<NewCrate<'a>>,
) -> Result<Json<Value>, Custom<Json<Value>>> {
    CratesRepository::create(&mut cnn, a_crate.into_inner())
        .await
        .map(|c| Json(json!(c)))
        .map_err(throw_interval_error)
}

#[rocket::put("/crates/<id>", data = "<a_crate>")]
pub async fn update_crate<'a>(
    mut cnn: Connection<DBConnection>,
    id: i32,
    a_crate: Json<NewCrate<'a>>,
) -> Result<Json<Value>, Custom<Json<Value>>> {
    CratesRepository::update(&mut cnn, id, a_crate.into_inner())
        .await
        .map(|c| Json(json!(c)))
        .map_err(throw_interval_error)
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(
    mut cnn: Connection<DBConnection>,
    id: i32,
) -> Result<Json<Value>, Custom<Json<Value>>> {
    CratesRepository::delete(&mut cnn, id)
        .await
        .map(|c| Json(json!(c)))
        .map_err(throw_interval_error)
}
