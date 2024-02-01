use crate::sys::fs as fs_imp;
use std::io::{self, Read};
use std::path::Path;

pub struct File {
    inner: fs_imp::File,
}

pub struct OpenOptions(fs_imp::OpenOptions);

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (&*self).read(buf)
    }
}

impl Read for &File {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl OpenOptions {
    pub fn new() -> Self {
        OpenOptions(fs_imp::OpenOptions::new())
    }

    pub fn read(&mut self, read: bool) -> &mut Self {
        self.0.read(read);
        self
    }

    pub fn mode(&mut self, mode: u32) -> &mut Self {
        self.0.mode(mode);
        self
    }

    pub fn open<P: AsRef<Path>>(&self, path: P) -> io::Result<File> {
        self._open(path.as_ref())
    }

    fn _open(&self, path: &Path) -> io::Result<File> {
        fs_imp::File::open(path, &self.0).map(|inner| File { inner })
    }
}
