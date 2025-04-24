#![no_std]
#![no_main]

extern crate no_std_scl;
use no_std_scl::format;
use no_std_scl::functions;
//use no_std_scl::functions::exit;

#[no_mangle]
pub fn main() {
    format!(b"Hello, world!\n");
}