#![allow(unused)]
fn main() {
use std::collections::HashMap;
let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point, try using them and
// see what compiler error you get!
// field_nameとfield_valueはこの時点で無効になる。試しに使ってみて
// どんなコンパイルエラーが出るか確認してみて！
//以下はアクセス方法が違うのでエラー
println!("{}", map["Favorite color"]);
println!("{}", map["Blue"]);

//これはmapに所有権が移動しているのでエラー
println!("{}", field_name);
println!("{}", field_value);
}