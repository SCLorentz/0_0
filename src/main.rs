#![no_std]
#![no_main]

#[macro_use]
mod functions;
use functions::{exit, fork, exec};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { exit(1) }

#[no_mangle]
pub extern "C" fn _start() -> !
{
    format!(b"Hello, World!\n");

    match fork()
    {
        0 => child(),
        _ => format!(b"Parent process\n"),
    }
    
    exit(0);
}

fn child()
{
    let argv: [*const u8; 4] =
    [
        b"/bin/sh\0".as_ptr(),
        b"-c\0".as_ptr(),
        "neofetch     #".as_bytes().as_ptr(),
        core::ptr::null(),
    ];
    if exec(b"/bin/sh\0", argv.as_ptr(), core::ptr::null()) < 0
    {
        format!(b"exec failed\n");
        exit(1);
    }
}