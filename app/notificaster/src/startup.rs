use actix_web::{Result, HttpResponse, web};
use async_graphql::http::GraphiQLSource;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use log::info;
use crate::schema::GraphNotificationSchema;

pub async fn graphiql() ->Result<HttpResponse>{
    info!("loading graphiql");
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("/")
                .subscription_endpoint("/ws")
                .finish()
        )
    )
}

pub async fn graphql_handler(
    schema: web::Data<GraphNotificationSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
