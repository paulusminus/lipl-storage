use uuid::Uuid;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

use crate::model::{LiplError, UuidExt};

struct Id(Uuid);

impl Id {
    fn new(uuid: Uuid) -> Self {
        Id(uuid)
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0.to_base58())
    }
}

impl FromStr for Id {
    type Err = LiplError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::try_from_base58(s)?;
        let id = Id(uuid);
        Ok(id)
    }
}