#![allow(unused)]
fn main() {
use std::collections::HashMap;
//タプルのベクタに対してcollectメソッドを呼び出すことで、
// ハッシュマップを生成できる:
let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
//ベクタのように、ハッシュマップは均質です: 
// キーは全て同じ型でなければならず、 
// 値も全て同じ型でなければなりません。
}