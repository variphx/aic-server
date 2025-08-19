use deadpool_diesel::{
    Runtime,
    postgres::{Manager, Pool},
};

#[derive(Clone)]
pub struct AppState {
    pool: Pool,
}

impl AppState {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            pool: Pool::builder(Manager::new(
                std::env::var("DATABASE_URL")?,
                Runtime::Tokio1,
            ))
            .build()?,
        })
    }
}

impl AppState {
    pub fn pool(&self) -> &Pool {
        &self.pool
    }
}
