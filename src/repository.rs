use crate::application_config;
use crate::errors::Error;
use sqlx::postgres::{PgConnectOptions, PgConnection};
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

        sqlx::migrate!("./migrations/").run(&pool).await?;

        Ok(Repository { pool })
    }

    fn convert_error(err: sqlx::Error) -> Error {
        match err {
            sqlx::Error::Database(db_error) => {
                if let Some(constraint_name) = db_error.constraint() {
                    match constraint_name {
                        "unique_username" => Error::UsernameAlreadyExists,
                        _ => Error::SqlxError {
                            source: sqlx::Error::Database(db_error),
                        },
                    }
                } else {
                    Error::SqlxError {
                        source: sqlx::Error::Database(db_error),
                    }
                }
            }
            _ => err.into(),
        }
    }

    pub async fn create_user(&self, username: &str, hashed_password: &str) -> Result<(), Error> {
        let mut connection = self.pool.acquire().await?;

        sqlx::query!(
            "insert into Users(username, hashed_password) values ($1, $2)",
            username,
            hashed_password
        )
        .execute(&mut connection)
        .await
        .map_err(|err| Self::convert_error(err))?;

        Ok(())
    }
}
