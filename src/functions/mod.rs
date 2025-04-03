pub mod base;
use crate::format;

mod bindings
{
    extern "C"
    {
        pub fn exit(code: u8) -> !;
        pub fn write(fd: u8, buf: *const u8, count: usize) -> isize;
        pub fn read(fd: u8, buf: *const u8, count: usize) -> isize;
        pub fn fork() -> isize;
        pub fn exec(pathname: *const u8, argv: *const *const u8, envp: *const *const u8) -> isize;
    }
}

#[inline(always)]
pub fn exit(code: u8) -> ! { unsafe { bindings::exit(code) } }

#[inline(always)]
pub fn write(text: &[u8]) -> isize { unsafe { bindings::write(1, text.as_ptr(), text.len() as usize) } }

#[inline(always)]
pub fn fork(func: impl Fn(), err: impl Fn()) -> isize
{
    match unsafe { bindings::fork() }
    {
        -1 => { err(); return -1 },
        0 => { func(); exit(0); }
        _ => 0
    }
}

#[inline(always)]
pub fn exec(pathname: &[u8], argv: *const *const u8, envp: *const *const u8) -> isize { unsafe { bindings::exec(pathname.as_ptr(), argv, envp) } }

#[macro_export]
macro_rules! format
{
    ($($arg:expr),*) =>
    {{
        let mut buffer = [0u8; 256];
        let mut index = 0;

        $(
            let bytes = $arg;
            if index + bytes.len() < buffer.len()
            {
                buffer[index..index + bytes.len()].copy_from_slice(bytes);
                index += bytes.len();
            }
        )*

        crate::functions::write(&buffer[..index]);
    }};
}

#[allow(unused)]
#[inline(always)]
pub fn read(text: &[u8]) -> [u8; 64]
{
    format!(text);
    let mut input = [0u8; 64];
    unsafe { bindings::read(0, input.as_mut_ptr(), input.len()); }

    input
}