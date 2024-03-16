use serde::Deserialize;

#[derive(Deserialize, Ord, PartialEq, PartialOrd, Eq, Clone, Copy, Debug)]
pub enum Status {
    #[serde(rename = "join me")]
    JoinMe,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "ask me")]
    AskMe,
    #[serde(rename = "busy")]
    Busy,
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

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct StreamBody {
    pub r#type: String,
    pub content: String, // json
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct FriendLocation {
    pub userId: String,
    pub location: String,
    pub travelingToLocation: Option<String>,
    pub worldId: String,
    pub canRequestInvite: Option<bool>,
    pub user: User,
    pub world: Option<World>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Eq)]
pub struct User {
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
    pub status: String,
    pub date_joined: String,
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
