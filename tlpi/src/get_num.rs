use anyhow::Result;
use libc::c_char;
use std::ffi::{CStr, CString};
use std::io::{self, Write};
use std::process;
use std::ptr;

pub const GN_NONNEG: u32 = 0o01;
pub const GN_GT_0: u32 = 0o02;

pub const GN_ANY_BASE: u32 = 0o100;
pub const GN_BASE_8: u32 = 0o200;
pub const GN_BASE_16: u32 = 0o400;

fn gn_fail(fname: &str, msg: &str, arg: &str, name: &str) -> Result<()> {
    let mut stderr = io::stderr().lock();
    stderr.write_fmt(format_args!("{} error", fname))?;
    if !name.is_empty() {
        stderr.write_fmt(format_args!(" (in {})", name))?;
    }
    stderr.write_fmt(format_args!(": {}\n", msg))?;
    if !arg.is_empty() {
        stderr.write_fmt(format_args!("        offending text: {}\n", arg))?;
    }
    stderr.flush()?;

    process::exit(libc::EXIT_FAILURE);
}

fn get_num(fname: &str, arg: &str, flags: u32, name: &str) -> Result<i64> {
    let arg_in_c = CString::new(arg).unwrap();
    let mut endp: *mut c_char = ptr::null_mut();
    let base = if flags & GN_ANY_BASE > 0 { 0 }
        else if flags & GN_BASE_8 > 0 { 8 }
        else if flags & GN_BASE_16 > 0 { 16 }
        else { 10 };

    unsafe {
        let errno = libc::__errno_location();
        *errno = 0;
    }

    let result = unsafe {
        libc::strtol(
            arg_in_c.as_ptr(),
            &mut endp as *mut *mut c_char,
            base,
        )
    };

    let errno = unsafe {
        let errno = libc::__errno_location();
        *errno
    };
    if errno != 0 {
        gn_fail(fname, "strtol() failed", arg, name)?;
    }

    let rest = unsafe { CStr::from_ptr(endp) };
    if !rest.is_empty() {
        gn_fail(fname, "nonnumeric characters", arg, name)?;
    }

    if flags & GN_NONNEG > 0 && result < 0 {
        gn_fail(fname, "negative value not allowed", arg, name)?;
    }

    if flags & GN_GT_0 > 0 && result <= 0 {
        gn_fail(fname, "value must be > 0", arg, name)?;
    }

    Ok(result)
}

pub fn get_long(arg: &str, flags: u32, name: &str) -> Result<i64> {
    get_num("get_long", arg, flags, name)
}

pub fn get_int(arg: &str, flags: u32, name: &str) -> Result<i32> {
    let result = get_num("get_int", arg, flags, name)?;
    if result > i32::MAX as i64 || result < i32::MIN as i64 {
        gn_fail("get_int", "integer out of range", arg, name)?;
    }
    Ok(result.try_into().unwrap())
}
