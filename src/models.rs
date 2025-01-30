use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde;

#[derive(Queryable, Selectable, serde::Serialize, serde::Deserialize)]
#[diesel(table_name = crate::schema::rustaceans)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::rustaceans)]
pub struct NewRustacean<'a> {
    pub name: &'a str,
    pub email: &'a str,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::rustaceans)]
pub struct UpdateRustacean<'a> {
    pub name: &'a str,
    pub email: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::crates)]
pub struct Crate {
    pub id: i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::crates)]
pub struct NewCrate<'a> {
    pub rustacean_id: i32,
    pub code: &'a str,
    pub name: &'a str,
    pub version: &'a str,
    pub description: Option<&'a str>,
}
