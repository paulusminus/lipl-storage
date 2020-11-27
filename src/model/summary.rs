use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::serde_uuid;

#[derive(Deserialize, Serialize)]
pub struct Summary {
    #[serde(with = "serde_uuid")]
    pub id: Uuid,
    pub title: Option<String>,
}
