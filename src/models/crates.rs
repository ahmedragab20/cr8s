use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, serde::Serialize)]
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

#[derive(Insertable, serde::Serialize, serde::Deserialize)]
#[diesel(table_name = crate::schema::crates)]
pub struct NewCrate<'a> {
    pub rustacean_id: i32,
    pub code: &'a str,
    pub name: &'a str,
    pub version: &'a str,
    pub description: Option<&'a str>,
}
