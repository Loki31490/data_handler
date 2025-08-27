#![allow(unused_variables)]

mod db_handler;
use db_handler::Database;
use db_handler::Tables;

use rusqlite::{Error, Result};
use std::path::PathBuf;

fn main() -> Result<(), Error> {
    env_logger::init();
    //trace!("trace"); PRIORITY = 4
    //info!("info"); PRIORITY = 3
    //warn!("warn"); PRIORITY = 2
    //error!("error"); PRIORITY = 1

    let path = PathBuf::from("/home/baptiste/Documents/Rust/data_handler/data/my_data.db3");
    let my_database = Database::new(path);
    my_database.insert_to_table(Tables::table(Tables::TestTable), "name", "Joe Danger")?;
    Ok(())
}
