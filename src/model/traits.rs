use crate::model::{Summary, Uuid};

pub trait HasId {
    fn id(&self) -> Uuid;
}

pub trait HasSummary {
    fn to_summary(&self) -> Summary;
}
