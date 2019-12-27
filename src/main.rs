extern crate actix_rt;
extern crate actix_web;
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate juniper;
extern crate litany;
extern crate r2d2;

use std::{env, io};

use actix_web::{middleware, App, HttpServer};

use litany::db::get_pool;
use litany::endpoints::graphql_endpoints;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    logging_setup();

    let pool = get_pool();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(graphql_endpoints)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn logging_setup() {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
}
