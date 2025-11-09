#![allow(unused)]
fn main() {
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Color型を引数に取る関数は、Pointを引数に取ることはできません。
let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
}