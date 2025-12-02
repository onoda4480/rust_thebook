#![allow(unused)]
fn main() {
    use std::slice;

    let address = 0x012345usize;
    let r = address as *mut i32;

    let slice = unsafe { slice::from_raw_parts_mut(r, 10000) };
    //sliceを有効なスライスであるかのように使用しようとすると、未定義動作が発生します
}
