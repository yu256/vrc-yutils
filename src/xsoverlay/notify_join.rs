#[cfg(feature = "websocket")]
use super::websocket::{send_message, XSOverlay};
#[cfg(not(feature = "websocket"))]
use crate::udp_client::send_message;
use crate::var::{APP_NAME, SELF_LOCATION};
use serde::Serialize;
use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub(crate) enum JoinType {
    PlayerJoining,
    PlayerJoined,
    PlayerLeft,
    JoinedRoom,
    LeftRoom,
}

impl Display for JoinType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JoinType::PlayerJoining => write!(f, "is joining"),
            JoinType::PlayerJoined => write!(f, "has joined"),
            JoinType::PlayerLeft => write!(f, "has left"),
            JoinType::JoinedRoom => todo!(),
            JoinType::LeftRoom => todo!(),
        }
    }
}

/// toがNoneの場合、locationのチェックを行わない
pub(crate) async fn notify_join(to: Option<&str>, display_name: &str, join_type: JoinType) {
    if matches!(to, Some(to) if to.is_empty() || to != *SELF_LOCATION.lock().await) {
        return;
    }

    let notification = Notification {
        title: &format!("{join_type:?}"),
        content: &format!("{display_name} {join_type}."),
        ..Default::default()
    };

    #[cfg(feature = "websocket")]
    let notification = XSOverlay {
        sender: APP_NAME,
        target: "xsoverlay",
        command: "SendNotification",
        json_data: &serde_json::to_string(&notification).unwrap(),
        raw_data: None,
    };

    if let Err(e) = send_message(&notification, None).await {
        eprintln!("{e}\nXSOverlayへメッセージを送信する際にエラーが発生しました。");
    }
}

#[derive(Serialize)]
struct Notification<'a> {
    #[cfg(not(feature = "websocket"))]
    #[serde(rename = "messageType")]
    message_type: i32,
    #[cfg(feature = "websocket")]
    #[serde(rename = "type")]
    message_type: i32,
    index: i32,
    timeout: f32,
    height: f32,
    opacity: f32,
    volume: f32,
    #[serde(rename = "audioPath")]
    audio_path: &'a str,
    title: &'a str,
    content: &'a str,
    #[serde(rename = "useBase64Icon")]
    use_base64_icon: bool,
    icon: &'a str,
    #[serde(rename = "sourceApp")]
    source_app: &'a str,
}

impl<'a> Default for Notification<'a> {
    fn default() -> Self {
        Notification {
            message_type: 1,
            index: 0,
            timeout: 1.5,
            height: 175.0,
            opacity: 1.0,
            volume: 0.7,
            audio_path: "default",
            title: "",
            content: "",
            use_base64_icon: false,
            icon: "default",
            source_app: APP_NAME,
        }
    }
}
