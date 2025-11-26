#![allow(unused)]
fn main() {
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
    //enumerateメソッドを使用してイテレータを改造し、
    //値とその値のイテレータでの添え字をタプルに配置して生成
    //enumerateの最初の呼び出しは、タプル(0, 'a')を生成
}
}