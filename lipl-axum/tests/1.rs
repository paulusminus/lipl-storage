use lipl_axum::{create_service};
use lipl_core::{Lyric, LyricPost, Summary, Playlist, PlaylistPost, Uuid};
use axum::{
    body::{Body},
    http::{Request, StatusCode}, Router,
};
use serde::{Serialize, de::DeserializeOwned};
use tower::{ServiceExt};

fn daar_bij_die_molen() -> LyricPost {
    LyricPost {
        title: "Daar bij die molen".to_owned(),
        parts: vec![
            vec![
                "Daar bij die molen, die mooie molen".to_owned(),
                "Daar woont het meiseje waar ik zo veel van hou".to_owned(),
                "Daar bij die molen, die mooie molen".to_owned(),
                "Daar bij die molen, die mooie molen".to_owned(),
            ]
        ],
    }
}

#[tokio::test(flavor = "current_thread")]
async fn playlist_list() {
    let service = create_service().await.unwrap();

    let playlists: Vec<Summary> = list(&service, "playlist".to_owned()).await; 
    assert_eq!(
        playlists[0].title,
        "Diversen".to_owned()
    );
}

#[tokio::test(flavor = "current_thread")]
async fn lyric_list() {
    let service = create_service().await.unwrap();

    let lyrics: Vec<Summary> = list(&service, "lyric".to_owned()).await;
    assert_eq!(
        lyrics[0].title,
        "Roodkapje".to_owned(),
    );
    assert_eq!(
        lyrics[1].title,
        "Sinterklaas".to_owned()
    );
}

#[tokio::test(flavor = "current_thread")]
async fn lyric_post() {
    let service = create_service().await.unwrap();

    let lyric_post = LyricPost {
        title: "Er is er één jarig".to_owned(),
        parts: vec![],
    };

    let lyric: Lyric = post(&service, "lyric".to_owned(), &lyric_post).await;
    let id = lyric.id.to_string();
    assert_eq!(lyric.title, lyric_post.title);
    assert_eq!(lyric.parts, lyric_post.parts);

    delete(&service, "lyric".to_owned(), id).await;
}

#[tokio::test(flavor = "current_thread")]
async fn lyric_post_change() {
    let service = create_service().await.unwrap();

    let mut lyric_post = daar_bij_die_molen();

    let lyric: Lyric = post(&service, "lyric".to_owned(), &lyric_post).await;
    let id = lyric.id.to_string();
    assert_eq!(lyric.title, lyric_post.title);
    assert_eq!(lyric.parts, lyric_post.parts);

    lyric_post.title = "Daar bij dat molengedrag".to_owned();
    let lyric_changed: Lyric = put(&service, "lyric".to_owned(), id.clone(), &lyric_post).await;
    
    assert_eq!(lyric_changed.title, lyric_post.title);
    delete(&service, "lyric".to_owned(), id).await;
}

#[tokio::test(flavor = "current_thread")]
async fn playlist_post() {
    let service = create_service().await.unwrap();

    let playlist_post = PlaylistPost {
        title: "Alle 13 goed".to_owned(),
        members: vec![],
    };

    let playlist: Playlist = post(&service, "playlist".to_owned(), &playlist_post).await;
    let id = playlist.id.to_string();
    assert_eq!(playlist.title, "Alle 13 goed".to_owned());

    let members: Vec<Uuid> = vec![];
    assert_eq!(playlist.members, members);

    delete(&service, "playlist".to_owned(), id).await;
}

async fn list<R: DeserializeOwned>(service: &Router<()>, name: String) -> Vec<R> {
    let response = service
        .clone()
        .oneshot(
            Request::get(format!("/api/v1/{name}"))
            .body(Body::empty())
            .unwrap()
        )
        .await
        .unwrap();

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let r: Vec<R> = serde_json::from_slice(&body).unwrap();
    r
}

async fn delete(service: &Router<()>, name: String, id: String) {
    let response = service
    .clone()
    .oneshot(
        Request::delete(format!("/api/v1/{name}/{id}"))
        .body(Body::empty())
        .unwrap()
    )
    .await
    .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

async fn post<T: Serialize, R: DeserializeOwned>(service: &Router<()>, name: String, t: &T) -> R {
    let body = serde_json::to_string(t).unwrap();
    let response =
        service
        .clone()
        .oneshot(
            Request::post(format!("/api/v1/{name}"))
            .header("Content-Type", "application/json")
            .body(body.into())
            .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let r: R = serde_json::from_slice(&body).unwrap();
    r
}

async fn put<T: Serialize, R: DeserializeOwned>(service: &Router<()>, name: String, id: String, t: &T) -> R {
    let body = serde_json::to_string(t).unwrap();
    let response =
        service
        .clone()
        .oneshot(
            Request::put(format!("/api/v1/{name}/{id}"))
            .header("Content-Type", "application/json")
            .body(body.into())
            .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let r: R = serde_json::from_slice(&body).unwrap();
    r
}