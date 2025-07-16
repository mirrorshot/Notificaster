use actix_web::{Result, HttpResponse, web};
use async_graphql::http::GraphiQLSource;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use log::info;
use crate::schema::GraphNotificationSchema;

pub const SERVICE_PATH: &str = "/v1/graphql";
pub const PLAYGROUND_PATH: &str = "/v1/playground";

pub async fn graphiql() ->Result<HttpResponse>{
    info!("loading graphiql");
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint(SERVICE_PATH)
                .subscription_endpoint(format!("{:?}/subscription", SERVICE_PATH).as_str())
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
