use crate::{
    fetcher::{self, ResponseExt as _},
    init::fetch_user_info,
    var::{ConfigRW, APP_NAME, CFG, UA, USERS},
    xsoverlay::notify_join::{notify_join, JoinType},
};
use async_once_cell::OnceCell;
use futures_util::StreamExt as _;
use hyper::Uri;
use std::time::Duration;
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

pub(crate) async fn process_websocket() {
    // インターネットに接続されていないときに無限に接続を試みてしまわないように
    let mut io_err_cnt = 0u8;

    loop {
        match connect_websocket().await {
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

macro_rules! remove_friend {
    ($friends:expr, $id:expr, [$($from:ident),*]) => {
        'block: {
            $(
                if $friends.$from.find_remove(|x| x.id == $id).is_some() {
                    break 'block;
                }
            )*
        }
    };
}

async fn connect_websocket() -> WSError {
    static REQUEST: OnceCell<Request<()>> = OnceCell::new();
    let request = REQUEST
        .get_or_init(async {
            let config = CFG.get().await;

            let uri = config
                .alt_url
                .as_ref()
                .and_then(|url| url.parse::<Uri>().ok());

            let host = uri
                .as_ref()
                .and_then(|u| u.host())
                .unwrap_or("pipeline.vrchat.cloud");

            let mut req = format!("wss://{host}/?{}", config.token)
                .into_client_request()
                .unwrap();

            req.headers_mut()
                .insert(UA, HeaderValue::from_static(APP_NAME));
            req
        })
        .await
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

        tokio::spawn(async move {
            let result = async move {
                let StreamBody { r#type, content } = serde_json::from_str::<StreamBody>(&message)?;

                match r#type.as_str() {
                    "friend-online" | "friend-location" => {
                        let (mut user, _) =
                            serde_json::from_str::<FriendLocation>(&content)?.normalize();
                        user.unsanitize();

                        if let Some(to) = &user.travelingToLocation {
                            let to = to.to_owned();
                            let display_name = user.displayName.clone();
                            tokio::spawn(async move {
                                notify_join(Some(&to), &display_name, JoinType::PlayerJoining).await
                            });
                        }

                        let users = &mut USERS.write().await;

                        if let Some(friend) =
                            users.online.iter_mut().find(|friend| friend.id == user.id)
                        {
                            let need_to_be_sorted = friend.status != user.status;
                            *friend = user;
                            if need_to_be_sorted {
                                users.online.sort();
                            }
                        } else {
                            remove_friend!(users, user.id, [offline, web]);
                            users.online.push_and_sort(user);
                        }
                    }

                    "friend-active" => {
                        let user = serde_json::from_str::<FriendActive>(&content)?.user;
                        let locked = &mut USERS.write().await;
                        remove_friend!(locked, user.id, [offline, online, web]);
                        locked.web.push_and_sort(user);
                    }

                    "friend-add" => {
                        let id = serde_json::from_str::<UserIdContent>(&content)?.userId;
                        let mut new_friend = fetcher::get(
                            &format!("https://api.vrchat.cloud/api/1/users/{id}"),
                            &CFG.get().await.token,
                        )
                        .await?
                        .json::<User>()
                        .await?;

                        new_friend.unsanitize();

                        let write = USERS.write();

                        if new_friend.location.as_ref().is_some_and(|l| l != "offline") {
                            if let Status::AskMe | Status::Busy = new_friend.status {
                                if fetch_user_info(&CFG.get().await.token)
                                    .await?
                                    .activeFriends
                                    .contains(&new_friend.id)
                                {
                                    write.await.web.push_and_sort(new_friend);
                                } else {
                                    write.await.online.push_and_sort(new_friend);
                                }
                            } else {
                                write.await.online.push_and_sort(new_friend);
                            }
                        } else {
                            write.await.offline.push(new_friend);
                        }
                    }

                    "friend-offline" => {
                        let id = serde_json::from_str::<UserIdContent>(&content)?.userId;
                        let locked = &mut USERS.write().await;

                        if let Some(mut friend) = locked
                            .online
                            .find_remove(|x| x.id == id)
                            .or_else(|| locked.web.find_remove(|x| x.id == id))
                        {
                            friend.status = Default::default();
                            friend.location = Default::default();
                            friend.travelingToLocation = Default::default();
                            locked.offline.push(friend);
                        }
                    }

                    "friend-delete" => {
                        let id = serde_json::from_str::<UserIdContent>(&content)?.userId;
                        let locked = &mut USERS.write().await;
                        remove_friend!(locked, id, [offline, web, online]);
                    }

                    "user-location" => {
                        let user = serde_json::from_str::<FriendLocation>(&content)?
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
                Ok::<_, anyhow::Error>(())
            }
            .await;

            if let Err(e) = result {
                eprintln!("{e}");
            }
        });
    }

    Disconnected
}

trait VecExt<T> {
    fn find_remove<F>(&mut self, fun: F) -> Option<T>
    where
        F: Fn(&T) -> bool;
    fn push_and_sort(&mut self, item: T)
    where
        T: Ord;
}

impl<T> VecExt<T> for Vec<T> {
    fn find_remove<F>(&mut self, fun: F) -> Option<T>
    where
        F: Fn(&T) -> bool,
    {
        self.iter().position(fun).map(|i| self.remove(i))
    }
    fn push_and_sort(&mut self, item: T)
    where
        T: Ord,
    {
        self.push(item);
        self.sort();
    }
}
