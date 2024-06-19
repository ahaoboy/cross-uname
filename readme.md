# cross-uname

Get name and information about current kernel by uname command

```rust
use cross_uname::uname;

fn main() {
    let info = uname().unwrap();
    // Print the hostname
    println!("{}", info.nodename);
    // Print everything
    println!("{:?}", info);
}

```