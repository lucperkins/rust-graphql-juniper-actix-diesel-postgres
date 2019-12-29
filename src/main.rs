extern crate actix_rt;
extern crate actix_web;
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate juniper;
extern crate r2d2;
extern crate todos;

use std::{env, io};
use std::sync::Arc;

use actix::SyncArbiter;
use actix_web::{middleware, App, HttpServer};

use todos::context::GraphQLContext;
use todos::db::DbExecutor;
use todos::endpoints::graphql_endpoints;
use todos::exec::{GraphQLExecutor, State};
use todos::graphql::create_schema;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    logging_setup();

    let db_addr = SyncArbiter::start(3, move || DbExecutor::new());
    let schema_ctx = GraphQLContext::new(db_addr);
    let schema = Arc::new(create_schema());
    let schema_addr = SyncArbiter::start(3, move || GraphQLExecutor::new(schema.clone(), schema_ctx.clone()));

    HttpServer::new(move || {
        App::new()
            .data(State::new(schema_addr))
            .wrap(middleware::Logger::default())
            .configure(graphql_endpoints)
    })
    .bind("127.0.0.1:4000")?
    .run()
    .await
}

fn logging_setup() {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
}
