use std::io::{Read};

use crate::model;
use model::{LiplResult, PlaylistPost};

pub fn get_playlist<R>(reader: R) -> LiplResult<PlaylistPost> 
where R: Read
{
    Ok(serde_yaml::from_reader::<R, PlaylistPost>(reader)?)
}

