use log::{error, info};
use rusqlite::{Connection, Error, Result};
use std::path::PathBuf;

use crate::db_handler::TableHandler;

#[derive(Default)]
pub struct Database {
    pub path: PathBuf,
}

impl TableHandler for Database {
    fn create_table(&self, table_name: &str, columns: Vec<&str>) -> Result<(), String> {
        unimplemented!("create_table() unimplemented")
    }
    fn insert_into_table(&self, table_name: &str, values: Vec<&str>) -> Result<(), String> {
        unimplemented!("insert_into_table() unimplemented")
    }
    fn select_from_table(
        &self,
        table_name: &str,
        columns: Vec<&str>,
    ) -> Result<Vec<Vec<String>>, String> {
        unimplemented!("select_from_table() unimplemented")
    }
    fn update_table(
        &self,
        table_name: &str,
        column: &str,
        value: &str,
        condition: &str,
    ) -> Result<(), String> {
        unimplemented!("update_table() unimplemented")
    }
    fn delete_from_table(&self, table_name: &str, condition: &str) -> Result<(), String> {
        unimplemented!("delete_from_table() unimplemented")
    }
}

impl Database {
    //create a table
    pub fn new(db_path: &PathBuf) -> Self {
        let conn = Connection::open(db_path).expect("connection not granted");
        let stmt = conn.execute(
            "
        CREATE TABLE IF NOT EXISTS test_table (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        lastname TEXT NOT NULL)",
            (),
        );
        match stmt {
            Ok(num) => info!("\nNew test_table created at path: {:?}", &db_path),
            Err(e) => error!("'{e}'"),
        }
        Self {
            path: PathBuf::from(db_path),
        }
    }
    pub fn insert_to_table(
        &self,
        table_name: &str,
        column_name: Vec<&str>,
        content: Vec<&str>,
    ) -> Result<(), Error> {
        let conn = Connection::open(&self.path)?;
        let columns = column_name.join(", ");
        let placeholders = vec!["?"; content.len()].join(", ");
        let query = format!("INSERT INTO {table_name} ({columns}) VALUES ({placeholders})");
        let stmt = conn.execute(&query, rusqlite::params_from_iter(content.iter().cloned()))?;
        Ok(info!(
            "
            Value inserted !
            \tTable : {}
            \tColumn : {}
            \tvalue: {:?}",
            table_name, columns, content
        ))
    }
}
