use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "rustaceans"]
pub struct NewRustacean<'a> {
    pub name: &'a str,
    pub email: &'a str,
}

#[derive(Queryable)]
pub struct Crate {
    pub id: i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}

#[derive(Insertable)]
#[table_name = "crates"]
pub struct NewCrate<'a> {
    pub rustacean_id: i32,
    pub code: &'a str,
    pub name: &'a str,
    pub version: &'a str,
    pub description: Option<&'a str>,
}
