#![allow(missing_docs, nonstandard_style)]

use crate::io::ErrorKind;

pub mod alloc;
pub mod args;
pub mod cmath;
#[path = "../unsupported/condvar.rs"]
pub mod condvar;
pub mod env;
pub mod fd;
#[path = "../unsupported/fs.rs"]
pub mod fs;
pub mod io;
#[path = "../unsupported/mutex.rs"]
pub mod mutex;
#[path = "../unsupported/net.rs"]
pub mod net;
pub mod os;
#[path = "../unix/path.rs"]
pub mod path;
pub mod pipe;
#[path = "../unsupported/process.rs"]
pub mod process;
#[path = "../unsupported/rwlock.rs"]
pub mod rwlock;
#[path = "../unsupported/stack_overflow.rs"]
pub mod stack_overflow;
pub mod stdio;
#[path = "../unsupported/thread.rs"]
pub mod thread;
#[path = "../unsupported/thread_local_key.rs"]
pub mod thread_local_key;
#[path = "../unsupported/time.rs"]
pub mod time;

mod common;
pub use common::*;

#[doc(hidden)]
pub trait IsMinusOne {
    fn is_minus_one(&self) -> bool;
}

macro_rules! impl_is_minus_one {
    ($($t:ident)*) => ($(impl IsMinusOne for $t {
        fn is_minus_one(&self) -> bool {
            *self == -1
        }
    })*)
}

impl_is_minus_one! { i8 i16 i32 i64 isize }

pub fn cvt<T: IsMinusOne>(t: T) -> crate::io::Result<T> {
    if t.is_minus_one() { Err(crate::io::Error::last_os_error()) } else { Ok(t) }
}

#[allow(dead_code)]
pub fn cvt_r<T, F>(mut f: F) -> crate::io::Result<T>
where
    T: IsMinusOne,
    F: FnMut() -> T,
{
    loop {
        match cvt(f()) {
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
            other => return other,
        }
    }
}

// On Unix-like platforms, libc::abort will unregister signal handlers
// including the SIGABRT handler, preventing the abort from being blocked, and
// fclose streams, with the side effect of flushing them so libc buffered
// output will be printed.  Additionally the shell will generally print a more
// understandable error message like "Abort trap" rather than "Illegal
// instruction" that intrinsics::abort would cause, as intrinsics::abort is
// implemented as an illegal instruction.
pub fn abort_internal() -> ! {
    unsafe { libc::abort() }
}
