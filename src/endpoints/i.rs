use crate::var::CFG;
use axum::Json;

#[derive(serde::Serialize)]
pub(super) struct Response {
    authorized: bool,
    compress_webp: bool,
}

pub(super) async fn infomation() -> Json<Response> {
    let authorized = loop {
        {
            let config = CFG.read().await;
            if !config.token.is_empty() {
                break config.token.as_str() != "default";
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    };

    Json(Response {
        authorized,
        compress_webp: true,
    })
}
