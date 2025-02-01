use crate::models::users::*;
use crate::schema::users;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct UsersRepository;

type UsersGetResult = Result<Option<User>, diesel::result::Error>;

impl UsersRepository {
    pub async fn find_by_id(conn: &mut AsyncPgConnection, id: i32) -> UsersGetResult {
        users::table.find(id).get_result(conn).await.optional()
    }

    pub async fn retrieve_by_limit(
        conn: &mut AsyncPgConnection,
        limit: i64,
    ) -> QueryResult<Vec<User>> {
        users::table.limit(limit).get_results(conn).await
    }

    pub async fn create(conn: &mut AsyncPgConnection, new_user: NewUser) -> UsersGetResult {
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)
            .await
            .optional()
    }

    pub async fn update(conn: &mut AsyncPgConnection, id: i32, user: UpdateUser) -> UsersGetResult {
        diesel::update(users::table.filter(users::id.eq(id)))
            .set(&user)
            .get_result(conn)
            .await
            .optional()
    }

    pub async fn delete(
        conn: &mut AsyncPgConnection,
        id: i32,
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(users::table.filter(users::id.eq(id)))
            .execute(conn)
            .await
    }
}
