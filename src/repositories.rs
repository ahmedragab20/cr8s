use crate::models::{Crate, NewCrate, NewRustacean, Rustacean, UpdateRustacean};
use crate::schema::crates::dsl as crates_dsl;
use crate::schema::rustaceans::dsl as rustaceans_dsl;
use diesel::associations::HasTable;
use diesel::dsl::{now, IntervalDsl};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, QueryResult};
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
        rustacean: Rustacean,
        updated_data: UpdateRustacean<'_>,
    ) -> RustaceansGetResult {
        diesel::update(rustaceans_dsl::rustaceans::table().find(rustacean.id))
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

pub struct CratesRepository;
type CratesGetResult = Result<Option<Crate>, diesel::result::Error>;

impl CratesRepository {
    pub async fn find_by_id(c: &mut AsyncPgConnection, id: i32) -> CratesGetResult {
        crates_dsl::crates::table()
            .find(id)
            .get_result(c)
            .await
            .optional()
    }

    pub async fn retrieve_by_limit(
        c: &mut AsyncPgConnection,
        limit: i64,
    ) -> QueryResult<Vec<Crate>> {
        crates_dsl::crates::table().limit(limit).load(c).await
    }

    pub async fn find_since(
        c: &mut AsyncPgConnection,
        hours_since: i32,
    ) -> QueryResult<Vec<Crate>> {
        crates_dsl::crates::table()
            .filter(crates_dsl::created_at.ge(now - hours_since.hours()))
            .load(c)
            .await
    }

    pub async fn create<'a>(c: &mut AsyncPgConnection, new_crate: NewCrate<'a>) -> CratesGetResult {
        diesel::insert_into(crates_dsl::crates::table())
            .values(&new_crate)
            .get_result(c)
            .await
            .optional()
    }

    pub async fn update<'a>(
        c: &mut AsyncPgConnection,
        id: i32,
        a_crate: NewCrate<'a>,
    ) -> CratesGetResult {
        diesel::update(crates_dsl::crates::table().find(id))
            .set((
                crates_dsl::rustacean_id.eq(a_crate.rustacean_id),
                crates_dsl::name.eq(a_crate.name),
                crates_dsl::code.eq(a_crate.code),
                crates_dsl::version.eq(a_crate.version),
                crates_dsl::description.eq(a_crate.description),
            ))
            .get_result(c)
            .await
            .optional()
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(crates_dsl::crates::table().find(id))
            .execute(c)
            .await
    }
}
