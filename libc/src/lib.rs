#[allow(non_camel_case_types)]
pub type c_char = i8;
pub type c_int = i32;
pub type c_void = std::ffi::c_void;

pub type mode_t = u32;
pub type size_t = usize;
pub type ssize_t = isize;

pub const O_RDONLY: c_int = 0;
pub const O_WRONLY: c_int = 1;

extern "C" {
    pub fn open(path: *const c_char, oflag: c_int, ...) -> c_int;
    pub fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
}
