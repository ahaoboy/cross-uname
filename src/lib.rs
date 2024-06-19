use std::io;

#[cfg(not(windows))]
mod unix;

#[cfg(windows)]
mod win;

#[derive(Debug, Clone)]
pub struct Info {
    pub sysname: String,
    pub nodename: String,
    pub release: String,
    pub version: String,
    pub machine: String,
}

pub fn uname() -> io::Result<Info> {
    Info::new()
}
