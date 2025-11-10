#![allow(unused)]
fn main() {
enum Coin {
   Penny,
   Nickel,
   Dime,
   Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        //メソッドがCoin::Pennyとともに呼び出されるたびに「Lucky penny!」と表示しつつ、 ブロックの最後の値、1を返す
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
}