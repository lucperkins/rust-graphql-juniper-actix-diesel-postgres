use super::context::GraphQLContext;
use super::db::PostgresPool;
use super::graphql::create_schema;
use super::graphql::Schema;
use actix_web::{web, Error, HttpResponse};
use juniper::http::playground::playground_source;
use juniper::http::GraphQLRequest;
use std::sync::Arc;

pub fn graphql_endpoints(config: &mut web::ServiceConfig) {
    let schema = Arc::new(create_schema());
    config
        .data(schema)
        .route("/graphql", web::post().to(graphql))
        .route("/graphql", web::get().to(graphql_playground));
}

async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source("/graphql"))
}

async fn graphql(
    pool: web::Data<PostgresPool>,
    schema: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = GraphQLContext {
        pool: pool.get_ref().to_owned(),
    };
    let res = web::block(move || {
        let res = data.execute(&schema, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await
    .map_err(Error::from)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(res))
}
