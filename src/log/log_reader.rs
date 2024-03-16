use super::processor::parse_and_process;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::BTreeMap;
use std::env;
use std::io::SeekFrom;
use std::path::PathBuf;
use std::time::SystemTime;
use tokio::fs::{read_dir, File};
use tokio::io::{self, AsyncBufReadExt as _, AsyncSeekExt as _, BufReader};
use tokio::time::sleep;

pub(crate) async fn process_log() {
    let mut path = None;
    let mut last_pos = 0;
    let mut buf = String::new();

    loop {
        let is_fst = path.is_none();

        let Some(cur_path) = get_latest_log_path().await else {
            return eprintln!("VRChatのログが見つかりませんでした。ログ関連の機能を使用するには、VRChatのログ機能を有効化してください。");
        };

        if !path.as_ref().is_some_and(|prev| prev == &cur_path) {
            path = Some(cur_path);
            last_pos = 0;
        }

        if let Err(e) = read_log(path.as_ref().unwrap(), &mut last_pos, &mut buf, is_fst).await {
            eprintln!("{e}");
        };

        sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

async fn read_log(
    path: &PathBuf,
    last_pos: &mut usize,
    buf: &mut String,
    is_fst: bool,
) -> io::Result<()> {
    let mut reader = BufReader::new(File::open(path).await?);

    loop {
        buf.clear();
        reader.seek(SeekFrom::Start(*last_pos as u64)).await?;

        match reader.read_line(buf).await? {
            0 => break Ok(()),
            n => {
                *last_pos += n;
                if !is_fst {
                    parse_and_process(buf).await;
                }
            }
        }
    }
}

async fn get_latest_log_path() -> Option<PathBuf> {
    let mut paths = get_log_paths()
        .await
        .inspect_err(|e| eprintln!("{e}"))
        .ok()?;

    let latest = paths.pop_last()?;
    Some(latest.1)
}

async fn get_log_paths() -> io::Result<BTreeMap<SystemTime, PathBuf>> {
    static PATH: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = PathBuf::from(env::var("AppData").unwrap());
        path.pop();
        path.push("LocalLow");
        path.push("VRChat");
        path.push("vrchat");
        path
    });

    let mut log_files = read_dir(&*PATH).await?;
    let mut paths = BTreeMap::new();

    while let Ok(Some(entry)) = log_files.next_entry().await {
        if entry
            .file_type()
            .await
            .is_ok_and(|file_type| file_type.is_file())
        {
            static LOG_PATTERN: Lazy<Regex> =
                Lazy::new(|| Regex::new(r"^output_log_.*\.txt$").unwrap());

            if entry
                .file_name()
                .to_str()
                .is_some_and(|name| LOG_PATTERN.is_match(name))
            {
                let modified = entry.metadata().await?.modified()?;
                paths.insert(modified, entry.path());
            }
        }
    }

    Ok(paths)
}
