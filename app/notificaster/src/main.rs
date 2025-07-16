mod graph_api;
mod schema;
mod startup;
mod storage;
mod types;

use crate::schema::create_schema;
use crate::startup::{graphiql, graphql_handler, PLAYGROUND_PATH, SERVICE_PATH};
use actix_web::{guard, web, App, HttpResponse, HttpServer};
use std::env;

/**
* Used to serve health status.
*
* To be improved checking db connectivity status and other services.
*/
async fn check_health_status() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("APP_PORT").unwrap_or_else(|_| "9001".to_string());
    println!("Starting server on port {}", port);
    let schema = create_schema();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(
                web::resource(PLAYGROUND_PATH)
                    .guard(guard::Get())
                    .to(graphiql),
            )
            .service(
                web::resource(SERVICE_PATH)
                    .guard(guard::Post())
                    .to(graphql_handler),
            )
            .route("/health", web::get().to(check_health_status))
    })
    .bind(("0.0.0.0", port.parse().unwrap()))?
    .run()
    .await
}
