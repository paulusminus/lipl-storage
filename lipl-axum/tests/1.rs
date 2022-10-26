use lipl_axum::{create_service};
use lipl_types::{Lyric, LyricPost, Summary, Playlist, PlaylistPost, Uuid};
use axum::{
    body::{Body},
    http::{Request, StatusCode},
};
use tower::{ServiceExt};

#[tokio::test]
async fn playlist_list() {
    let app = create_service().await.unwrap();

    let response =
        app
        .oneshot(
            Request::get("/api/v1/playlist").body(Body::empty()).unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let playlists: Vec<Summary> = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        playlists.get(0).unwrap().title,
        "Diversen".to_owned()
    );
}

#[tokio::test]
async fn lyric_list() {
    let app = create_service().await.unwrap();

    let response =
        app
        .oneshot(
            Request::get("/api/v1/lyric")
            .body(Body::empty())
            .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: Vec<Summary> = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        body.get(0).unwrap().title,
        "Roodkapje".to_owned(),
    );
    assert_eq!(
        body.get(1).unwrap().title,
        "Sinterklaas".to_owned()
    );
}

#[tokio::test]
async fn lyric_post() {
    let app = create_service().await.unwrap();

    let lyric_post = LyricPost {
        title: "Er is er één jarig".to_owned(),
        parts: vec![],
    };
    let body = serde_json::to_string(&lyric_post).unwrap();
    let response =
        app
        .clone()
        .oneshot(
            Request::post("/api/v1/lyric")
            .header("Content-Type", "application/json")
            .body(body.into())
            .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: Lyric = serde_json::from_slice(&body).unwrap();
    let id = body.id.to_string();
    assert_eq!(body.title, "Er is er één jarig".to_owned());

    let result: Vec<Vec<String>> = vec![];
    assert_eq!(body.parts, result);

    let response_delete =
        app
        .clone()
        .oneshot(
            Request::delete(format!("/api/v1/lyric/{}", id))
            .body(Body::empty())
            .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response_delete.status(), StatusCode::OK);
}

#[tokio::test]
async fn lyric_post_change() {
    let app = create_service().await.unwrap();

    let lyric_post = LyricPost {
        title: "Daar bij die molen".to_owned(),
        parts: vec![
            vec![
                "Daar bij die molen, die mooie molen".to_owned(),
                "Daar woont het meiseje waar ik zo veel van hou".to_owned(),
                "Daar bij die molen, die mooie molen".to_owned(),
                "Daar bij die molen, die mooie molen".to_owned(),
            ]
        ],
    };
    let body = serde_json::to_string(&lyric_post).unwrap();
    let response =
        app
        .clone()
        .oneshot(
            Request::post("/api/v1/lyric")
            .header("Content-Type", "application/json")
            .body(body.into())
            .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: Lyric = serde_json::from_slice(&body).unwrap();
    let id = body.id.to_string();
    assert_eq!(body.title, "Daar bij die molen".to_owned());
    assert_eq!(body.parts[0].len(), 4);

    let mut lyric_post_put: LyricPost = body.into();
    lyric_post_put.title = "Daar bij dat molengedrag".to_owned();
    let body_put = serde_json::to_string(&lyric_post_put).unwrap();
    let response_put =
        app
        .clone()
        .oneshot(
            Request::put(format!("/api/v1/lyric/{}", id))
            .header("Content-Type", "application/json")
            .body(body_put.into())
            .unwrap()
        )
        .await
        .unwrap();
    
    assert_eq!(response_put.status(), StatusCode::OK);
    let response_body_put = hyper::body::to_bytes(response_put.into_body()).await.unwrap();
    let response_body_put: Lyric = serde_json::from_slice(&response_body_put).unwrap();
    assert_eq!(response_body_put.title, "Daar bij dat molengedrag".to_owned());

    let response_delete =
        app
        .clone()
        .oneshot(
            Request::delete(format!("/api/v1/lyric/{}", id))
            .body(Body::empty())
            .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response_delete.status(), StatusCode::OK);
}

#[tokio::test]
async fn playlist_post() {
    let app = create_service().await.unwrap();

    let playlist_post = PlaylistPost {
        title: "Alle 13 goed".to_owned(),
        members: vec![],
    };
    let body = serde_json::to_string(&playlist_post).unwrap();
    let response =
        app
        .clone()
        .oneshot(
            Request::post("/api/v1/playlist")
            .header("Content-Type", "application/json")
            .body(body.into())
            .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let playlist: Playlist = serde_json::from_slice(&body).unwrap();
    let id = playlist.id.to_string();
    assert_eq!(playlist.title, "Alle 13 goed".to_owned());

    let members: Vec<Uuid> = vec![];
    assert_eq!(playlist.members, members);

    let response_delete =
        app
        .clone()
        .oneshot(
            Request::delete(format!("/api/v1/playlist/{}", id))
            .body(Body::empty())
            .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response_delete.status(), StatusCode::OK);
}

