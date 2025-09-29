use std::sync::Arc;

use unitracker_psql::database::Database;

pub struct ApplicationContext {
    database: Database,
}

impl ApplicationContext {
    pub fn database(&self) -> &Database {
        &self.database
    }
}

pub fn create_context(database_uri: &str) -> Arc<ApplicationContext> {
    let db = Database::new(database_uri).unwrap();
    let ctx = ApplicationContext { database: db };
    Arc::new(ctx)
}
