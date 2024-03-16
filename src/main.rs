mod authorize;
mod fetcher;
mod log;
mod udp_client;
mod var;
mod vrc_structs;
mod websocket;
mod xsoverlay;

use crate::websocket::stream::process_websocket;
use log::process_log;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<_>>();
    // let uri = find_arg_value(&args, "--url").and_then(|a| a.parse::<Uri>().ok());
    let uri = None;

    let auth;
    let auth = match find_arg_value(&args, "--auth") {
        Some(auth) => auth,
        None => {
            auth = authorize::auth().await?;
            &auth
        }
    };

    tokio::join!(process_websocket(auth, uri.as_ref()), process_log());

    Ok(())
}

fn find_arg_value<'a>(args: &'a [String], prefix: &str) -> Option<&'a str> {
    args.iter()
        .find(|arg| arg.starts_with(prefix))
        .and_then(|arg| arg.split('=').last())
}
