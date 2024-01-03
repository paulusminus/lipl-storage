use std::vec;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use http_body_util::BodyExt;
use lipl_core::{Lyric, LyricPost, Playlist, PlaylistPost, Summary, Uuid};
use lipl_storage_server::{create_router, environment::RepoType};
use serde::{de::DeserializeOwned, Serialize};
use tower::ServiceExt;

const LYRIC: &str = "lyric";
const PLAYLIST: &str = "playlist";
const PREFIX: &str = "/lipl/api/v1/";

async fn router() -> Router {
    create_router(RepoType::Memory(false)).await.unwrap()
}


fn daar_bij_die_molen() -> LyricPost {
    LyricPost {
        title: "Daar bij die molen".to_owned(),
        parts: vec![vec![
            "Daar bij die molen, die mooie molen".to_owned(),
            "Daar woont het meiseje waar ik zo veel van hou".to_owned(),
            "Daar bij die molen, die mooie molen".to_owned(),
            "Daar bij die molen, die mooie molen".to_owned(),
        ]],
    }
}

fn roodkapje() -> LyricPost {
    LyricPost {
        title: "Roodkapje".to_owned(),
        parts: vec![
            vec![
                "Zeg roodkapje waar ga je hene, zo alleen, zo alleen".to_owned(),
                "Zeg roodkapje waar ga je hene, zo alleen".to_owned(),
            ],
            vec![
                "'k ga naar grootmoeder koekjes brengen in het bos, in het bos".to_owned(),
                "'k ga naar grootmoeder koekjes brengen in het bos".to_owned(),
            ],
        ],
    }
}

#[tokio::test(flavor = "current_thread")]
async fn lyric_list() {
    let service = router().await;

    let _daar_bij_die_molen: Lyric = post(&service, LYRIC, &daar_bij_die_molen()).await;
    let _roodkapje: Lyric = post(&service, LYRIC, &roodkapje()).await;

    let lyrics: Vec<Summary> = list(&service, LYRIC).await;
    assert_eq!(lyrics[0].title, "Daar bij die molen".to_owned(),);
    assert_eq!(lyrics[1].title, "Roodkapje".to_owned());
}

#[tokio::test(flavor = "current_thread")]
async fn lyric_post() {
    let service = router().await;

    let lyric_post = LyricPost {
        title: "Er is er één jarig".to_owned(),
        parts: vec![],
    };

    let lyric: Lyric = post(&service, LYRIC, &lyric_post).await;
    assert_eq!(lyric.title, lyric_post.title);
    assert_eq!(lyric.parts, lyric_post.parts);
}

#[tokio::test(flavor = "current_thread")]
async fn lyric_post_change() {
    let service = router().await;

    let mut lyric_post = daar_bij_die_molen();

    let lyric: Lyric = post(&service, LYRIC, &lyric_post).await;
    let id = lyric.id.to_string();
    assert_eq!(lyric.title, lyric_post.title);
    assert_eq!(lyric.parts, lyric_post.parts);

    lyric_post.title = "Daar bij dat molengedrag".to_owned();
    let lyric_changed: Lyric = put(&service, LYRIC, id.clone(), &lyric_post).await;

    assert_eq!(lyric_changed.title, lyric_post.title);
}

#[tokio::test(flavor = "current_thread")]
async fn lyric_delete() {
    let service = router().await;

    let list_before_post: Vec<Summary> = list(&service, LYRIC).await;
    assert_eq!(list_before_post.len(), 0);

    let roodkapje: Lyric = post(&service, LYRIC, &roodkapje()).await;
    let daar_bij_die_molen: Lyric = post(&service, LYRIC, &daar_bij_die_molen()).await;
    let list_after_post: Vec<Summary> = list(&service, LYRIC).await;
    assert_eq!(list_after_post.len(), 2);

    delete(&service, LYRIC, roodkapje.id.to_string()).await;
    delete(&service, LYRIC, daar_bij_die_molen.id.to_string()).await;

    let list_after_delete: Vec<Summary> = list(&service, LYRIC).await;
    assert_eq!(list_after_delete.len(), 0);
}

#[tokio::test(flavor = "current_thread")]
async fn playlist_list() {
    let service = router().await;

    let playlist_post = PlaylistPost {
        title: "Alle 13 goed".to_owned(),
        members: vec![],
    };

    let _playlist: Playlist = post(&service, PLAYLIST, &playlist_post).await;

    let playlists: Vec<Summary> = list(&service, PLAYLIST).await;
    assert_eq!(playlists[0].title, "Alle 13 goed".to_owned());
}

#[tokio::test(flavor = "current_thread")]
async fn playlist_post() {
    let service = router().await;

    let playlist_post = PlaylistPost {
        title: "Alle 13 goed".to_owned(),
        members: vec![],
    };

    let playlist: Playlist = post(&service, PLAYLIST, &playlist_post).await;
    assert_eq!(playlist.title, "Alle 13 goed".to_owned());

    let members: Vec<Uuid> = vec![];
    assert_eq!(playlist.members, members);
}

#[tokio::test(flavor = "current_thread")]
async fn playlist_post_lyric_delete() {
    let service = router().await;

    let roodkapje: Lyric = post(&service, LYRIC, &roodkapje()).await;
    let daar_bij_die_molen: Lyric = post(&service, LYRIC, &daar_bij_die_molen()).await;

    let playlist_post = PlaylistPost {
        title: "Alle 13 goed".to_owned(),
        members: vec![roodkapje.id, daar_bij_die_molen.id],
    };

    let playlist: Playlist = post(&service, PLAYLIST, &playlist_post).await;
    assert_eq!(playlist.title, "Alle 13 goed".to_owned());
    assert_eq!(playlist.members, vec![roodkapje.id, daar_bij_die_molen.id]);

    delete(&service, LYRIC, roodkapje.id.to_string()).await;
    let playlist: Playlist = item(&service, PLAYLIST, playlist.id.to_string()).await;
    assert_eq!(playlist.members, vec![daar_bij_die_molen.id]);
}

async fn list<R: DeserializeOwned>(service: &Router<()>, name: &'static str) -> Vec<R> {
    let response = service
        .clone()
        .oneshot(
            Request::get(format!("{PREFIX}{name}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let b = response.into_body().collect().await.unwrap().to_bytes();
    let r: Vec<R> = serde_json::from_slice(&b).unwrap();
    r
}

async fn item<R: DeserializeOwned>(service: &Router<()>, name: &'static str, uuid: String) -> R {
    let response = service
        .clone()
        .oneshot(
            Request::get(format!("{PREFIX}{name}/{uuid}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let b = response.into_body().collect().await.unwrap().to_bytes();
    let r: R = serde_json::from_slice(&b).unwrap();
    r
}

async fn delete(service: &Router<()>, name: &'static str, id: String) {
    let response = service
        .clone()
        .oneshot(
            Request::delete(format!("{PREFIX}{name}/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

async fn post<'a, T: Serialize, R: DeserializeOwned>(
    service: &'a Router<()>,
    name: &str,
    t: &T,
) -> R {
    let body = serde_json::to_string(t).unwrap();
    let response = service
        .clone()
        .oneshot(
            Request::post(format!("{}{}", PREFIX, name.to_string()))
                .header("Content-Type", "application/json")
                .body(body)
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let b = response.into_body().collect().await.unwrap().to_bytes();
    let r: R = serde_json::from_slice(&b).unwrap();
    r
}

async fn put<'a, T: Serialize, R: DeserializeOwned>(
    service: &'a Router<()>,
    name: &str,
    id: String,
    t: &T,
) -> R {
    let body = serde_json::to_string(t).unwrap();
    let response = service
        .clone()
        .oneshot(
            Request::put(format!("{PREFIX}{name}/{id}"))
                .header("Content-Type", "application/json")
                .body(body)
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let b = response.into_body().collect().await.unwrap().to_bytes();
    let r: R = serde_json::from_slice(&b).unwrap();
    r
}
