mod schema;
mod startup;
mod types;
mod storage;
mod graph_api;

use crate::schema::create_schema;
use crate::startup::{graphiql, graphql_handler};
use actix_web::{guard, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = create_schema();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(
                web::resource("/playground")
                    .guard(guard::Get())
                    .to(graphiql),
            )
            .service(web::resource("/").guard(guard::Post()).to(graphql_handler))
    })
    .bind("0.0.0.0:9001")?
    .run()
    .await
}
