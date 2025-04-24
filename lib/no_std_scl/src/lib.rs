#![crate_name = "no_std_scl"]
#![no_std]
#![no_main]

unsafe extern "Rust"
{
    fn main();
}

#[macro_use]
pub mod functions;
use functions::exit;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { exit(1) }

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> !
{
    unsafe { main(); }
    exit(0);
}