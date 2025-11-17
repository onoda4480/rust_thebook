fn prints_and_returns_10(a: i32) -> i32 {
    //{}という値を得た
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}

//cargo test -- --nocaptureで実行すると
//テスト成功時のprintln!の出力が見れる
//cargo test -- --nocapture --test-threads=1だと並列数を1に制限することで
//出力が混ざらないようにしている