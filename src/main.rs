extern "C" {
    fn printVersion();
}

fn main() {
    println!("Cool");
    unsafe {
        printVersion();
    }
}
