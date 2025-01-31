use rocket::routes;
use rocket_db_pools::Database;
mod models;
mod repositories;
mod rocket_routes;
mod schema;

use rocket_routes::crates as crates_routes;
use rocket_routes::rustaceans as rustaceans_routes;

#[derive(Database)]
#[database("postgres")]
struct DBConnection(rocket_db_pools::diesel::PgPool);

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                // Rustaceans
                rustaceans_routes::get_rustaceans,
                rustaceans_routes::get_rustacean_by_id,
                rustaceans_routes::create_rustacean,
                rustaceans_routes::update_rustacean,
                rustaceans_routes::delete_rusacean,
                // Crates
                crates_routes::get_crates,
                crates_routes::get_crate_by_id,
                crates_routes::get_crates_since,
                crates_routes::create_crate,
                crates_routes::update_crate,
                crates_routes::delete_crate,
            ],
        )
        .attach(DBConnection::init())
        .launch()
        .await;
}
