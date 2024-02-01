use crate::os::fd::{FromRawFd};
use crate::sys::fd::FileDesc;
use libc::{c_int, mode_t};
use std::ffi::{CStr, CString};
use std::io;
use std::path::Path;

pub struct File(FileDesc);

pub struct OpenOptions {
    read: bool,
    mode: mode_t,
}

impl File {
    pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> {
        let bytes = path.as_os_str().as_encoded_bytes();
        let path = CString::new(bytes)
            .expect("file name contained an unexpected NUL byte");
        File::open_c(&path, opts)
    }

    pub fn open_c(path: &CStr, opts: &OpenOptions) -> io::Result<File> {
        let flags = opts.get_access_mode()?;
        let fd = unsafe {
            libc::open(path.as_ptr(), flags, opts.mode as c_int)
        };
        if fd == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(File(unsafe { FileDesc::from_raw_fd(fd) }))
        }
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl OpenOptions {
    pub fn new() -> Self {
        OpenOptions {
            read: false,
            mode: 0o666,
        }
    }

    pub fn read(&mut self, read: bool) {
        self.read = read;
    }

    pub fn mode(&mut self, mode: u32) {
        self.mode = mode as mode_t;
    }

    fn get_access_mode(&self) -> io::Result<c_int> {
        match (self.read, ) {
            (true, ) => Ok(libc::O_RDONLY),
            (false, ) => Ok(libc::O_WRONLY),
        }
    }
}
