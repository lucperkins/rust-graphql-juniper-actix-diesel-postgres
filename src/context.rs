use super::db::PostgresPool;

// The GraphQL context, which needs to provide everything necessary for
// interacting with the database.
pub struct GraphQLContext {
    pub pool: PostgresPool,
}

// This impl allows us to pass in GraphQLContext as the Context for GraphQL
// objects
impl juniper::Context for GraphQLContext {}
