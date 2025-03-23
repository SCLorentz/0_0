#![no_std]
#![no_main]

#[macro_use]
mod functions;
use functions::{exit_s as exit, read_s as read};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { exit(1) }

#[no_mangle]
pub extern "C" fn _start() -> !
{
    format!(b"Hello, World!\n");
    format!(b"your input was: ", &read(b"input: "));

    exit(0)
}