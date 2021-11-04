use crate::model::{Summary, Uuid, LiplResult};

pub trait HasId {
    fn id(&self) -> Uuid;
}

pub trait HasSummary {
    fn to_summary(&self) -> Summary;
}

pub trait HasSummaries {
    fn to_summaries(&self) -> Vec<Summary>;
}

pub trait HasExtension {
    fn has_extension(&self, ext: &str) -> bool;
}

pub trait ToDiskFormat {
    fn to_disk_format(&self) -> LiplResult<String>;
}

pub trait TryFromDiskFormat<T>: Sized {
    fn from_disk_format(value: T) -> LiplResult<Self>;
}

pub trait ExtractUuid {
    fn extract_uuid(&self) -> LiplResult<Uuid>;
}