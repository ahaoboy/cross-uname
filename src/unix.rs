extern crate libc;
use std::ffi::CStr;
use libc::{utsname, c_char};

impl Info {
    pub fn new() -> io::Result<Self> {
        let mut n = unsafe { std::mem::zeroed() };
        let r = unsafe { libc::uname(&mut n) };
        if r == 0 {
            Ok(From::from(n))
        } else {
            Err(io::Error::last_os_error())
        }
    }
}

#[inline]
fn to_cstr(buf: &[c_char]) -> &CStr {
    unsafe { CStr::from_ptr(buf.as_ptr()) }
}

impl From<utsname> for Info {
    fn from(x: utsname) -> Self {
        Info {
            sysname: to_cstr(&x.sysname[..]).to_string_lossy().into_owned(),
            nodename: to_cstr(&x.nodename[..]).to_string_lossy().into_owned(),
            release: to_cstr(&x.release[..]).to_string_lossy().into_owned(),
            version: to_cstr(&x.version[..]).to_string_lossy().into_owned(),
            machine: to_cstr(&x.machine[..]).to_string_lossy().into_owned(),
            _priv: (),
        }
    }
}