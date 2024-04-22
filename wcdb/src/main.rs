fn main() {
    println!("Cool");
    libwcdb_sys::print_version();
    let db = wcdb::core::database::Database::create("/tmp/test.db").unwrap();
    println!("{:?}", db.error());
    println!("{:?}", db.get_path());
    let handle = db.get_handle().unwrap();
}
