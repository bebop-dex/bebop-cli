use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

const TTL_SECS: u64 = 3600;

pub(crate) fn cache_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".cache").join("bebop-cli")
}

fn cache_path(key: &str) -> PathBuf {
    cache_dir().join(format!("{key}.json"))
}

pub fn read(key: &str) -> Option<String> {
    let path = cache_path(key);
    let modified = fs::metadata(&path).ok()?.modified().ok()?;
    let age = SystemTime::now().duration_since(modified).ok()?;
    if age.as_secs() > TTL_SECS {
        return None;
    }
    fs::read_to_string(&path).ok()
}

pub fn write(key: &str, data: &str) {
    let _ = fs::create_dir_all(cache_dir());
    let _ = fs::write(cache_path(key), data);
}
