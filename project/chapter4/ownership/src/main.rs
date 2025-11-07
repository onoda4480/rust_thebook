fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    // ✅ 方法1: Stringの参照（自動的に&strに変換される）
    let word = first_word(&my_string);
    println!("方法1: {}", word);

    // ✅ 方法2: Stringのスライス全体
    let word = first_word(&my_string[..]);
    println!("方法2: {}", word);

    // ✅ 方法3: Stringの部分スライス
    let word = first_word(&my_string[0..5]);
    println!("方法3: {}", word);

    let my_string_literal = "hello world";

    // ✅ 方法4: 文字列リテラル（それ自体が&str）
    let word = first_word(my_string_literal);
    println!("方法4: {}", word);

    // ✅ 方法5: 文字列リテラルのスライス
    let word = first_word(&my_string_literal[..]);
    println!("方法5: {}", word);
}
