use crate::unsanitizer::Unsanitizer as _;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Ord, PartialEq, PartialOrd, Eq, Clone, Copy, Debug)]
pub enum Status {
    #[serde(rename = "join me")]
    JoinMe,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "ask me")]
    AskMe,
    #[serde(rename = "busy")]
    Busy,
    #[serde(rename = "offline")]
    Offline,
}

impl Default for Status {
    fn default() -> Self {
        Self::Offline
    }
}

impl Ord for User {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.status.cmp(&other.status)
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status
    }
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.status.cmp(&other.status))
    }
}

impl User {
    pub fn unsanitize(&mut self) {
        self.bio = self.bio.unsanitize();
        self.statusDescription = self.statusDescription.unsanitize();
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Eq)]
pub struct User {
    pub id: String,
    pub location: Option<String>,
    pub travelingToLocation: Option<String>,
    pub displayName: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "str::is_empty")]
    pub userIcon: String,
    #[serde(default)]
    pub bio: String,
    #[serde(default)]
    pub bioLinks: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "str::is_empty")]
    pub profilePicOverride: String,
    #[serde(default)]
    pub statusDescription: String,
    #[serde(default)]
    pub currentAvatarImageUrl: String,
    #[serde(default)]
    pub currentAvatarThumbnailImageUrl: String,
    pub tags: Vec<String>,
    pub developerType: String,
    pub last_login: String,
    pub last_platform: String,
    pub status: Status,
    pub isFriend: bool,
    pub friendKey: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct StreamBody {
    pub r#type: String,
    pub content: String, // json
}

impl FriendLocation {
    pub fn normalize(self) -> (User, Option<World>) {
        (
            User {
                bio: self.user.bio,
                bioLinks: self.user.bioLinks,
                currentAvatarThumbnailImageUrl: self.user.currentAvatarThumbnailImageUrl,
                displayName: self.user.displayName,
                id: self.user.id,
                isFriend: self.user.isFriend,
                location: self.location,
                travelingToLocation: self.travelingToLocation,
                status: self.user.status,
                statusDescription: self.user.statusDescription,
                tags: self.user.tags,
                userIcon: self.user.userIcon,
                profilePicOverride: self.user.profilePicOverride,
                currentAvatarImageUrl: self.user.currentAvatarImageUrl,
                developerType: self.user.developerType,
                last_login: self.user.last_login,
                last_platform: self.user.last_platform,
                friendKey: self.user.friendKey,
            },
            self.world,
        )
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct UserIdContent {
    pub userId: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub(super) struct FriendActive {
    pub(super) user: User,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct FriendLocation {
    pub userId: String,
    pub location: Option<String>,
    pub travelingToLocation: Option<String>,
    pub worldId: Option<String>,
    pub canRequestInvite: Option<bool>,
    pub user: EventUser,
    pub world: Option<World>,
}

// location / travelingToLocationが欠けている
#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct EventUser {
    pub id: String,
    pub displayName: String,
    #[serde(default)]
    pub userIcon: String,
    #[serde(default)]
    pub bio: String,
    #[serde(default)]
    pub bioLinks: Vec<String>,
    #[serde(default)]
    pub profilePicOverride: String,
    #[serde(default)]
    pub statusDescription: String,
    #[serde(default)]
    pub currentAvatarImageUrl: String,
    #[serde(default)]
    pub currentAvatarThumbnailImageUrl: String,
    pub currentAvatarTags: Vec<String>,
    pub state: String,
    pub tags: Vec<String>,
    pub developerType: String,
    pub last_login: String,
    pub last_platform: String,
    pub allowAvatarCopying: bool,
    pub status: Status,
    pub isFriend: bool,
    pub friendKey: String,
    pub last_activity: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct World {
    pub id: String,
    pub name: String,
    pub description: String,
    pub authorId: String,
    pub authorName: String,
    pub releaseStatus: String,
    pub featured: bool,
    pub capacity: i32,
    pub recommendedCapacity: i32,
    pub imageUrl: String,
    pub thumbnailImageUrl: String,
    pub namespace: String,
    pub version: i32,
    pub organization: String,
    pub previewYoutubeId: Option<String>,
    pub udonProducts: Vec<String>,
    pub favorites: i32,
    pub visits: i32,
    pub popularity: i32,
    pub heat: i32,
    pub publicationDate: String,
    pub labsPublicationDate: String,
    pub instances: Vec<String>,
    pub publicOccupants: i32,
    pub privateOccupants: i32,
    pub occupants: i32,
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub displayName: String,
    pub userIcon: String,
    pub bio: String,
    pub bioLinks: Vec<String>,
    pub profilePicOverride: String,
    pub statusDescription: String,
    pub username: String,
    pub pastDisplayNames: Vec<String>,
    pub hasEmail: bool,
    pub hasPendingEmail: bool,
    pub obfuscatedEmail: String,
    pub obfuscatedPendingEmail: String,
    pub emailVerified: bool,
    pub hasBirthday: bool,
    pub hideContentFilterSettings: bool,
    pub unsubscribe: bool,
    pub statusHistory: Vec<String>,
    pub statusFirstTime: bool,
    pub friends: Vec<String>,
    pub friendGroupNames: Vec<String>,
    pub queuedInstance: Option<String>,
    pub userLanguage: String,
    pub userLanguageCode: String,
    pub currentAvatarImageUrl: String,
    pub currentAvatarThumbnailImageUrl: String,
    pub currentAvatarTags: Vec<String>,
    pub currentAvatar: String,
    pub currentAvatarAssetUrl: String,
    pub fallbackAvatar: String,
    pub accountDeletionDate: Option<String>,
    pub accountDeletionLog: Option<String>,
    pub acceptedTOSVersion: u32,
    pub acceptedPrivacyVersion: u32,
    pub steamId: String,
    pub steamDetails: SteamDetails,
    pub googleId: String,
    pub googleDetails: GoogleDetails,
    pub oculusId: String,
    pub picoId: String,
    pub viveId: String,
    pub hasLoggedInFromClient: bool,
    pub homeLocation: String,
    pub twoFactorAuthEnabled: bool,
    pub twoFactorAuthEnabledDate: Option<String>,
    pub updated_at: String,
    pub state: String,
    pub tags: Vec<String>,
    pub developerType: String,
    pub last_login: String,
    pub last_platform: String,
    pub allowAvatarCopying: bool,
    pub status: Status,
    pub isFriend: bool,
    pub friendKey: String,
    pub last_activity: String,
    pub onlineFriends: Vec<String>,
    pub activeFriends: Vec<String>,
    pub presence: Presence,
    pub offlineFriends: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct SteamDetails {}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct GoogleDetails {}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct Presence {
    pub platform: String,
    pub instance: String,
    pub profilePicOverride: String,
    pub currentAvatarTags: String,
    pub avatarThumbnail: String,
    pub status: Status,
    pub instanceType: String,
    pub travelingToWorld: String,
    pub travelingToInstance: String,
    pub groups: Vec<String>,
    pub world: String,
    pub displayName: String,
    pub id: String,
    pub debugflag: String,
    pub isRejoining: String,
    pub userIcon: String,
}
