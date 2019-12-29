use super::schema::todos;
use juniper::GraphQLInputObject;

// The core data type undergirding the GraphQL interface
#[derive(Queryable)]
pub struct Todo {
    pub id: i32,
    pub task: String,
    pub done: bool,
}

// I've chosen to make the fields explicit, but could've gotten the same result
// applying #[derive(juniper::GraphQLObject)] to the Todo struct above
#[juniper::object]
impl Todo {
    #[graphql(name = "id")]
    fn id(&self) -> i32 {
        self.id
    }

    #[graphql(name = "task")]
    pub fn task(&self) -> &str {
        self.task.as_str()
    }

    #[graphql(name = "done")]
    fn done(&self) -> bool {
        self.done
    }
}

// Used to create new TODOs
#[derive(Insertable)]
#[table_name = "todos"]
pub struct NewTodo<'a> {
    pub task: &'a str,
    pub done: &'a bool,
}

// The GraphQL input object for creating TODOs
#[derive(GraphQLInputObject)]
pub struct CreateTodoInput {
    pub task: String,
    pub done: Option<bool>,
}
