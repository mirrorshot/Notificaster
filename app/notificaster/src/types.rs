use async_graphql::{OutputType, SimpleObject};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct Notification {
    timestamp: DateTime<Local>,
    organization: OrganizationID,
    device: DeviceId,
    manufacturer: Manufacturer,
}

pub type OrganizationID = Uuid;
pub type Manufacturer = String;
pub type DeviceId = String;

