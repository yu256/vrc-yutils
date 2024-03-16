use crate::{
    var::{APP_NAME, SELF_LOCATION, UA},
    xsoverlay::notify_join::{notify_join, JoinType},
};
use futures_util::StreamExt as _;
use hyper::Uri;
use std::{sync::OnceLock, time::Duration};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        self,
        client::IntoClientRequest as _,
        http::{HeaderValue, Request},
        Message,
    },
};

enum WSError {
    Disconnected,
    Token,
    Unknown(String),
    IoErr(tungstenite::error::Error),
    Other(String),
}

use WSError::*;

pub(crate) async fn process_websocket(auth: &str, uri: Option<&Uri>) {
    // インターネットに接続されていないときに無限に接続を試みてしまわないように
    let mut io_err_cnt = 0u8;

    loop {
        match connect_websocket(auth, uri).await {
            Disconnected => {
                io_err_cnt = 0;
            }
            Other(e) => {
                eprintln!("{e}");
                io_err_cnt = 0;
            }
            Unknown(e) => {
                eprintln!("Unknown Error: {e}");
                break;
            }
            Token => {
                eprintln!("トークンの有効期限が切れました。再認証を行ってください。");
                break;
            }
            IoErr(e) => {
                eprintln!("{e}");

                io_err_cnt += 1;

                match io_err_cnt {
                    1 => (),
                    20 => break,
                    _ => tokio::time::sleep(Duration::from_secs(10)).await,
                }
            }
        }
    }
    eprintln!("WebSocketから切断されました。");
}

async fn connect_websocket(auth: &str, uri: Option<&Uri>) -> WSError {
    static REQUEST: OnceLock<Request<()>> = OnceLock::new();
    let request = REQUEST
        .get_or_init(|| {
            let host = uri
                .and_then(|u| u.host())
                .unwrap_or("pipeline.vrchat.cloud");
            let mut req = format!("wss://{host}/?auth={auth}")
                .into_client_request()
                .unwrap();
            req.headers_mut()
                .insert(UA, HeaderValue::from_static(APP_NAME));
            req
        })
        .clone();

    let mut stream = match connect_async(request).await {
        Ok(stream) => stream.0,
        Err(e @ tungstenite::error::Error::Io(_)) => return IoErr(e),
        Err(e) => return Unknown(e.to_string()),
    };

    while let Some(message) = stream.next().await {
        use crate::vrc_structs::*;

        let message = match message {
            Ok(Message::Text(text)) if text.starts_with(r#"{"err"#) => {
                return if !text.contains("authToken") {
                    Unknown(text)
                } else {
                    Token
                };
            }
            Ok(Message::Text(text)) => text,
            Ok(Message::Close(_)) => return Disconnected,
            Err(e) => return Other(e.to_string()),
            _ => continue,
        };

        let StreamBody { r#type, content } = serde_json::from_str::<StreamBody>(&message).unwrap();

        match r#type.as_str() {
            "friend-location" => {
                let FriendLocation {
                    travelingToLocation,
                    user,
                    ..
                } = serde_json::from_str(&content).unwrap();
                if let Some(to) = &travelingToLocation {
                    notify_join(Some(to), &user.displayName, JoinType::PlayerJoining).await;
                }
            }

            "user-location" => {
                let user = serde_json::from_str::<FriendLocation>(&content).unwrap();
                *SELF_LOCATION.lock().await = user.location
            }

            "friend-add" | "friend-online" | "friend-offline" | "friend-delete"
            | "friend-active" | "notification" | "notification-v2" => {}

            _ => {
                if cfg!(debug_assertions) {
                    println!("unknown event: {message}")
                }
            }
        }
    }

    Disconnected
}
