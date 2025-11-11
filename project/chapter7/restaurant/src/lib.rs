#![allow(unused)]
fn main() {
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
    // （略）
    Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    // （略）
    Ok(())
}
}