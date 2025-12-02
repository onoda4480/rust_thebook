#![allow(unused)]
fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    //ポインタの生成だけでは安全だが,
    //ポインタが指している値にアクセスしようとすると未定義動作を引き起こす可能性がある.
    //そのためunsafeブロック内でのみ使用可能
}
