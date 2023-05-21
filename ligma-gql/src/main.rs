use actix_cors::Cors;
use actix_web::{get, middleware::Logger, post, web, App, HttpServer, Responder};
use actix_web_lab::respond::Html;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use location::{LocationQuery, LocationSchema};

#[post("/gql")]
async fn graphql(schema: web::Data<LocationSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[get("/gql")]
async fn graphql_playground() -> impl Responder {
    Html(playground_source(GraphQLPlaygroundConfig::new("/gql")))
}

mod location;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(LocationQuery, EmptyMutation, EmptySubscription).finish();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(graphql)
            .service(graphql_playground)
            .wrap(Cors::permissive())
            .wrap(Logger::default())
    })
    .workers(1)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
