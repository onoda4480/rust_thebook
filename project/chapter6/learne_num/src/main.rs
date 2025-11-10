#![allow(unused)]
fn main() {
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
//上記は`None`がカバーされていないためエラーを引き起こす。
//マッチは包括的なので、すべての可能性をカバーする必要がある。
let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
println!("{:?}, {:?}, {:?}", six, none, five);
}