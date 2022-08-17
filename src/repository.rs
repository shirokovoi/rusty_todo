use crate::errors::Error;
use sqlx::PgPool;

pub struct Repository {
    pool: PgPool,
}

impl Clone for Repository {
    fn clone(&self) -> Self {
        unimplemented!();
    }
}

impl Repository {
    pub fn new() -> Result<Self, Error> {
        unimplemented!();
    }
}
