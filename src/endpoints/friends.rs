use crate::var::USERS;
use axum::extract::ConnectInfo;
use std::net::SocketAddr;

pub(super) async fn friends(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    println!("{:?}", addr);
    serde_json::to_string(&*USERS.read().await).unwrap()
}
