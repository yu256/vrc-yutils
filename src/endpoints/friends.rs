use crate::var::USERS;
use axum::{extract::ConnectInfo, http::HeaderValue, response::IntoResponse};
use hyper::header;
use std::{
    net::{Ipv4Addr, SocketAddr},
    time::Duration,
};

const LOCALHOST: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

pub(super) async fn friends(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
    if addr.ip() != LOCALHOST {
        return (
            [(
                header::CONTENT_TYPE,
                HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
            )],
            "{}".into(),
        );
    }

    // todo: 認証を入れる

    loop {
        {
            let users = &*USERS.read().await;
            if users.myself.is_some() {
                break (
                    [(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
                    )],
                    serde_json::to_string(users).unwrap(),
                );
            }
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
