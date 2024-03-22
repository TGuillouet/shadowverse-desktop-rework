use std::path::PathBuf;

pub fn local_directory() -> PathBuf {
    dirs_next::data_local_dir().unwrap()
}

pub fn config_directory() -> PathBuf {
    dirs_next::config_dir().unwrap()
}
