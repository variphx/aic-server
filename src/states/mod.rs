use std::sync::Arc;

use deadpool_diesel::{
    Runtime,
    postgres::{Manager, Pool},
};
use qdrant_client::Qdrant;

#[derive(Clone)]
pub struct AppState {
    diesel_pool: Pool,
    qdrant_client: Arc<Qdrant>,
}

impl AppState {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            diesel_pool: Self::diesel_pool_helper()?,
            qdrant_client: Self::qdrant_client_helper()?,
        })
    }

    fn diesel_pool_helper() -> anyhow::Result<Pool> {
        Pool::builder(Manager::new(
            std::env::var("DATABASE_URL")?,
            Runtime::Tokio1,
        ))
        .build()
        .map_err(Into::into)
    }

    fn qdrant_client_helper() -> anyhow::Result<Arc<Qdrant>> {
        Qdrant::from_url(&std::env::var("QDRANT_URL")?)
            .build()
            .map(Arc::new)
            .map_err(Into::into)
    }
}

impl AppState {
    pub fn pool(&self) -> &Pool {
        &self.diesel_pool
    }

    pub fn qdrant_client(&self) -> &Qdrant {
        &self.qdrant_client
    }
}
