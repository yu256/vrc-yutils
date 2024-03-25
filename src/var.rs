use crate::vrc_structs::User;
use serde::Serialize;
use tokio::sync::RwLock;

pub(crate) const APP_NAME: &str = "vrc-yutils";
pub(crate) const UA: &str = "User-Agent";

pub(crate) static USERS: RwLock<Users> = RwLock::const_new(Users::new());

#[derive(Serialize)]
pub(crate) struct Users {
    pub myself: Option<User>,
    pub online: Vec<User>,
    pub web: Vec<User>,
    pub offline: Vec<User>,
}

impl Users {
    const fn new() -> Self {
        Self {
            myself: None,
            online: Vec::new(),
            web: Vec::new(),
            offline: Vec::new(),
        }
    }
}
