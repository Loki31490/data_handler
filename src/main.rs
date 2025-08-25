#![allow(unused_variables)]

use rusqlite::{Error, Result};
use std::path::PathBuf;
mod db_handler;
use db_handler::Database;

fn main() -> Result<(), Error> {
    env_logger::init();
    //trace!("trace"); PRIORITY = 4
    //info!("info"); PRIORITY = 3
    //warn!("warn"); PRIORITY = 2
    //error!("error"); PRIORITY = 1

    let path = PathBuf::from("/home/baptiste/Documents/Rust/data_handler/data/my_data.db3");
    let database = Database::new(path);
    database.insert_to_table("test_table", "name", "Joe Danger")?;
    Ok(())
}
