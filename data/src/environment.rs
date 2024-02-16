use std::path::PathBuf;

pub fn config_directory() -> PathBuf {
    dirs_next::config_dir().unwrap()
}
