use std::io::{Read};

use lipl_core::{PlaylistPost};

pub fn playlistpost_from_reader<R>(reader: R) -> lipl_core::Result<PlaylistPost> 
where R: Read
{
    serde_yaml::from_reader::<R, PlaylistPost>(reader)
    .map_err(Into::into)
}
