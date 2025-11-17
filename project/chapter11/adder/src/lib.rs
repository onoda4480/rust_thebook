pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
//cargo test one_hundredやcargo test add_two_and_twoのように
//特定のテストだけを実行できる
//cargo test addのようにでテスト関数名の一部を指定して実行することもできる
//上記の場合はaddで始まるテスト関数がすべて実行される