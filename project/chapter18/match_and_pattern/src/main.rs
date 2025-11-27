fn main() {
// こんにちは！
let s = Some(String::from("Hello!"));

if let Some(_s) = s {
    //アンダースコアで始まる未使用の変数は値を束縛し、値の所有権を奪う可能性がある
    // 文字列が見つかりました
    println!("found a string");
}

println!("{:?}", s);
}