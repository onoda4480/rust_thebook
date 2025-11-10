#![allow(unused)]
fn main() {
//let some_u8_value = Some(0u8);
let some_u8_value = Some(3u8);
if let Some(3) = some_u8_value {
    println!("three");
}
//if letではmatchであった強制された包括性チェックは働かない
//matchかif letかの選択は、 特定の場面でどんなことをしたいかと簡潔性を得ることが
// 包括性チェックを失うのに適切な代償となるかによる。
}