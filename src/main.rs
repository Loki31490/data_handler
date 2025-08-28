#![allow(unused_variables)]

mod db_handler;
use db_handler::Database;
use db_handler::Tables;

use rusqlite::{Error, Result};
use std::path::PathBuf;

use text_io::read;

fn main() -> Result<(), Error> {
    env_logger::init();
    //trace!("trace"); PRIORITY = 4
    //info!("info"); PRIORITY = 3
    //warn!("warn"); PRIORITY = 2
    //error!("error"); PRIORITY = 1
    println!("Enter name : ");
    let user_input1: String = read!("{}");
    println!("Enter last name : ");
    let user_input2: String = read!("{}");
    let full_name: String = user_input1 + " " + &user_input2;

    let path = PathBuf::from("/home/baptiste/Documents/Rust/data_handler/data/my_data.db3");
    let my_database = Database::new(&path);
    my_database.insert_to_table(Tables::table(Tables::TestTable), "name", &full_name)?;

    println!("{}", &my_database.path.display());
    Ok(())
}
