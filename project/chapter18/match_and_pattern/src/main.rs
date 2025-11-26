#![allow(unused)]
fn main() {
    //let式もパターンマッチを使って値を変数に束縛している。
    let a = 5;
    let (x, y, z) = (1, 2, 3);
    println!("a: {}", a);
    println!("x: {}, y: {}, z: {}", x, y, z);
    //以下はコンパイルエラーになる例
    //let (x, y) = (1, 2, 3);
}