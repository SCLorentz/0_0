#[unsafe(no_mangle)]
pub extern "C" fn memset(ptr: *mut u8, value: u8, num: usize) -> *mut u8
{
    unsafe
    {
        let mut i = 0;
        while i < num
        {
            *ptr.add(i) = value;
            i += 1;
        }
    }
    ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn memcpy(dest: *mut u8, src: *const u8, num: usize) -> *mut u8
{
    unsafe
    {
        let mut i = 0;
        while i < num
        {
            *dest.add(i) = *src.add(i);
            i += 1;
        }
    }
    dest
}