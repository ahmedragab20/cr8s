use chrono::NaiveDateTime;
use diesel::prelude::*;
use std::borrow::Cow;

#[derive(Queryable, Selectable, serde::Serialize, serde::Deserialize)]
#[diesel(table_name = crate::schema::rustaceans)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, serde::Serialize, serde::Deserialize)]
#[diesel(table_name = crate::schema::rustaceans)]
pub struct NewRustacean<'a> {
    pub name: Cow<'a, str>,
    pub email: Cow<'a, str>,
}

#[derive(Queryable, Selectable, AsChangeset, serde::Serialize, serde::Deserialize)]
#[diesel(table_name = crate::schema::rustaceans)]
pub struct UpdateRustacean<'a> {
    pub name: Option<&'a str>,
    pub email: Option<&'a str>,
}
