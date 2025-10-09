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

pub struct ContextParameters {
    pub connection_string: String,
}

impl Context {
    pub async fn init(params: ContextParameters) -> Self {
        Self {
            clients: Clients {
                psql: Database::new(&params.connection_string).unwrap(),
                chsu: ChsuClient::new().await,
            },
        }
    }
    pub fn chsu(&self) -> &ChsuClient {
        &self.clients.chsu()
    }

    pub fn database(&self) -> &Database {
        &self.clients.psql()
    }
}
