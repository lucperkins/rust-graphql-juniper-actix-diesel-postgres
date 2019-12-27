use super::context::Context as GraphQLContext;
use diesel::pg::PgConnection;
use juniper::{FieldResult, RootNode};

use super::data::Todos;
use super::models::{CreateTodoInput, Todo};

pub struct Query;

#[juniper::object(Context = GraphQLContext)]
impl Query {
    #[graphql(name = "allTodos")]
    pub fn all_todos(context: &GraphQLContext) -> FieldResult<Vec<Todo>> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Todos::all_todos(conn)
    }

    #[graphql(name = "getTodoById")]
    pub fn get_todo_by_id(context: &GraphQLContext, id: i32) -> FieldResult<Todo> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Todos::get_todo_by_id(conn, id)
    }
}

pub struct Mutation;

#[juniper::object(Context = GraphQLContext)]
impl Mutation {
    #[graphql(name = "createTodo")]
    pub fn create_todo(
        context: &GraphQLContext,
        input: CreateTodoInput,
    ) -> FieldResult<Todo> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Todos::create_todo(conn, input)
    }

    #[graphql(name = "markTodoAsDone")]
    pub fn mark_todo_as_done(context: &GraphQLContext, id: i32) -> FieldResult<Todo> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Todos::mark_todo_as_done(conn, id)
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}
