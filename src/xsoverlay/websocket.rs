use crate::var::APP_NAME;
use anyhow::Result;
use futures_util::{stream::FusedStream as _, SinkExt};
use std::sync::OnceLock;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{client::IntoClientRequest, http::Request, Message},
    MaybeTlsStream, WebSocketStream,
};

type WS = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[derive(serde::Serialize)]
pub(crate) struct XSOverlay<'a> {
    pub(crate) sender: &'a str,
    pub(crate) target: &'a str,
    pub(crate) command: &'a str,
    #[serde(rename = "jsonData")]
    pub(crate) json_data: &'a str,
    #[serde(rename = "rawData")]
    pub(crate) raw_data: Option<&'a str>,
}

pub(crate) async fn send_message(message: &XSOverlay<'_>, port: Option<u16>) -> Result<()> {
    static WS_CONNECTION: Mutex<Option<WS>> = Mutex::const_new(None);

    let mut ws = WS_CONNECTION.lock().await;
    if !ws.as_ref().is_some_and(|ws| !ws.is_terminated()) {
        try_new(&mut ws, None).await?;
    }

    let mut tried = false;
    loop {
        match unsafe { ws.as_mut().unwrap_unchecked() }
            .send(Message::Text(serde_json::to_string(&message)?))
            .await
        {
            Ok(_) => break Ok(()),
            Err(_) if !tried => {
                try_new(&mut ws, port).await?;
                tried = true;
            }
            Err(e) => return Err(e.into()),
        }
    }
}

#[inline]
async fn try_new(ws_ref: &mut Option<WS>, port: Option<u16>) -> Result<()> {
    static REQUEST: OnceLock<Request<()>> = OnceLock::new();
    let request = REQUEST
        .get_or_init(|| {
            format!(
                "ws://localhost:{}/?client={APP_NAME}",
                port.unwrap_or(42070)
            )
            .into_client_request()
            .unwrap()
        })
        .clone();
    *ws_ref = Some(connect_async(request).await?.0);
    Ok(())
}
