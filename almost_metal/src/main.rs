#![no_std]
#![no_main]

extern crate no_std_scl;
use no_std_scl::{format, functions::{self, read, Vec}};

#[no_mangle]
pub fn main()
{
    format!(b"Hello, world!\n");

    let a = read(b"input ");
}