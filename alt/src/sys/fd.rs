use crate::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};
use std::cmp;
use std::io;

const READ_LIMIT: usize = libc::ssize_t::MAX as usize;

pub struct FileDesc(OwnedFd);

impl FileDesc {
    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        let num_read = unsafe {
            libc::read(
                self.as_raw_fd(),
                buf.as_mut_ptr() as *mut libc::c_void,
                cmp::min(buf.len(), READ_LIMIT),
            )
        };
        if num_read == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(num_read as usize)
        }
    }
}

impl AsRawFd for FileDesc {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl FromRawFd for FileDesc {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self(FromRawFd::from_raw_fd(fd))
    }
}
