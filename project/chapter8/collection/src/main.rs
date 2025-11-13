#![allow(unused)]
fn main() {
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {}", s2);
//push_strは参照を取るので、s2の所有権は奪わない‼️
}