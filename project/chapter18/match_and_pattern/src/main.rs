#![allow(unused)]
fn main() {
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
    //match式でx座標のみ処理し、 yとzフィールドの値は無視する
}
}