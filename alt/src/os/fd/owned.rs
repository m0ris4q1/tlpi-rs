use std::mem::forget;
use super::raw::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

pub struct OwnedFd {
    fd: RawFd,
}


impl AsRawFd for OwnedFd {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

impl Drop for OwnedFd {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let _ = libc::close(self.fd);
        }
    }
}

impl FromRawFd for OwnedFd {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        assert_ne!(fd, u32::MAX as RawFd);
        // SAFETY: we just asserted that the value is in the valid range and
        // isn't `-1` (the only value bigger than `0xFF_FF_FF_FF` unsigned)
        Self { fd }
    }
}

impl IntoRawFd for OwnedFd {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        let fd = self.fd;
        forget(self);
        fd
    }
}
