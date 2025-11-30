fn main() {
    let x = 4;
    let y = false;

    match x {
        // はい
        4 | 5 | 6 if y => println!("yes"),
        // 4、5、6のいずれかであり、かつyがtrueの場合にマッチします。
        //(4 | 5 | 6) if y =>です。
        //決して4 | 5 | (6 if y) => ではない

        // いいえ
        _ => println!("no"),
    }
}
