use log::{error, info};
use rusqlite::{Connection, Error, Result, params};
use std::path::PathBuf;

pub enum Tables {
    TestTable,
}

impl Tables {
    pub fn table(table: Tables) -> &'static str {
        match table {
            Tables::TestTable => "test_table",
        }
    }
}

impl std::fmt::Display for Tables {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self)?;
        Ok(())
    }
}

pub struct Database {
    path: PathBuf,
}

impl Database {
    //create a table
    pub fn new(db_path: PathBuf) -> Self {
        let conn = Connection::open(&db_path).expect("connection not granted");
        let stmt = conn.execute(
            &format!(
                "
        CREATE TABLE IF NOT EXISTS test_table (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL)"
            ),
            (),
        );
        match stmt {
            Ok(num) => info!("\nNew test_table created at path: {:?}", &db_path),
            Err(e) => error!("'{e}'"),
        }
        Self { path: db_path }
    }
    pub fn insert_to_table(
        self,
        table_name: &str,
        column_name: &str,
        content: &str,
    ) -> Result<(), Error> {
        let conn = Connection::open(&self.path)?;
        let stmt = conn.execute(
            &format!(
                "
            INSERT INTO {table_name} ({column_name})
            VALUES (?1)"
            ),
            params![content],
        )?;
        Ok(info!(
            "
            Value inserted !
            \tTable : {}
            \tColumn : {}
            \tvalue: {}",
            table_name, column_name, content
        ))
    }
}
