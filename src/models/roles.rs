use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, serde::Serialize, serde::Deserialize)]
#[diesel(table_name = crate::schema::roles)]
pub struct Role {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, serde::Serialize, serde::Deserialize)]
#[diesel(table_name = crate::schema::roles)]
pub struct NewRole {
    pub code: String,
    pub name: String,
}

#[derive(Queryable, Selectable, AsChangeset, serde::Serialize, serde::Deserialize)]
#[diesel(table_name = crate::schema::roles)]
pub struct UpdateRole {
    pub code: Option<String>,
    pub name: Option<String>,
}
