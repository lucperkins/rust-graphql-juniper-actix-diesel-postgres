use actix::{Actor, Addr, Handler, Message, SyncContext};
use actix_web::Error;
use juniper::http::GraphQLRequest;
use std::sync::Arc;
use super::context::GraphQLContext;
use super::graphql::Schema;

#[derive(Deserialize, Serialize)]
pub struct GraphQLData(GraphQLRequest);

impl Message for GraphQLData {
    type Result = Result<String, Error>;
}

pub struct GraphQLExecutor {
    schema: Arc<Schema>,
    context: GraphQLContext,
}

impl GraphQLExecutor {
    pub fn new(schema: Arc<Schema>, context: GraphQLContext) -> Self {
        Self {
            schema: schema,
            context: context,
        }
    }
}

impl Actor for GraphQLExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<GraphQLData> for GraphQLExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: GraphQLData, _: &mut Self::Context) -> Self::Result {
        let res = msg.0.execute(&self.schema, &self.context);
        let res_str = serde_json::to_string(&res)?;
        Ok(res_str)
    }
}

pub struct State {
    addr: Addr<GraphQLExecutor>,
}

impl State {
    pub fn new(addr: Addr<GraphQLExecutor>) -> Self {
        Self {
            addr: addr,
        }
    }
}
