pub mod base;

extern "C" { fn exit(code: u8) -> !; }
extern "C" { pub fn write(fd: u8, buf: *const u8, count: usize) -> isize; }
extern "C" { fn read(fd: u8, buf: *const u8, count: usize) -> isize; }

#[inline(always)]
pub fn exit_s(code: u8) -> ! { unsafe { exit(code) } }

#[inline(always)]
pub fn write_s(text: &[u8]) { unsafe { write(1, text.as_ptr(), text.len() as usize); } }


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

        crate::functions::write_s(&buffer[..index]);
    }};
}

#[inline(always)]
pub fn read_s(text: &[u8]) -> [u8; 64]
{
    format!(text);
    let mut input = [0u8; 64];
    unsafe { read(0, input.as_mut_ptr(), input.len()); }

    input
}