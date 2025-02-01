use crate::models::rustaceans::{NewRustacean, Rustacean, UpdateRustacean};
use crate::schema::rustaceans::dsl as rustaceans_dsl;
use diesel::associations::HasTable;
use diesel::{OptionalExtension, QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct RustaceansRepository;

type RustaceansGetResult = Result<Option<Rustacean>, diesel::result::Error>;
impl RustaceansRepository {
    pub async fn find_by_id(conn: &mut AsyncPgConnection, id: i32) -> RustaceansGetResult {
        rustaceans_dsl::rustaceans::table()
            .find(id)
            .get_result(conn)
            .await
            .optional()
    }

    pub async fn retrieve_by_limit(
        conn: &mut AsyncPgConnection,
        limit: i64,
    ) -> QueryResult<Vec<Rustacean>> {
        rustaceans_dsl::rustaceans::table()
            .limit(limit)
            .get_results(conn)
            .await
    }

    pub async fn create(
        conn: &mut AsyncPgConnection,
        new_rustacean: NewRustacean<'_>,
    ) -> RustaceansGetResult {
        diesel::insert_into(rustaceans_dsl::rustaceans::table())
            .values(&new_rustacean)
            .get_result(conn)
            .await
            .optional()
    }

    pub async fn update(
        conn: &mut AsyncPgConnection,
        id: i32,
        updated_data: UpdateRustacean<'_>,
    ) -> RustaceansGetResult {
        diesel::update(rustaceans_dsl::rustaceans::table().find(id))
            .set(updated_data)
            .get_result(conn)
            .await
            .optional()
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans_dsl::rustaceans::table().find(id))
            .execute(conn)
            .await
    }
}
