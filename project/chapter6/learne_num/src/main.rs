#![allow(unused)]
fn main() {
#[derive(Debug)]
enum UsState {
   Alabama,
   Alaska,
}

enum Coin {
   Penny,
   Nickel,
   Dime,
   Quarter(UsState),
}
//let coin = Coin::Penny;
let coin = Coin::Quarter(UsState::Alaska);
let mut count = 0;
match coin {
    // {:?}州のクォーターコイン
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
}