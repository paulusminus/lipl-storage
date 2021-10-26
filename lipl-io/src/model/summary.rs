use serde::{Deserialize, Serialize};
use crate::model::{Uuid};

#[derive(Deserialize, Serialize)]
pub struct Summary {
    pub id: Uuid,
    pub title: Option<String>,
}
