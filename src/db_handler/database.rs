use log::{error, info, trace};
use rusqlite::Connection;
use std::path::PathBuf;

pub struct Database {
    pub path: PathBuf,
}

impl Database {
    pub fn from(path: PathBuf) -> Self {
        trace!("\nNEW DATABASE\nfrom path: {:?}", &path);
        Self { path: path }
    }

    pub fn print_db(&self) {
        let conn = Connection::open(&self.path);
        match conn {
            Ok(ok) => info!("\nCONNECTION GRANTED\n{ok:?}"),
            Err(e) => error!("\nCONNECTION NOT GRANTED\n{e}"),
        }
    }
}
