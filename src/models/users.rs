use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, serde::Serialize, serde::Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, serde::Serialize, serde::Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub user_name: String,
    pub password: String,
}

#[derive(Queryable, Selectable, AsChangeset, serde::Serialize, serde::Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct UpdateUser {
    pub user_name: Option<String>,
    pub password: Option<String>,
}
