#![allow(warnings)]
pub mod WCDBBridging;
pub mod bridged;

pub use WCDBBridging::*;

extern "C" {
    fn printVersion();
}

pub fn print_version() {
    unsafe {
        printVersion();
    }
}
