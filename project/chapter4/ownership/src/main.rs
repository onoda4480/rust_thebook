fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let s = String::from("hello world");
    let word_end = first_word(&s);

    println!("最初の単語の終わり: {}", word_end);  // 5
    println!("最初の単語: {}", &s[0..word_end]);  // hello
}
