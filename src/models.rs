use juniper::GraphQLObject;

#[derive(Queryable, GraphQLObject)]
pub struct Todo {
    pub id: i32,
    pub task: String,
    pub done: bool,
}
