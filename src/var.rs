use std::{env, path::PathBuf};

use crate::vrc_structs::User;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::{
    fs::File,
    io::{AsyncReadExt as _, AsyncWriteExt as _, BufReader, BufWriter},
    sync::RwLock,
};

pub(crate) const APP_NAME: &str = "vrc-yutils";
pub(crate) const UA: &str = "User-Agent";
pub(crate) const CFG_FILE_NAME: &str = "config.json";

pub(crate) static CURRENT_DIR: Lazy<PathBuf> = Lazy::new(|| env::current_dir().unwrap());

pub(crate) static USERS: RwLock<Users> = RwLock::const_new(Users::new());

pub(crate) static CFG: RwLock<Config> = RwLock::const_new(Config {
    token: String::new(),
    alt_url: None,
});

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

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Config {
    pub(crate) token: String,
    pub(crate) alt_url: Option<String>,
}

pub(crate) trait ConfigRW {
    async fn init(&self) -> anyhow::Result<()>;
    async fn get(&self) -> tokio::sync::RwLockReadGuard<'_, Config>;
    // async fn set(&self, new_config: &Config) -> anyhow::Result<()>;
    async fn set(&self, setter: impl FnOnce(&Config) -> Config) -> anyhow::Result<()>;
}

impl ConfigRW for RwLock<Config> {
    async fn init(&self) -> anyhow::Result<()> {
        let Ok(file) = File::open(CURRENT_DIR.join(CFG_FILE_NAME)).await else {
            self.set(|_| Config {
                token: "default".into(),
                alt_url: None,
            })
            .await?;
            return Box::pin(self.init()).await;
        };

        let mut file = BufReader::new(file);

        let mut content = String::new();

        file.read_to_string(&mut content).await?;

        let config = serde_json::from_str(&content)?;

        *self.write().await = config;

        Ok(())
    }

    async fn get(&self) -> tokio::sync::RwLockReadGuard<'_, Config> {
        loop {
            {
                let config = self.read().await;
                if !matches!(config.token.as_str(), "default" | "") {
                    break config;
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    }

    async fn set(&self, setter: impl FnOnce(&Config) -> Config) -> anyhow::Result<()> {
        async fn write_json(new_config: &Config) -> anyhow::Result<()> {
            let file = File::create(CURRENT_DIR.join(CFG_FILE_NAME)).await?;

            let json = serde_json::to_vec(new_config)?;

            let mut file = BufWriter::new(file);

            file.write_all(&json).await?;
            file.flush().await.map_err(From::from)
        }

        let new_config = setter(&*self.read().await);

        tokio::try_join!(write_json(&new_config), async {
            *self.write().await = new_config.clone();
            Ok(())
        })?;

        Ok(())
    }
}
