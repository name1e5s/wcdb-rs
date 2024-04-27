use wcdb::{core::database::ConfigPriority, winq::identifiers::bind::BindParameter};

fn main() {
    println!("Cool");
    libwcdb_sys::print_version();
    let db = wcdb::core::database::Database::create("/tmp/test.db").unwrap();
    println!("{:?}", db.error());
    println!("{:?}", db.get_path());
    println!("{:?}", db.can_open());
    db.set_config(
        "test",
        |_| {
            println!("New handle");
            true
        },
        |_| {
            println!("New handle closed");
            true
        },
        ConfigPriority::High,
    )
    .unwrap();
    let handle = db.get_handle().unwrap();
    handle
        .exec_sql(c"create table if not exists a(int b)")
        .unwrap();
    drop(handle);
    db.close_with_callback(|| {
        println!("Database closed");
    });
    let exp = BindParameter::_1();
    println!("{:?}", exp.description());
}
