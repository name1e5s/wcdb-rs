fn main() {
    println!("Cool");
    libwcdb_sys::print_version();
    let db = wcdb::core::database::Database::open("/tmp/test.db").unwrap();
    println!("{:?}", db.error());
}
