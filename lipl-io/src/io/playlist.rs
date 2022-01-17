use std::io::{Read};

use lipl_types::{PlaylistPost, RepoError, RepoResult};

pub fn playlistpost_from_reader<R>(reader: R) -> RepoResult<PlaylistPost> 
where R: Read
{
    serde_yaml::from_reader::<R, PlaylistPost>(reader)
    .map_err(RepoError::from)
}

