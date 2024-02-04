use crate::os::fd::{AsRawFd, FromRawFd, RawFd};
use crate::sys::fd::FileDesc;
use libc::{c_int, mode_t, off64_t};
use std::ffi::{CStr, CString};
use std::io::{self, SeekFrom};
use std::path::Path;

pub struct File(FileDesc);

pub struct OpenOptions {
    read: bool,
    write: bool,
    truncate: bool,
    create: bool,
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
        let flags = opts.get_access_mode()?
            | opts.get_creation_mode()?;
        let fd = unsafe {
            libc::open(path.as_ptr(), flags, opts.mode as c_int)
        };
        if fd == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(File(unsafe { FileDesc::from_raw_fd(fd) }))
        }
    }

    pub fn close(self) -> io::Result<()> {
        self.0.close()
    }
    
    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    #[inline]
    pub fn flush(&self) -> io::Result<()> {
        Ok(())
    }

    pub fn seek(&self, pos: SeekFrom) -> io::Result<u64> {
        let (whence, pos) = match pos {
            SeekFrom::Start(off) => (libc::SEEK_SET, off as i64),
            SeekFrom::End(off) => (libc::SEEK_END, off),
            SeekFrom::Current(off) => (libc::SEEK_CUR, off),
        };
        let n = unsafe {
            libc::lseek64(self.as_raw_fd(), pos as off64_t, whence)
        };
        if n == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(n as u64)
        }
    }
}

impl AsRawFd for File {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl OpenOptions {
    pub fn new() -> Self {
        OpenOptions {
            read: false,
            write: false,
            truncate: false,
            create: false,
            mode: 0o666,
        }
    }

    pub fn read(&mut self, read: bool) {
        self.read = read;
    }

    pub fn write(&mut self, write: bool) {
        self.write = write;
    }

    pub fn truncate(&mut self, truncate: bool) {
        self.truncate = truncate;
    }

    pub fn create(&mut self, create: bool) {
        self.create = create;
    }

    pub fn mode(&mut self, mode: u32) {
        self.mode = mode as mode_t;
    }

    fn get_access_mode(&self) -> io::Result<c_int> {
        match (self.read, self.write, ) {
            (true, false, ) => Ok(libc::O_RDONLY),
            (false, true, ) => Ok(libc::O_WRONLY),
            (true, true, ) => Ok(libc::O_RDWR),
            (false, false, ) =>
                Err(io::Error::from_raw_os_error(libc::EINVAL)),
        }
    }

    fn get_creation_mode(&self) -> io::Result<c_int> {
        match (self.write, ) {
            (true, ) => {}
            (false, ) => {
                if self.truncate || self.create {
                    return Err(io::Error::from_raw_os_error(libc::EINVAL));
                }
            }
        }

        Ok(match (self.create, self.truncate, ) {
            (false, false, ) => 0,
            (true, false, ) => libc::O_CREAT,
            (false, true, ) => libc::O_TRUNC,
            (true, true, ) => libc::O_CREAT | libc::O_TRUNC,
        })
    }
}

impl Default for OpenOptions {
    fn default() -> Self {
        OpenOptions::new()
    }
}
