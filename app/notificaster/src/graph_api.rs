use async_graphql::Object;
use crate::storage::Storage;
use crate::types::{Notification, OrganizationID};

pub struct Query;

#[Object]
impl Query {
    async fn notifications(&self) -> Vec<Notification> {
        let storage = Storage::init().await;
        storage.get_notifications(OrganizationID::new_v4()).await
            .expect("failed reading notifications")
    }
}
