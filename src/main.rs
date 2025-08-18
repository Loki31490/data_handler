use std::path::PathBuf;

mod db_handler;

fn main() {
    let path = PathBuf::from("/home/baptiste/Bureau/Rust/data_handler/data");
    let database = db_handler::Database::new(path);

    println!("{:?}", database.path);
}
