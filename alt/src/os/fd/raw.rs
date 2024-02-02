pub type RawFd = libc::c_int;

pub trait AsRawFd {
    fn as_raw_fd(&self) -> RawFd;
}

pub trait FromRawFd {
    /// # Safety
    ///
    /// The `fd` passed in must be an [owned file descriptor][io-safety];
    /// in particular, it must be open.
    unsafe fn from_raw_fd(fd: RawFd) -> Self;
}

pub trait IntoRawFd {
    fn into_raw_fd(self) -> RawFd;
}
