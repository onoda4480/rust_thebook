pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
        //internal_adderは非公開関数なので、lib.rsの外からは呼び出せないが、
        //同じモジュール内であればテスト可能
    }
}
