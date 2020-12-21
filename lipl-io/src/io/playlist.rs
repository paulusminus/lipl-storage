use std::io::{Read};

use crate::model;
use model::{LiplError, LiplResult, PlaylistPost};

pub fn playlistpost_from_reader<R>(reader: R) -> LiplResult<PlaylistPost> 
where R: Read
{
    serde_yaml::from_reader::<R, PlaylistPost>(reader)
    .map_err(LiplError::from)
}

