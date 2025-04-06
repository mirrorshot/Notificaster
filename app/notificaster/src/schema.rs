use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use crate::graph_api::Query;

pub type GraphNotificationSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> GraphNotificationSchema {
    Schema::build(Query, EmptyMutation, EmptySubscription).finish()
}
