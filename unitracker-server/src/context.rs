use unitracker_chsu::ChsuClient;
use unitracker_psql::database::Database;

pub struct Clients {
    psql: Database,
    chsu: ChsuClient,
}

impl Clients {
    pub fn chsu(&self) -> &ChsuClient {
        &self.chsu
    }

    pub fn psql(&self) -> &Database {
        &self.psql
    }
}
pub struct Context {
    clients: Clients,
}
