mod endpoints;
mod fetcher;
mod init;
#[cfg(not(feature = "server"))]
mod log;
#[cfg(not(feature = "server"))]
mod udp_client;
mod unsanitizer;
mod var;
mod vrc_structs;
mod websocket;
#[cfg(not(feature = "server"))]
mod xsoverlay;

use crate::websocket::stream::process_websocket;

#[cfg(not(feature = "server"))]
fn main() -> anyhow::Result<()> {
    std::thread::spawn(|| {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                tokio::join!(
                    endpoints::launch(),
                    process_websocket(),
                    log::process_log(),
                    async {
                        if let Err(e) = init::init_var().await {
                            eprintln!("{e}")
                        }
                    }
                );
            })
    });

    let event_loop = tao::event_loop::EventLoop::new();
    let window = tao::window::WindowBuilder::new()
        .with_title(var::APP_NAME)
        .build(&event_loop)?;

    let _webview = wry::WebViewBuilder::new(&window)
        .with_url("http://localhost:8000")
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = tao::event_loop::ControlFlow::Wait;

        if let tao::event::Event::WindowEvent {
            event: tao::event::WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = tao::event_loop::ControlFlow::Exit
        }
    });
}

#[cfg(feature = "server")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tokio::join!(endpoints::launch(), process_websocket(), async {
        if let Err(e) = init::init_var().await {
            eprintln!("{e}")
        }
    });

    Ok(())
}
