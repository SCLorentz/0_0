#![no_std]
#![no_main]

extern crate no_std_scl;
use no_std_scl::{format, functions::{self, Exec, Vec, exit}};

#[no_mangle]
pub fn main()
{
    format!(b"Hello, world!\n");

    let args = [
        b"ls\0".as_ptr(),
        b"-l\0".as_ptr(),
        b"/\0".as_ptr(),
    ];
    let exec = Exec::new(b"/bin/ls\0",  &args);
    exec.exec().unwrap_or_else(|err| {
        format!(b"Exec failed: {}\n", err);
        exit(1);
    });

    format!(b"Exec succeeded\n");
}