use crate::{
    fetcher::{self, ResponseExt as _},
    var::USERS,
    vrc_structs::{User, UserProfile},
};

pub async fn init_var(token: &str) -> anyhow::Result<()> {
    let (user_profile, online, offline) = tokio::try_join!(
        fetch_user_info(token),
        fetch_all_friends(token, false),
        fetch_all_friends(token, true),
    )?;

    let (web, online) = online
        .into_iter()
        .partition(|u| user_profile.activeFriends.contains(&u.id));

    let presence = user_profile.presence;
    let myself = User {
        bio: user_profile.bio,
        bioLinks: user_profile.bioLinks,
        currentAvatarThumbnailImageUrl: user_profile.currentAvatarThumbnailImageUrl,
        displayName: user_profile.displayName,
        id: user_profile.id,
        isFriend: user_profile.isFriend,
        location: (!presence.world.is_empty() && !presence.instance.is_empty())
            .then(|| format!("{}:{}", presence.world, presence.instance)),
        travelingToLocation: (!presence.travelingToWorld.is_empty()
            && !presence.travelingToInstance.is_empty())
        .then(|| {
            format!(
                "{}:{}",
                presence.travelingToWorld, presence.travelingToInstance
            )
        }),
        status: user_profile.status,
        statusDescription: user_profile.statusDescription,
        tags: user_profile.tags,
        userIcon: user_profile.userIcon,
        profilePicOverride: user_profile.profilePicOverride,
        currentAvatarImageUrl: user_profile.currentAvatarImageUrl,
        developerType: user_profile.developerType,
        last_login: user_profile.last_login,
        last_platform: user_profile.last_platform,
        friendKey: user_profile.friendKey,
    };

    let users = &mut USERS.write().await;

    users.myself = Some(myself);
    users.online = online;
    users.web = web;
    users.offline = offline;

    Ok(())
}

pub async fn fetch_user_info(token: &str) -> anyhow::Result<UserProfile> {
    fetcher::get("https://api.vrchat.cloud/api/1/auth/user", token)
        .await?
        .json()
        .await
}

async fn fetch_all_friends(token: &str, is_offline: bool) -> anyhow::Result<Vec<User>> {
    let mut offset = 0u16;
    let mut friends = Vec::new();

    loop {
        let friends_ = fetch_friends(token, is_offline, offset).await?;
        if friends_.is_empty() {
            break;
        }
        friends.extend(friends_);
        offset += 50;
    }

    friends.iter_mut().for_each(User::unsanitize);

    Ok(friends)
}

async fn fetch_friends(token: &str, is_offline: bool, offset: u16) -> anyhow::Result<Vec<User>> {
    fetcher::get(
        &format!("https://api.vrchat.cloud/api/1/auth/user/friends?offline={is_offline}&n=50&offset={offset}"),
        token,
    )
    .await?
    .json()
    .await
}
