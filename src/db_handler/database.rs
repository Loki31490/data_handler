use log::{error, info};
use rusqlite::{Connection, Error, Result, params};
use std::path::PathBuf;

pub struct Database {
    path: PathBuf,
}

impl Database {
    //create a table
    pub fn new(path: PathBuf) -> Self {
        let conn = Connection::open(&path).expect("connection not granted");
        let stmt = conn.execute(
            "
        CREATE TABLE IF NOT EXISTS test_table (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL)",
            (),
        );
        match stmt {
            Ok(num) => info!("New test_table created at path: \n{:?}", &path),
            Err(e) => error!("'{e}'"),
        }
        Self { path: path }
    }
    pub fn insert_to_table(self, table: &str, column: &str, content: &str) -> Result<(), Error> {
        let conn = Connection::open(&self.path)?;
        let stmt = conn.execute(
            "
            INSERT INTO test_table (name)
            VALUES (?3)
",
            params![table, column, content],
        )?;
        Ok(())
    }
}
