use lipl_axum::{create_service};
use axum::{
    body::{Body},
    http::{Request, StatusCode},
};
use serde_json::{Value, json};
use tower::{ServiceExt};

#[tokio::test]
async fn list_playlist() {
    let app = create_service().await.unwrap();

    let response =
        app
        .oneshot(
            Request::builder().uri("/api/v1/playlist").body(Body::empty()).unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, json!([ { "id": "K16oCoxhVyHxMHkaqGnJZG", "title": "Diversen" }]));
}

#[tokio::test]
async fn list_lyrics() {
    let app = create_service().await.unwrap();

    let response =
        app
        .oneshot(
            Request::builder().uri("/api/v1/lyric").body(Body::empty()).unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        body,
        json!(
            [ 
                { "id": "CJDcu6VvwdocmjTKEGHB1B", "title": "Roodkapje" },
                { "id": "K6qY8z7BmNnuH1ALBhNHZa", "title": "Sinterklaas" },
            ]
        )
    );
}

