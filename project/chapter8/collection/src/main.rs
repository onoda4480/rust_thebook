#![allow(unused)]
fn main() {
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);

println!("{}", s);
//format!は所有権を取らないので、s1, s2, s3は有効
println!("{}", s1);
println!("{}", s2);
println!("{}", s3);
}