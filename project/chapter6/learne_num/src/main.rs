#![allow(unused)]
fn main() {
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        // メソッド本体はここに定義される
    }
}

let m = Message::Write(String::from("hello"));
m.call();
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
