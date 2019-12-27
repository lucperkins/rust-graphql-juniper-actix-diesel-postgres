use super::context::Context as GraphQLContext;
use diesel::pg::PgConnection;
use juniper::{FieldResult, RootNode};

pub struct Query;

#[juniper::object(Context = GraphQLContext)]
impl Query {}

pub struct Mutation;

#[juniper::object(Context = GraphQLContext)]
impl Mutation {}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}
