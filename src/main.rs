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
    thread::spawn(|| {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                tokio::join!(
                    endpoints::launch(),
                    process_websocket(),
                    process_log(),
                    async {
                        if let Err(e) = init::init_var().await {
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
