use serde::{Deserialize, Serialize};
use crate::model::{serde_uuid, Uuid};

#[derive(Deserialize, Serialize)]
pub struct Summary {
    #[serde(with = "serde_uuid")]
    pub id: Uuid,
    pub title: Option<String>,
}
