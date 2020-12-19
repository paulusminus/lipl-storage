use std::io::{Read};

use crate::model;
use model::{LiplResult, PlaylistPost};

pub fn get_playlist<R: Read>(reader: R) -> LiplResult<PlaylistPost> {
    Ok(serde_yaml::from_reader::<R, PlaylistPost>(reader)?)
}

