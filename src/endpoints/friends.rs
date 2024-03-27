use crate::var::{JSON, USERS};
use axum::{
    extract::ConnectInfo,
    response::{IntoResponse, Response},
};
use hyper::StatusCode;
use std::{
    net::{Ipv4Addr, SocketAddr},
    time::Duration,
};

const LOCALHOST: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

pub(super) async fn friends(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> Response {
    if addr.ip() != LOCALHOST {
        return (JSON, "{}").into_response();
    }

    // todo: 認証を入れる

    let mut try_count = 0u16;

    loop {
        {
            let users = &*USERS.read().await;
            if users.myself.is_some() {
                break (JSON, serde_json::to_string(users).unwrap()).into_response();
            }
            if try_count > 50 {
                break (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    (JSON, "{error:\"unauthorized\"}"),
                )
                    .into_response();
            }
        }
        try_count += 1;
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
