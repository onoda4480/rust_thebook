pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
//ジェネリクスでトレイトを定義
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
//例
// Counter に対して複数の実装が可能！
// impl Iterator<u32> for Counter {
//     fn next(&mut self) -> Option<u32> { /* ... */ }
// }

// impl Iterator<String> for Counter {
//     fn next(&mut self) -> Option<String> { /* ... */ }
// }

// impl Iterator<char> for Counter {
//     fn next(&mut self) -> Option<char> { /* ... */ }
// }
//ジェネリクスだと使う時に混乱する
//そのため関数型の方が明確
//// ❌ どの Iterator を使えばいい？
// counter.next()
// //      ^^^^ Iterator<u32>? Iterator<String>? Iterator<char>?
// //           型注釈が必要！

// // 型注釈が必要
// let x: Option<u32> = counter.next();
// // または
// Iterator::<u32>::next(&mut counter);
