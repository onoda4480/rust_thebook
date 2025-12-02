#![allow(unused)]
fn main() {
    let address = 0x012345usize;
    let r = address as *const i32;
    //メモリが確保されていないアドレスを参照しようとしているため、
    //そのアドレスにデータがある可能性もあるし、ない可能性もあるので、
    //安全ではない操作として扱われる
}
