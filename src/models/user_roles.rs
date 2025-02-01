use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::user_roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, serde::Serialize, serde::Deserialize)]
#[diesel(table_name = crate::schema::user_roles)]
pub struct NewUserRole {
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Queryable, Selectable, AsChangeset, serde::Serialize, serde::Deserialize)]
#[diesel(table_name = crate::schema::user_roles)]
pub struct UpdateUserRole {
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
}
