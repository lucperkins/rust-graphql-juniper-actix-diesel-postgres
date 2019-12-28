use super::models::{CreateTodoInput, NewTodo, Todo};
use super::schema::todos::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use juniper::{graphql_value, FieldError, FieldResult};

pub struct Todos;

impl Todos {
    pub fn all_todos(conn: &PgConnection) -> FieldResult<Vec<Todo>> {
        let res = todos.load::<Todo>(conn);

        graphql_translate(res)
    }

    pub fn create_todo(
        conn: &PgConnection,
        new_todo: CreateTodoInput,
    ) -> FieldResult<Todo> {
        use super::schema::todos;

        let new_todo = NewTodo {
            task: &new_todo.task,
            done: &new_todo.done.unwrap_or(false),
        };

        let res = diesel::insert_into(todos::table)
            .values(&new_todo)
            .get_result(conn);

        graphql_translate(res)
    }

    pub fn get_todo_by_id(conn: &PgConnection, todo_id: i32) -> FieldResult<Option<Todo>> {
        match todos.find(todo_id).get_result::<Todo>(conn) {
            Ok(todo) => Ok(Some(todo)),
            Err(e) => match e {
                diesel::result::Error::NotFound => FieldResult::Ok(None),
                _                               => FieldResult::Err(FieldError::from(e))
            }
        }
    }

    pub fn mark_todo_as_done(conn: &PgConnection, todo_id: i32) -> FieldResult<Todo> {
        mark_todo_as(conn, todo_id, true)
    }

    pub fn mark_todo_as_not_done(conn: &PgConnection, todo_id: i32) -> FieldResult<Todo> {
        mark_todo_as(conn, todo_id, false)
    }
}

fn graphql_translate<T>(res: Result<T, diesel::result::Error>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => FieldResult::Err(FieldError::from(e)),
    }
}

fn mark_todo_as(conn: &PgConnection, todo_id: i32, is_done: bool) -> FieldResult<Todo> {
    let res = todos.find(todo_id).get_result::<Todo>(conn);
    
    let msg = if is_done { "done" } else { "not done" };

    match res {
        Ok(todo) => {
            if todo.done == is_done {
                let err = FieldError::new(
                    format!("TODO already marked as {}", msg),
                    graphql_value!({ "foo": "bar "})
                );
                FieldResult::Err(err)
            } else {
                let res = diesel::update(todos.find(todo_id))
                    .set(done.eq(is_done))
                    .get_result::<Todo>(conn);
                graphql_translate(res)
            }
        }
        Err(e) => FieldResult::Err(FieldError::from(e)),
    }
}
