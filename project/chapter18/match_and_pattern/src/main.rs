fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {}", second)
        },
    }
}
//..を使うのは明確しないといけない
//どの値がマッチしてどの値が無視されるべきかが不明瞭なら
//コンパイラはエラー