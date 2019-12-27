use super::context::Context as GraphQLContext;
use diesel::pg::PgConnection;
use juniper::{FieldResult, RootNode};

use super::data::Todos;
use super::models::Todo;

pub struct Query;

#[juniper::object(Context = GraphQLContext)]
impl Query {
    #[graphql(name = "allTodos")]
    pub fn all_todos(context: &GraphQLContext) -> FieldResult<Vec<Todo>> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Todos::all_todos(conn)
    }
}

pub struct Mutation;

#[juniper::object(Context = GraphQLContext)]
impl Mutation {}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}
