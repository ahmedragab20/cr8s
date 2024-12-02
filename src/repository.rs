use crate::{
    models::{NewRustacean, Rustacean},
    schema::rustaceans,
};
use diesel::{QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct Repository;

impl Repository {
    pub async fn find(connection: &mut AsyncPgConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).first(connection).await
    }
    pub async fn find_many(connection: &mut AsyncPgConnection) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table.load::<Rustacean>(connection).await
    }
    pub async fn create(connection: &mut AsyncPgConnection, rustacean: NewRustacean) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table).values(rustacean).execute(connection).await?;
        rustaceans::table.find(rustaceans::id).first(connection).await
    }
}
