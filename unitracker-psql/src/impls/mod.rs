use eyre::Context;
use sqlx::migrate::Migrator;

use crate::database::Database;

pub mod auditorium;
pub mod building;
pub mod class;
mod complex;
pub mod discipline;
pub mod group;
pub mod teacher;

impl Database {
    #[tracing::instrument]
    pub async fn transaction(&self) -> eyre::Result<()> {
        sqlx::query!("BEGIN")
            .execute(self)
            .await
            .wrap_err("Failed to start a transaction")?;
        Ok(())
    }
    #[tracing::instrument]
    pub async fn commit(&self) -> eyre::Result<()> {
        sqlx::query!("COMMIT")
            .execute(self)
            .await
            .wrap_err("Failed to commit a transaction")?;
        Ok(())
    }
}
