use super::db::PostgresPool;

pub struct Context {
    pub pool: PostgresPool,
}

impl juniper::Context for Context {}
