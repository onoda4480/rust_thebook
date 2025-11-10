#![allow(unused)]
fn main() {
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
}
//以下のように定義することもできるがあまりにもめんどくさいのでenumを使うほうが良い
// #![allow(unused)]
// fn main() {
// struct QuitMessage; // ユニット構造体
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // タプル構造体
// struct ChangeColorMessage(i32, i32, i32); // タプル構造体
// }
