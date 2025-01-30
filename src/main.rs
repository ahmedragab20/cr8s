use crate::repositories::RustaceansRepository;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::routes;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket_db_pools::{Connection, Database};
use serde::de::Unexpected::Option;

mod models;
mod repositories;
mod schema;

#[derive(Database)]
#[database("postgres")]
struct DBConnection(rocket_db_pools::diesel::PgPool);

#[rocket::get("/rustaceans")]
async fn get_rustaceans(mut cnn: Connection<DBConnection>) -> Result<Value, Custom<Value>> {
    RustaceansRepository::retrieve_by_limit(&mut cnn, 100)
        .await
        .map(|r| json!(r))
        .map_err(|_| Custom(Status::InternalServerError, json!("Internal Server Error")))
}
#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![get_rustaceans])
        .attach(DBConnection::init())
        .launch()
        .await;
}
