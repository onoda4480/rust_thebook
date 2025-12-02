unsafe extern "C" {
    fn abs(input: i32) -> i32;
    // C標準ライブラリのabs関数を宣言
    //thebookだとunsafeなしで宣言していたが、
    //Rust 1.63以降はunsafeが必要らしい
}

fn main() {
    unsafe {
        // -3の絶対値は、Cによると{}
        // unsafeブロック内で呼び出す必要がある
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
