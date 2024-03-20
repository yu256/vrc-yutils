use crate::{
    fetcher::{self, ResponseExt as _},
    init::fetch_user_info,
    var::{APP_NAME, UA, USERS},
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
            let mut req = format!("wss://{host}/.unwrap(){auth}")
                .into_client_request()
                .unwrap();
            req.headers_mut()
                .insert(UA, HeaderValue::from_static(APP_NAME));
            req
        })
        .clone();

    let mut stream = match connect_async(request).await {
        Ok((stream, _)) => stream,
        Err(e @ tungstenite::error::Error::Io(_)) => return IoErr(e),
        Err(e) => return Unknown(e.to_string()),
    };

    while let Some(message) = stream.next().await {
        use crate::vrc_structs::*;

        let message = match message {
            Ok(Message::Text(text)) if text.starts_with(r#"{"err"#) => {
                return if text.contains("authToken") {
                    Token
                } else {
                    Unknown(text)
                };
            }
            Ok(Message::Text(text)) => text,
            Ok(tungstenite::Message::Close(_)) | Err(tungstenite::error::Error::Protocol(_)) => {
                return Disconnected
            }
            Err(e @ tungstenite::error::Error::Io(_)) => return IoErr(e),
            Err(e) => return Unknown(e.to_string()),
            _ => continue,
        };

        let StreamBody { r#type, content } = serde_json::from_str::<StreamBody>(&message).unwrap();

        match r#type.as_str() {
            "friend-online" | "friend-location" => {
                let (mut user, world) = serde_json::from_str::<FriendLocation>(&content)
                    .unwrap()
                    .normalize();
                user.unsanitize();

                if let Some(to) = &user.travelingToLocation {
                    notify_join(Some(to), &user.displayName, JoinType::PlayerJoining).await;
                }

                let users = &mut USERS.write().await;

                if let Some(index) = users.offline.iter().position(|x| x.id == user.id) {
                    users.offline.remove(index);
                }

                if let Some(friend) = users.online.iter_mut().find(|users| users.id == user.id) {
                    *friend = user;
                } else {
                    users.online.push(user);
                    users.online.sort();
                }
            }

            "friend-add" => {
                let id = serde_json::from_str::<UserIdContent>(&content)
                    .unwrap()
                    .userId;
                let mut new_friend =
                    fetcher::get(&format!("https://api.vrchat.cloud/api/1/users/{id}"), auth)
                        .await
                        .unwrap()
                        .json::<User>()
                        .await
                        .unwrap();

                new_friend.unsanitize();

                if new_friend.location.as_ref().is_some_and(|l| l != "offline") {
                    if let Status::AskMe | Status::Busy = new_friend.status {
                        if fetch_user_info(auth)
                            .await
                            .unwrap()
                            .activeFriends
                            .contains(&new_friend.id)
                        {
                            let locked = &mut USERS.write().await;
                            locked.web.push(new_friend);
                            locked.web.sort();
                        } else {
                            let locked = &mut USERS.write().await;
                            locked.online.push(new_friend);
                            locked.online.sort();
                        }
                    } else {
                        let locked = &mut USERS.write().await;
                        locked.online.push(new_friend);
                        locked.online.sort();
                    }
                } else {
                    let locked = &mut USERS.write().await;
                    locked.offline.push(new_friend);
                    locked.offline.sort();
                }
            }

            t @ ("friend-offline" | "friend-delete" | "friend-active") => {
                let id = serde_json::from_str::<UserIdContent>(&content)
                    .unwrap()
                    .userId;
                let users = &mut USERS.write().await;

                macro_rules! move_friend {
                    ([$($from:ident),*], $to:ident) => {
                        'block: {
                            $(
                                if let Some(index) = users.$from.iter().position(|x| x.id == id) {
                                    let user = users.$from.remove(index);
                                    users.$to.push(user);
                                    users.$to.sort();
                                    break 'block;
                                }
                            )*
                        }
                    };
                }

                macro_rules! remove_friend {
                    ([$($from:ident),*]) => {
                        'block: {
                            $(
                                if let Some(index) = users.$from.iter().position(|x| x.id == id) {
                                    users.$from.remove(index);
                                    break 'block;
                                }
                            )*
                        }
                    };
                }

                match t {
                    "friend-offline" => move_friend!([online, web], offline),
                    "friend-delete" => remove_friend!([online, web, offline]),
                    "friend-active" => move_friend!([online, offline], web),
                    _ => unreachable!(),
                }
            }

            "user-location" => {
                let user = serde_json::from_str::<FriendLocation>(&content)
                    .unwrap()
                    .normalize()
                    .0;
                USERS.write().await.myself = Some(user);
            }

            "notification"
            | "see-notification"
            | "hide-notification"
            | "notification-v2"
            | "notification-v2-update"
            | "notification-v2-delete" => {}

            _ => {
                if cfg!(debug_assertions) {
                    println!("unknown event: {message}")
                }
            }
        }
    }

    Disconnected
}
