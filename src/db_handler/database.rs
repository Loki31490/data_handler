use std::path::PathBuf;

pub struct Database {
    pub path: PathBuf,
}

impl Database {
    pub fn new(path: PathBuf) -> Self {
        Self { path: path }
    }
}
