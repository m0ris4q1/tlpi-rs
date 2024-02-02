use anyhow::Result;
use libc::EXIT_FAILURE;
use std::env;
use std::ffi::CStr;
use std::fmt::Arguments;
use std::io::{self, Write};
use std::process;
use super::errors::{ERROR_NAMES, MAX_ERROR_INDEX};

fn terminate(use_exit3: bool) -> ! {
    if let Ok(var) = env::var("EF_DUMPCORE") {
        if !var.is_empty() {
            process::abort();
        }
    }

    if use_exit3 {
        process::exit(libc::EXIT_FAILURE);
    } else {
        unimplemented!("_exit(2)");
        /*
        unsafe {
            libc::_exit(libc::EXIT_FAILURE);
        }
        */
    }
}

fn output_error(
    err: Option<i32>,
    flush_stdout: bool,
    args: Arguments,
) -> Result<()> {
    if flush_stdout {
        io::stdout().flush()?;
    }

    let mut stderr = io::stderr().lock();
    stderr.write_all(b"ERROR")?;

    if let Some(err) = err {
        stderr.write_all(b" [")?;
        {
            let err: usize = err.try_into()?;
            if 0 < err && err <= MAX_ERROR_INDEX {
                stderr.write_all(ERROR_NAMES[err].as_bytes())?;
            } else {
                stderr.write_all(b"?UNKNOWN?")?;
            }
        }
        stderr.write_all(b" ")?;
        unsafe {
            let err_msg = libc::strerror(err);
            let err_msg = CStr::from_ptr(err_msg);
            stderr.write_all(err_msg.to_str()?.as_bytes())?;
        }
        stderr.write_all(b"]")?;
    } else {
        stderr.write_all(b":")?;
    }

    stderr.write_all(b" ")?;
    stderr.write_fmt(args)?;
    stderr.write_all(b"\n")?;
    stderr.flush()?;

    Ok(())
}

pub fn err_msg(args: Arguments) -> Result<()> {
    let saved_errno = unsafe {
        let errno = libc::__errno_location();
        *errno
    };

    output_error(Some(saved_errno), true, args)?;

    unsafe {
        let errno = libc::__errno_location();
        *errno = saved_errno;
    }

    Ok(())
}

pub fn err_exit3(args: Arguments) -> Result<()> {
    let errno = unsafe {
        let errno = libc::__errno_location();
        *errno
    };

    output_error(Some(errno), true, args)?;
    terminate(true);
}

pub fn err_exit2(args: Arguments) -> Result<()> {
    let errno = unsafe {
        let errno = libc::__errno_location();
        *errno
    };

    output_error(Some(errno), false, args)?;
    terminate(false);
}

pub fn err_exit_en(err_num: i32, args: Arguments) -> Result<()> {
    output_error(Some(err_num), true, args)?;
    terminate(true);
}

pub fn fatal(args: Arguments) -> Result<()> {
    output_error(None, true, args)?;
    terminate(true);
}

pub fn usage_err(args: Arguments) -> Result<()> {
    io::stdout().flush()?;

    let mut stderr = io::stderr().lock();
    stderr.write_all(b"Usage: ")?;
    stderr.write_fmt(args)?;
    stderr.flush()?;

    process::exit(EXIT_FAILURE);
}

pub fn cmd_line_err(args: Arguments) -> Result<()> {
    io::stdout().flush()?;

    let mut stderr = io::stderr().lock();
    stderr.write_all(b"Command-line usage error: ")?;
    stderr.write_fmt(args)?;
    stderr.flush()?;

    process::exit(EXIT_FAILURE);
}
