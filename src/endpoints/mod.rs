mod auth;
mod friends;
mod i;

use self::friends::friends;
use crate::endpoints::auth::auth;
use crate::endpoints::i::infomation;
use axum::{
    routing::{get, post},
    Router,
};
use hyper::{header::CONTENT_TYPE, Method};
use include_dir::{include_dir, Dir};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_serve_static::ServeDir;

pub async fn launch() {
    static ASSETS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/frontend/dist");
    let service = ServeDir::new(&ASSETS_DIR);

    let app = Router::new()
        .route("/i", get(infomation))
        .route("/auth", post(auth))
        .route("/friends", get(friends))
        .nest_service("/", service)
        .layer(
            CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([CONTENT_TYPE]),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
