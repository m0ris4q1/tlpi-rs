#![allow(non_camel_case_types)]
pub type c_char = i8;
pub type c_int = i32;
pub type c_void = std::ffi::c_void;

pub type mode_t = u32;
pub type size_t = usize;
pub type ssize_t = isize;

pub const EXIT_SUCCESS: c_int = 0;
pub const EXIT_FAILURE: c_int = 1;

pub const O_RDONLY: c_int = 0;
pub const O_WRONLY: c_int = 1;
pub const O_RDWR: c_int = 2;

pub const O_CREAT: c_int = 64;
pub const O_TRUNC: c_int = 512;

// errno.h
pub const EINVAL: c_int = 22;

extern "C" {
    pub fn __errno_location() -> *mut c_int;
    pub fn _exit(status: c_int) -> !;

    pub fn open(path: *const c_char, oflag: c_int, ...) -> c_int;
    pub fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    pub fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    pub fn close(fd: c_int) -> c_int;

    pub fn strerror(n: c_int) -> *mut c_char;
}
