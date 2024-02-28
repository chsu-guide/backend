use eyre::Result;
use futures::{future::BoxFuture, stream::BoxStream};
use sqlx::{
    pool::PoolConnection,
    postgres::{PgPoolOptions, PgQueryResult, PgRow, PgStatement, PgTypeInfo},
    Describe, Either, Error as SqlxError, Execute, Executor, PgPool, Postgres, Transaction,
};

#[derive(Debug)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub fn new(uri: &str) -> Result<Self> {
        let pool = PgPoolOptions::new().connect_lazy(uri)?;

        tokio::spawn(refresh_materialized_views(pool.clone()));

        Ok(Self { pool })
    }

    /// Retrieves a connection from the pool.
    pub(crate) async fn acquire(&self) -> Result<PoolConnection<Postgres>, SqlxError> {
        self.pool.acquire().await
    }

    /// Retrieves a connection and immediately begins a new transaction.
    pub(crate) async fn begin(&self) -> Result<Transaction<'static, Postgres>, SqlxError> {
        self.pool.begin().await
    }
}