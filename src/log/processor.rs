use crate::xsoverlay::notify_join::{notify_join, JoinType};
use once_cell::sync::Lazy;
use regex::{Match, Regex};

pub(super) async fn parse_and_process(line: &str) {
    if let Some(cap) = parse(line).await {
        process_event(cap).await;
    }
}

async fn parse(line: &str) -> Option<(&JoinType, Option<Match>)> {
    static PATTERNS: Lazy<[(JoinType, regex::Regex); 4]> = Lazy::new(|| {
        [
            (
                JoinType::PlayerJoined,
                Regex::new(r"\[Behaviour\] OnPlayerJoined (.+)").unwrap(),
            ),
            (
                JoinType::PlayerLeft,
                Regex::new(r"\[Behaviour\] OnPlayerLeft (.+)").unwrap(),
            ),
            (
                JoinType::JoinedRoom,
                Regex::new(r"\[Behaviour\] OnJoinedRoom").unwrap(),
            ),
            (
                JoinType::LeftRoom,
                Regex::new(r"\[Behaviour\] OnLeftRoom").unwrap(),
            ),
        ]
    });

    PATTERNS.iter().find_map(|(pattern, regex)| match pattern {
        JoinType::PlayerJoined | JoinType::PlayerLeft => regex
            .captures(line)
            .and_then(|c| c.get(1))
            .map(|c| (pattern, Some(c))),
        _ if regex.is_match(line) => Some((pattern, None)),
        _ => None,
    })
}

async fn process_event(cap: (&JoinType, Option<Match<'_>>)) {
    if let (join_type @ (JoinType::PlayerJoined | JoinType::PlayerLeft), Some(cap)) = cap {
        notify_join(None, cap.as_str(), *join_type).await;
    }
}
