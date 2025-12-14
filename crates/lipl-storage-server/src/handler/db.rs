use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::Response};
use lipl_core::{Repo, RepoDb};

use super::{to_error_response, to_json_response};

/// Handler for getting the database
pub async fn get<R: Repo>(State(connection): State<Arc<R>>) -> Response {
    let result = async move {
        let lyrics = connection.get_lyrics().await?;
        let playlists = connection.get_playlists().await?;
        Ok(RepoDb { lyrics, playlists })
    }
    .await;

    match result {
        Ok(db) => to_json_response(StatusCode::OK)(db),
        Err(err) => to_error_response(err),
    }
}

/// Handler for replacing the database
pub async fn put<R: Repo>(State(connection): State<Arc<R>>, Json(db): Json<RepoDb>) -> Response {
    let result = async move {
        for playlist in connection.get_playlists().await? {
            connection.delete_playlist(playlist.id).await?
        }
        for lyric in connection.get_lyrics().await? {
            connection.delete_lyric(lyric.id).await?;
        }
        for lyric in db.lyrics {
            connection.upsert_lyric(lyric).await?;
        }
        for playlist in db.playlists {
            connection.upsert_playlist(playlist).await?;
        }
        Ok(())
    }
    .await;
    match result {
        Ok(db) => to_json_response(StatusCode::OK)(db),
        Err(err) => to_error_response(err),
    }
}
