use crate::models::roles::*;
use crate::schema::roles;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct RolesRepository;

type RolesGetResult = Result<Option<Role>, diesel::result::Error>;

impl RolesRepository {
    pub async fn find_by_id(conn: &mut AsyncPgConnection, id: i32) -> RolesGetResult {
        roles::table.find(id).get_result(conn).await.optional()
    }

    pub async fn retrieve_by_limit(
        conn: &mut AsyncPgConnection,
        limit: i64,
    ) -> QueryResult<Vec<Role>> {
        roles::table.limit(limit).get_results(conn).await
    }

    pub async fn create(conn: &mut AsyncPgConnection, new_role: NewRole) -> RolesGetResult {
        diesel::insert_into(roles::table)
            .values(&new_role)
            .get_result(conn)
            .await
            .optional()
    }

    pub async fn update(conn: &mut AsyncPgConnection, id: i32, role: UpdateRole) -> RolesGetResult {
        diesel::update(roles::table.filter(roles::id.eq(id)))
            .set(&role)
            .get_result(conn)
            .await
            .optional()
    }

    pub async fn delete(
        conn: &mut AsyncPgConnection,
        id: i32,
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(roles::table.filter(roles::id.eq(id)))
            .execute(conn)
            .await
    }
}
