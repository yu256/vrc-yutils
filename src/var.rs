use tokio::sync::Mutex;

pub(crate) const APP_NAME: &str = "vrc-yutils";
pub(crate) const UA: &str = "User-Agent";

pub(crate) static SELF_LOCATION: Mutex<Option<String>> = Mutex::const_new(None);
