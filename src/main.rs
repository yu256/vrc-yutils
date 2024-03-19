mod authorize;
mod fetcher;
mod log;
mod udp_client;
mod var;
mod vrc_structs;
mod websocket;
mod xsoverlay;

use crate::{fetcher::ResponseExt, websocket::stream::process_websocket};
use log::process_log;
use serde::Deserialize;
use std::env;
use var::SELF_LOCATION;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<_>>();
    // let uri = find_arg_value(&args, "--url").and_then(|a| a.parse::<Uri>().ok());
    let uri = None;

    let auth;
    let auth = match args
        .iter()
        .find(|arg| arg.starts_with("--auth"))
        .map(|a| &a[2..])
    {
        Some(auth) => auth,
        None => {
            auth = authorize::auth().await?;
            &auth
        }
    };

    tokio::join!(
        process_websocket(auth, uri.as_ref()),
        process_log(),
        fetch_self_location(auth)
    );

    Ok(())
}

async fn fetch_self_location(auth: &str) {
    let Ok(res) = fetcher::get("https://api.vrchat.cloud/api/1/auth/user", &auth).await else {
        return;
    };

    #[derive(Deserialize)]
    struct Response {
        presence: Inner,
    }

    #[derive(Deserialize)]
    struct Inner {
        world: String,
        instance: String,
    }

    let res = res.json::<Response>().await.ok().map(
        |Response {
             presence: Inner { world, instance },
         }| format!("{world}:{instance}"),
    );

    *SELF_LOCATION.lock().await = res;
}
