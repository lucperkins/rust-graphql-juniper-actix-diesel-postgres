use super::db::PostgresPool;

pub struct GraphQLContext {
    pub pool: PostgresPool,
}

impl juniper::Context for GraphQLContext {}
