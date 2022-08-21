use crate::{application_config, errors::Error, models::internal::*};
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

    pub async fn check_credentials(
        &self,
        username: &str,
        password: &str,
    ) -> Result<UserIdentity, Error> {
        unimplemented!();
    }

    pub async fn get_user_list(&self, identity: &UserIdentity) -> Result<u32, Error> {
        unimplemented!();
    }

    pub async fn get_all_list_ids(&self) -> Result<Vec<u32>, Error> {
        unimplemented!();
    }

    pub async fn create_list(&self, identity: &UserIdentity) -> Result<u32, Error> {
        unimplemented!();
    }

    async fn check_user_owns_list(&self, identity: &UserIdentity) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn delete_list(&self, identity: &UserIdentity, list_id: u32) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn add_entry(
        &self,
        identity: &UserIdentity,
        list_id: u32,
        entry_value: &str,
    ) -> Result<(), Error> {
        unimplemented!();
    }

    pub async fn delete_entry(
        &self,
        identity: &UserIdentity,
        list_id: u32,
        entry_id: u32,
    ) -> Result<(), Error> {
        unimplemented!();
    }

    pub async fn modify_entry_order(
        &self,
        identity: &UserIdentity,
        list_id: u32,
        version: u32,
        priorities: Vec<EntryPriority>,
    ) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn get_list(&self, list_id: u32, count: u32, offset: u32) -> Result<List, Error> {
        unimplemented!();
    }
}
