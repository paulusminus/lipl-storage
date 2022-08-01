use std::fmt::{Display};
use lipl_fs_repo::{FileSystem};
use lipl_types::{RepoResult, request::{send, Request}};
use lipl_fs_repo::elapsed::{Elapsed};

pub fn print<D>(d: D) 
where 
    D: Display
{
    println!("{}", d);
}

pub async fn process() -> RepoResult<()> {
    let (mut tx, _) = FileSystem::new(
        "./data/".to_owned(),
        "yaml".to_owned(),
        "txt".to_owned(),
    )?;

    println!("Lyrics");
    send(&mut tx, Request::LyricSummaries).await?
    .into_iter()
    .for_each(print);


    println!();

    println!("Playlists");
    send(&mut tx, Request::PlaylistSummaries).await?
    .into_iter()
    .for_each(print);

    Ok(())
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> RepoResult<()>{
    println!("Elapsed: {} milliseconds", process.elapsed().await?);
    Ok(())
}
