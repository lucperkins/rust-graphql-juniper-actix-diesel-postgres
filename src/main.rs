extern crate actix_rt;
extern crate actix_web;
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate juniper;
extern crate r2d2;
extern crate todos;

use std::{env, io};

use actix_web::{middleware, App, HttpServer};

use todos::db::get_pool;
use todos::endpoints::graphql_endpoints;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    logging_setup();

    // Instantiate a new connection pool
    let pool = get_pool();

    // Start up the server, passing in (a) the connection pool
    // to make it available to all endpoints and (b) the configuration
    // function that adds the /graphql logic.
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(graphql_endpoints)
    })
    .bind("127.0.0.1:4000")?
    .run()
    .await
}

// TODO: more fine-grained logging setup
fn logging_setup() {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
}
