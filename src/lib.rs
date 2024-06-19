use std::io;
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

fn exec(cmd: &str, arg: &str) -> String {
    let out = std::process::Command::new(cmd)
        .arg(arg)
        .output()
        .unwrap()
        .stdout;
    String::from_utf8(out).unwrap().trim().to_string()
}

impl Info {
    pub fn new() -> io::Result<Self> {
        let sysname = exec("uname", "-s");
        let nodename = exec("uname", "-n");
        let release = exec("uname", "-r");
        let version = exec("uname", "-v");
        let machine = exec("uname", "-m");

        Ok(Info {
            sysname,
            nodename,
            release,
            version,
            machine,
        })
    }
}
