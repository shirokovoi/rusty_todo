use crate::application_config;
use crate::errors::Error;
use sqlx::postgres::PgConnectOptions;
use sqlx::PgPool;

pub struct Repository {
    pool: PgPool,
}

impl Clone for Repository {
    fn clone(&self) -> Self {
        Repository {
            pool: self.pool.clone(),
        }
    }
}

impl Repository {
    // TODO configuration for pool options (connections count, timeouts, etc.)
    pub async fn new(db_config: &application_config::DB) -> Result<Self, Error> {
        let connection_options = PgConnectOptions::new()
            .host(&db_config.host)
            .port(db_config.port)
            .username(&db_config.username)
            .password(&db_config.password)
            .database("rusty_todo");

        let pool = PgPool::connect_with(connection_options).await?;
        Ok(Repository { pool })
    }
}
