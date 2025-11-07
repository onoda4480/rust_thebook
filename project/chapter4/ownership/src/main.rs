fn main() {
    // String型
    let my_string = String::from("hello world");

    // 型を確認する小技
    // my_stringの型は？
    let _type_check: String = my_string.clone();
    // ↑ String型だと確認できる

    // 文字列リテラルの型
    let my_literal = "hello world";

    // 型を確認
    let _type_check: &str = my_literal;
    // ↑ &str型だと確認できる

    println!("my_string の型: String");
    println!("my_literal の型: &str");

    // だから...
    println!("\n--- 関数呼び出し ---");

    // Stringは&を付ける
    first_word(&my_string);
    println!("✅ first_word(&my_string) - &を付けた");

    // &strはそのまま
    first_word(my_literal);
    println!("✅ first_word(my_literal) - そのまま");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
