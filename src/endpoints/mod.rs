mod friends;

use self::friends::friends;
use axum::{routing::get, Router};
use hyper::{header::CONTENT_TYPE, Method};
use include_dir::{include_dir, Dir};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_serve_static::ServeDir;

pub async fn launch() {
    static ASSETS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/frontend/dist");
    let service = ServeDir::new(&ASSETS_DIR);

    let app = Router::new()
        .route("/friends", get(friends))
        .nest_service("/", service)
        .layer(
            CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_methods([Method::GET])
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
