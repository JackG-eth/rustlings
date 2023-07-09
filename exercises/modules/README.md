# Modules

In this section we'll give you an introduction to Rust's module system.

## Further information

- [The Module System](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

- Can pass in multiple modules in one use case.

// TODO: Complete this use statement // after get compiler analyser working
use std::time::{SystemTime,UNIX_EPOCH};

fn main() {
match SystemTime::now().duration*since(UNIX_EPOCH) {
Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
Err(*) => panic!("SystemTime before UNIX EPOCH!"),
}
}
