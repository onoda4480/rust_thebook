#![allow(unused)]
fn main() {
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
//prelude(初期化処理)にも含まれているので、明示的にuseしなくても使える
//Noneは明示的にOption<好きな型>(例:Option<i32>)で定義しないといけない
// Stringなのかi32なのかどんなNoneかがわからないから
// Some(5)やSome("a string")は中身があるので型推論でわかる
}