use crate::models::user_roles::*;
use crate::schema::user_roles;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct UserRolesRepository;

type UserRolesGetResult = Result<Option<UserRole>, diesel::result::Error>;

impl UserRolesRepository {
    pub async fn find_by_id(conn: &mut AsyncPgConnection, id: i32) -> UserRolesGetResult {
        user_roles::table
            .select(UserRole::as_select())
            .filter(user_roles::id.eq(id))
            .first(conn)
            .await
            .optional()
    }

    pub async fn retrieve_by_limit(
        conn: &mut AsyncPgConnection,
        limit: i64,
    ) -> QueryResult<Vec<UserRole>> {
        user_roles::table
            .select(UserRole::as_select())
            .limit(limit)
            .get_results(conn)
            .await
    }

    pub async fn create(
        conn: &mut AsyncPgConnection,
        new_user_role: NewUserRole,
    ) -> UserRolesGetResult {
        diesel::insert_into(user_roles::table)
            .values(&new_user_role)
            .returning(UserRole::as_select())
            .get_result(conn)
            .await
            .optional()
    }

    pub async fn update(
        conn: &mut AsyncPgConnection,
        id: i32,
        user_role: UpdateUserRole,
    ) -> UserRolesGetResult {
        diesel::update(user_roles::table.filter(user_roles::id.eq(id)))
            .set(&user_role)
            .returning(UserRole::as_select())
            .get_result(conn)
            .await
            .optional()
    }

    pub async fn delete(
        conn: &mut AsyncPgConnection,
        id: i32,
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(user_roles::table.filter(user_roles::id.eq(id)))
            .execute(conn)
            .await
    }
}
