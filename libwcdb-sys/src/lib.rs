#![allow(warnings)]
mod WCDBBridging;

pub use WCDBBridging::*;

extern "C" {
    fn printVersion();
}

pub fn print_version() {
    unsafe {
        printVersion();
    }
}