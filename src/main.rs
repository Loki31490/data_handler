use std::path::PathBuf;

mod db_handler;

fn main() {
    env_logger::init();
    //trace!("trace");
    //info!("info");
    //warn!("warn");
    //error!("error");

    let path = PathBuf::from("/home/baptiste/Bureau/Rust/data_handler/data/my_data");
    let database = db_handler::Database::from(path);
    database.print_db();
}
