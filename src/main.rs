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

    let pid = fork();
    if pid == 0
    {
        let argv: [*const u8; 3] =
        [
            b"/bin/ls\0".as_ptr(),
            b"-l\0".as_ptr(),
            core::ptr::null(),
        ];
        if exec(b"/bin/ls\0", argv.as_ptr(), core::ptr::null()) < 0
        {
            format!(b"exec failed\n");
            exit(1);
        }
    }
    else
    {
        exit(0);
    }

    loop {}
}