mod authorize;
mod endpoints;
mod fetcher;
mod init;
mod log;
mod udp_client;
mod unsanitizer;
mod var;
mod vrc_structs;
mod websocket;
mod xsoverlay;

use crate::{var::APP_NAME, websocket::stream::process_websocket};
use log::process_log;
use std::thread;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

fn main() -> anyhow::Result<()> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()?;

    let args = std::env::args().collect::<Vec<_>>();

    // let uri = find_arg_value(&args, "--url").and_then(|a| a.parse::<Uri>().ok());
    let uri = None;

    let auth = match args
        .iter()
        .find(|arg| arg.starts_with("--auth"))
        .map(|a| &a[2..])
    {
        Some(auth) => auth.to_string(),
        None => runtime.block_on(authorize::auth())?,
    }
    .leak();

    thread::spawn(move || {
        runtime.block_on(async {
            tokio::join!(
                endpoints::launch(),
                process_websocket(auth, uri.as_ref()),
                process_log(),
                async {
                    if let Err(e) = init::init_var(auth).await {
                        eprintln!("{e}")
                    }
                }
            );
        })
    });

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(APP_NAME)
        .build(&event_loop)?;

    let _webview = WebViewBuilder::new(&window)
        .with_url("http://localhost:8000")
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit
        }
    });
}
