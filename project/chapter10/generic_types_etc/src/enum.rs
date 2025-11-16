#![allow(unused)]
fn pub enum_test() {
// ジェネリック型を使ったenumの定義例
enum Option<T> {
    Some(T),
    None,
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}
}