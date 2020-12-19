//! Global initialization and retrieval of command line arguments.
//!
//! On some platforms these are stored during runtime startup,
//! and on some they are retrieved from the system on demand.

#![allow(dead_code)] // runtime init functions not used during testing

use crate::ffi::OsString;
use crate::marker::PhantomData;
use crate::vec;

/// One-time global initialization.
pub unsafe fn init(argc: isize, argv: *const *const u8) {
    imp::init(argc, argv)
}

/// One-time global cleanup.
pub unsafe fn cleanup() {
    imp::cleanup()
}

/// Returns the command line arguments
pub fn args() -> Args {
    imp::args()
}

pub struct Args {
    iter: vec::IntoIter<OsString>,
    _dont_send_or_sync_me: PhantomData<*mut ()>,
}

impl Args {
    pub fn inner_debug(&self) -> &[OsString] {
        self.iter.as_slice()
    }
}

impl Iterator for Args {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl ExactSizeIterator for Args {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl DoubleEndedIterator for Args {
    fn next_back(&mut self) -> Option<OsString> {
        self.iter.next_back()
    }
}

mod imp {
    use super::Args;
    use crate::ffi::{CStr, OsString};
    use crate::marker::PhantomData;
    use crate::ptr;
    use crate::sync::atomic::{AtomicIsize, AtomicPtr, Ordering};
    use crate::sys::os_str::OsStringExt;

    use crate::sys_common::mutex::StaticMutex;

    static ARGC: AtomicIsize = AtomicIsize::new(0);
    static ARGV: AtomicPtr<*const u8> = AtomicPtr::new(ptr::null_mut());
    // We never call `ENV_LOCK.init()`, so it is UB to attempt to
    // acquire this mutex reentrantly!
    static LOCK: StaticMutex = StaticMutex::new();

    #[inline(always)]
    pub unsafe fn init(argc: isize, argv: *const *const u8) {
        let _guard = LOCK.lock();
        ARGC.store(argc, Ordering::Relaxed);
        ARGV.store(argv as *mut _, Ordering::Relaxed);
    }

    pub unsafe fn cleanup() {
        let _guard = LOCK.lock();
        ARGC.store(0, Ordering::Relaxed);
        ARGV.store(ptr::null_mut(), Ordering::Relaxed);
    }

    pub fn args() -> Args {
        Args { iter: clone().into_iter(), _dont_send_or_sync_me: PhantomData }
    }

    fn clone() -> Vec<OsString> {
        unsafe {
            let _guard = LOCK.lock();
            let argc = ARGC.load(Ordering::Relaxed);
            let argv = ARGV.load(Ordering::Relaxed);
            (0..argc)
                .map(|i| {
                    let cstr = CStr::from_ptr(*argv.offset(i) as *const i8);
                    OsStringExt::from_vec(cstr.to_bytes().to_vec())
                })
                .collect()
        }
    }
}
