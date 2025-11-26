#![allow(unused)]
fn main() {
let x = 1;

match x {
    // 1か2
    // 「|」はorを意味する
    1 | 2 => println!("one or two"),
    // 3
    3 => println!("three"),
    // なんでも
    _ => println!("anything"),
}
}