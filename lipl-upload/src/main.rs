mod api;
mod args;
mod client;
mod error;
mod fs;
mod model;

use crate::api::Api;
use crate::error::UploadError;
use clap::Parser;
use futures::{Future, TryStreamExt, TryFutureExt};
use lipl_types::{Summary, Uuid, PlaylistPost};
use std::time::Instant;
use crate::model::{try_iter};

pub type UploadResult<T> = std::result::Result<T, UploadError>;

async fn delete_collection<G, H, I>(i: I, g: G) -> UploadResult<()> 
where 
    G: Fn(Summary) -> H, 
    H: Future<Output=UploadResult<()>>,
    I: Future<Output=UploadResult<Vec<Summary>>>,
{
    i
    .and_then(|summaries| {
        try_iter(summaries)
        .and_then(g)
        .try_collect()
    })
    .await
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    let args = args::Args::parse();

    let client = crate::client::UploadClient::new(args.prefix);

    delete_collection(
        client.playlist_summaries(),
        |s| client.playlist_delete(s.id),
    ).await?;
    println!("All playlists deleted");

    delete_collection(
        client.lyric_summaries(),
        |s| client.lyric_delete(s.id),
    ).await?;
    println!("All lyrics deleted");

    let ids = 
        fs::post_lyrics(
            args.source_path,
            fs::extension_filter("txt"),
            &client,
        )
        .await?
        .try_collect::<Vec<Uuid>>()
        .await?;

    ids.iter().for_each(
        |id| println!("Lyric posted with id {}", id)
    );

    let playlist_post = PlaylistPost {
        title: args.playlist_name,
        members: ids,
    };
    let playlist = client.playlist_insert(playlist_post).await?;
    println!("Playlist posted with id {}, title {}", playlist.id, playlist.title);

    println!("Elapsed: {} milliseconds", now.elapsed().as_millis());
    Ok(())
}
