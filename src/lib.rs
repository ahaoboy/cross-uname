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

impl Info {
    pub fn new() -> io::Result<Self> {
        fn exec(arg: &str) -> String {
            let out = std::process::Command::new("uname")
                .arg(arg)
                .output()
                .unwrap()
                .stdout;
            String::from_utf8(out).unwrap().trim().to_string()
        }

        let sysname = exec("-s");
        let nodename = exec("-n");
        let release = exec("-r");
        let version = exec("-v");
        let machine = exec("-m");

        Ok(Info {
            sysname,
            nodename,
            release,
            version,
            machine,
        })
    }
}
