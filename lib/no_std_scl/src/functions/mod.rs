pub mod base;
use crate::format;

mod bindings
{
    unsafe extern "C"
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
pub fn exec(pathname: &[u8], argv: *const *const u8, envp: *const *const u8) -> isize { unsafe { bindings::exec(pathname.as_ptr(), argv, envp) } }

pub struct Exec<'buf>
{
    pub pathname: &'buf [u8],
    pub argv: &'buf [*const u8],
    pub envp: *const *const u8,
}

impl<'a> Exec<'a>
{
    pub fn new(pathname: &'a [u8], argv: &'a [*const u8]) -> Self
    {
        Exec
        {
            pathname,
            argv,
            envp: core::ptr::null(), // for now, let's not use envp
        }
    }

    pub fn exec(&self) -> Result<(), &[u8]>
    {
        unsafe
        {
            return match bindings::fork()
            {
                0 =>
                {
                    if bindings::exec(self.pathname.as_ptr(), self.argv.as_ptr(), self.envp) == -1
                    {
                        return Err(self.pathname);
                    }
                    return Ok(());
                }
                -1 =>  Err(self.pathname),
                _ =>  Ok(()),
            }
        };
    }
}

#[macro_export]
macro_rules! format
{
    ($($arg:expr),*) =>
    {{
        let mut buffer = Vec::new();

        $(
            let bytes = $arg;
            for &byte in bytes
            {
                buffer.push(byte);
            }
        )*

        crate::functions::write(buffer.as_slice());
    }};
}

#[allow(unused)]
#[inline(always)]
pub fn read(text: &[u8]) -> Vec
{
    format!(text);

    let mut input = Vec::new();
    unsafe {
        bindings::read(0, input.data.as_mut_ptr(), input.cap);
    }

    unsafe { input.len = bindings::read(0, input.data.as_mut_ptr(), input.cap) as usize; }

    input
}

pub struct Vec
{
    pub data: [u8; 256],
    pub len: usize,
    pub cap: usize,
}

impl Vec
{
    pub fn new() -> Self
    {
        Vec
        {
            data: [0u8; 256], // Define o tamanho máximo do buffer
            len: 0,
            cap: 256,
        }
    }

    pub fn push(&mut self, byte: u8)
    {
        if self.len < self.cap
        {
            self.data[self.len] = byte;
            self.len += 1;
        }
    }

    pub fn as_slice(&self) -> &[u8]
    {
        &self.data[..self.len]
    }

    pub fn as_ref(&self) -> &[u8]
    {
        &self.data[..self.len]
    }

    pub fn len(&self) -> usize
    {
        self.len
    }
}

pub struct VecIterator<'a>
{
    vec: &'a Vec,
    index: usize,
}

impl<'a> Iterator for VecIterator<'a>
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.index < self.vec.len
        {
            let result = self.vec.data[self.index];
            self.index += 1;
            Some(result)
        }
        else { None }
    }
}

impl<'a> IntoIterator for &'a Vec
{
    type Item = u8;
    type IntoIter = VecIterator<'a>;

    fn into_iter(self) -> Self::IntoIter
    {
        VecIterator { vec: self, index: 0 }
    }
}