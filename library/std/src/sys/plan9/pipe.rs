use crate::io::{self, IoSlice, IoSliceMut};
// use crate::mem;
use crate::sys::fd::FileDesc;
// use crate::sys::{cvt, cvt_r};
use crate::sys::cvt;

////////////////////////////////////////////////////////////////////////////////
// Anonymous pipes
////////////////////////////////////////////////////////////////////////////////

pub struct AnonPipe(FileDesc);

#[allow(dead_code)]
pub fn anon_pipe() -> io::Result<(AnonPipe, AnonPipe)> {
    let mut fds = [0; 2];

    cvt(unsafe { libc::pipe(fds.as_mut_ptr()) })?;

    let fd0 = FileDesc::new(fds[0]);
    let fd1 = FileDesc::new(fds[1]);
    fd0.set_cloexec()?;
    fd1.set_cloexec()?;
    Ok((AnonPipe(fd0), AnonPipe(fd1)))
}

impl AnonPipe {
    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }

    pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        self.0.read_vectored(bufs)
    }

    #[inline]
    pub fn is_read_vectored(&self) -> bool {
        self.0.is_read_vectored()
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        self.0.write_vectored(bufs)
    }

    #[inline]
    pub fn is_write_vectored(&self) -> bool {
        self.0.is_write_vectored()
    }

    #[allow(dead_code)]
    pub fn fd(&self) -> &FileDesc {
        &self.0
    }

    #[allow(dead_code)]
    pub fn into_fd(self) -> FileDesc {
        self.0
    }

    pub fn diverge(&self) -> ! {
         // match self.0 {}
         loop {}
    }

}

pub fn read2(_p1: AnonPipe, _v1: &mut Vec<u8>, _p2: AnonPipe, _v2: &mut Vec<u8>) -> io::Result<()> {
    // match p1.0 {}
    Ok(())
}
