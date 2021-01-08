use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct Query {
    pub full: bool
}
