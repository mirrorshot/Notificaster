use crate::types::{Notification, OrganizationID};
use dotenv::dotenv;
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, extjson::de::Error};
use mongodb::{Client, Collection, Cursor};
use std::env;
pub struct Storage {
    notifications: Collection<Notification>,
}

impl Storage {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGODB_URI") {
            Ok(v) => v.to_string(),
            Err(e) => format!("Error loading env variable MONGODB_URI: {:?}", e),
        };
        let client = Client::with_uri_str(&uri);
        let db = client.await.unwrap().database("notificaster");

        Storage {
            notifications:db.collection("notifications"),
        }
    }
    pub async fn get_notifications(&self, org: OrganizationID) -> Result<Vec<Notification>, Error> {
        let filter = doc! {"organisation": String::from(org)};
        let cursor:Cursor<Notification> = self.notifications
            .find(filter)
            .await
            .expect("failed to read from mongodb");
        Ok(cursor.try_collect().await.expect("failed to read from mongodb"))
    }
}