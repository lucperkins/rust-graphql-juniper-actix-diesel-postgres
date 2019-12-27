use juniper::{GraphQLInputObject, GraphQLObject};
use super::schema::todos;

#[derive(Queryable, GraphQLObject)]
pub struct Todo {
    pub id: i32,
    pub task: String,
    pub done: bool,
}

#[derive(Insertable)]
#[table_name = "todos"]
pub struct NewTodo<'a> {
    pub task: &'a str,
    pub done: &'a bool,
}

#[derive(GraphQLInputObject)]
pub struct CreateTodoInput {
    pub task: String,
    pub done: bool,
}
