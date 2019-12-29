use actix::Addr;
use super::db::DbExecutor;

#[derive(Clone)]
pub struct GraphQLContext {
    pub db: Addr<DbExecutor>,
}

impl GraphQLContext {
    pub fn new(db: Addr<DbExecutor>) -> Self {
        Self {
            db: db,
        }
    }
}

impl juniper::Context for GraphQLContext {}
