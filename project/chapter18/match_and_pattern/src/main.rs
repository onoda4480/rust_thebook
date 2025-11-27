#![allow(unused)]
fn main() {
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        // 何らかの数値: {}, {}, {}
        println!("Some numbers: {}, {}, {}", first, third, fifth)
    },
}
}