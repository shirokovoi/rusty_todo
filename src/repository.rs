use crate::{application_config, errors::Error, models::internal::*};
use sqlx::postgres::PgConnectOptions;
use sqlx::{Executor, PgPool, Postgres};

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
                        "unique_owner" => Error::TodoListAlreadyCreated,
                        _ => Error::SqlxError {
                            source: sqlx::Error::Database(db_error),
                        },
                    }
                } else {
                    Error::SqlxError {
                        source: sqlx::Error::Database(db_error),
                    }
                }
            },
            sqlx::Error::RowNotFound => {
                Error::NotFound
            },
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
        .map_err(Self::convert_error)?;

        Ok(())
    }

    pub async fn check_credentials<F: Fn(&str) -> Result<bool, Error>>(
        &self,
        username: &str,
        pasword_checker: F,
    ) -> Result<UserIdentity, Error> {
        let mut connection = self.pool.acquire().await?;
        let user_info = sqlx::query!(
            "select id, hashed_password from Users where username = $1",
            username
        )
        .fetch_one(&mut connection)
        .await?;

        let is_password_match = pasword_checker(&user_info.hashed_password)?;
        if is_password_match {
            Ok(UserIdentity {
                username: username.to_owned(),
                id: user_info.id as u32,
            })
        } else {
            Err(Error::Unauthorized)
        }
    }

    pub async fn get_user_list(&self, identity: &UserIdentity) -> Result<u32, Error> {
        let mut connection = self.pool.acquire().await?;

        let list_id = sqlx::query!(
            "select id from todolist where owner_id = $1",
            identity.id as i32
        )
        .fetch_one(&mut connection)
        .await?;

        Ok(list_id.id as u32)
    }

    pub async fn get_all_list_ids(&self) -> Result<Vec<u32>, Error> {
        let mut connection = self.pool.acquire().await?;

        let ids = sqlx::query!("select id from todolist")
            .fetch_all(&mut connection)
            .await?;

        Ok(ids.into_iter().map(|record| record.id as u32).collect())
    }

    pub async fn create_list(&self, identity: &UserIdentity) -> Result<u32, Error> {
        let mut connection = self.pool.acquire().await?;

        let result = sqlx::query!(
            "insert into todolist(owner_id) values ($1) returning id",
            identity.id as i32
        )
        .fetch_one(&mut connection)
        .await;

        result
            .map_err(Self::convert_error)
            .map(|record| record.id as u32)
    }

    pub async fn delete_list(&self, identity: &UserIdentity, list_id: u32) -> Result<(), Error> {
        let mut connection = self.pool.acquire().await?;

        sqlx::query!(
            "delete from todolist where id = $1 and owner_id = $2",
            list_id as i32,
            identity.id as i32
        )
        .execute(&mut connection)
        .await?;
        unimplemented!()
    }

    async fn check_user_owns_list(
        user_id: u32,
        list_id: u32,
        executor: impl Executor<'_, Database = Postgres>,
    ) -> Result<bool, Error> {
        let result = sqlx::query!(
            "select id from todolist where owner_id = $1",
            user_id as i32
        )
        .fetch_one(executor)
        .await;

        match result {
            Ok(record) => {
                let id = record.id as u32;
                if id == list_id {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(sqlx::Error::RowNotFound) => Ok(false),
            Err(err) => Err(err.into()),
        }
    }

    async fn update_list_version(list_id: u32, executor: impl Executor<'_, Database = Postgres>) -> Result<(), Error> {
        sqlx::query!(
            "update todolist set version = version + 1 where id = $1",
            list_id as i32
        )
        .execute(executor)
        .await?;

        Ok(())
    }


    // TODO get rid of copy-paste (move transaction begin/commit and user check into function that
    // takes closure)
    pub async fn add_entry(
        &self,
        identity: &UserIdentity,
        list_id: u32,
        version: u32,
        entry_value: &str,
    ) -> Result<(), Error> {
        let mut transaction = self.pool.begin().await?;

        let check_ok = Self::check_user_owns_list(identity.id, list_id, &mut transaction).await?;
        if !check_ok {
            return Err(Error::Forbidden);
        }

        sqlx::query!(
            "insert into todoentry(priority, list_id, value) values ((select max(id) from todoentry where list_id = $1), (select id from todolist where id = $1 and version = $3), $2)", 
            list_id as i32,
            entry_value, 
            version as i32,
        )
            .execute(&mut transaction)
            .await.map_err(Self::convert_error)?;

        Self::update_list_version(list_id, &mut transaction).await?;

        transaction.commit().await?;
        Ok(())
    }

    pub async fn delete_entry(
        &self,
        identity: &UserIdentity,
        version: u32,
        list_id: u32,
        entry_id: u32,
    ) -> Result<(), Error> {
        let mut transaction = self.pool.begin().await?;

        let check_ok = Self::check_user_owns_list(identity.id, list_id, &mut transaction).await?;
        if !check_ok {
            return Err(Error::Forbidden);
        }

        sqlx::query!("delete from todoentry where id = $1 and list_id = (select id from todolist where version = $2 and id = $3)", entry_id as i32, version as i32, list_id as i32).execute(&mut transaction).await.map_err(Self::convert_error)?;

        Self::update_list_version(list_id, &mut transaction).await?;
        transaction.commit().await?;
        Ok(())
    }

    pub async fn modify_entry_order(
        &self,
        identity: &UserIdentity,
        list_id: u32,
        version: u32,
        priorities: Vec<EntryPriority>,
    ) -> Result<(), Error> {
        let mut transaction = self.pool.begin().await?;

        let check_ok = Self::check_user_owns_list(identity.id, list_id, &mut transaction).await?;
        if !check_ok {
            return Err(Error::Forbidden);
        }

        let (ids, priority_values): (Vec<_>, Vec<_>) = priorities.iter().map(|entry| { (entry.entry_id as i32, entry.priority as i32) }).unzip();
        sqlx::query!(
            r#"update todoentry set priority = bulk_query.priority from
            (select * from unnest($1::integer[], $2::integer[]) as t(priority, id)) as bulk_query 
            where todoentry.id = bulk_query.id and list_id = (select id from todolist where list_id = $3 and version = $4)"#,
            &priority_values,
            &ids,
            list_id as i32,
            version as i32
        )
        .execute(&mut transaction)
        .await?;

        Self::update_list_version(list_id, &mut transaction).await?;
        transaction.commit().await?;
        Ok(())

    }

    pub async fn get_list(&self, list_id: u32, count: u32, offset: u32) -> Result<List, Error> {
        let mut connection = self.pool.acquire().await?;

        let list = sqlx::query!("select todolist.version as version, todoentry.id as todoentry_id, todoentry.priority as priority, todoentry.value as value, count(*) OVER() as total from todoentry left join todolist on todolist.id = todoentry.list_id where todoentry.list_id = $1 order by todoentry.priority limit $2 offset $3", list_id as i32, count as i32, offset as i32).fetch_all(&mut connection).await?;

        let (version, count) = list.first().ok_or(Error::InternalError).map(|record| { (record.version as u32, record.total.unwrap_or(0) as u32) })?;
        let entries = list.into_iter().map(|record| {
            ListEntry { id: record.todoentry_id as u32, priority: record.priority as u32, value: record.value }
        }).collect();

        Ok(List {
            entries,
            version,
            entiries_count: count
        })
    }
}
