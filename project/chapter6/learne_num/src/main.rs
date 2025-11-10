#![allow(unused)]
fn main() {
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
//i8とOption<i8>が異なる型なので、 足し合わせる方法がコンパイラにはわからないからエラー！！
}