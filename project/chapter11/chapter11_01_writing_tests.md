# 第11章 まとめ (1/3): テストの書き方

## テストとは

**定義:** コードが期待通りに動作することを自動的に確認する仕組み

**目的:** バグを早期に発見し、コードの品質を保証する

---

## 基本的なテストの書き方

### テスト関数

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
```

**実行:**
```bash
cargo test
```

**出力:**
```
running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## assert! マクロ

### 基本構文

```rust
assert!(condition);
```

**意味:** 条件が `true` なら OK、`false` ならパニック

---

### 使用例

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }
}
```

---

## assert_eq! と assert_ne! マクロ

### assert_eq! - 等しいことを確認

```rust
assert_eq!(left, right);
```

**意味:** `left == right` なら OK

---

### assert_ne! - 異なることを確認

```rust
assert_ne!(left, right);
```

**意味:** `left != right` なら OK

---

### 使用例

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_does_not_add_three() {
        assert_ne!(5, add_two(2));
    }
}
```

---

## カスタムエラーメッセージ

### 基本構文

```rust
assert!(condition, "カスタムメッセージ");
assert_eq!(left, right, "カスタムメッセージ");
```

---

### 使用例

```rust
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}
```

**失敗時の出力:**
```
Greeting did not contain name, value was `Hello Carol!`
```

---

## パニックを期待するテスト

### #[should_panic] 属性

```rust
#[test]
#[should_panic]
fn test_panic() {
    panic!("This should panic");
}
```

**意味:** このテストはパニックするべき（パニックしたら成功）

---

### 使用例

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

---

### expected パラメータ

特定のパニックメッセージを期待する:

```rust
#[test]
#[should_panic(expected = "Guess value must be between 1 and 100")]
fn greater_than_100() {
    Guess::new(200);
}
```

**意味:** パニックメッセージに "Guess value must be between 1 and 100" が含まれることを期待

---

## Result<T, E> を使うテスト

### 基本構文

```rust
#[test]
fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}
```

**意味:**
- `Ok(())` を返せば成功
- `Err(...)` を返せば失敗

---

### 使用例

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() -> Result<(), String> {
        if add(2, 2) == 4 {
            Ok(())
        } else {
            Err(String::from("addition failed"))
        }
    }

    #[test]
    fn test_with_question_mark() -> Result<(), String> {
        let result = add(2, 2);
        if result != 4 {
            return Err(String::from("addition failed"));
        }
        Ok(())
    }
}
```

---

### ? 演算子を使う

```rust
#[test]
fn test_with_result() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string("test.txt")?;
    assert!(file.contains("hello"));
    Ok(())
}
```

**メリット:** エラー処理が簡潔

---

## Python との比較

### Python (unittest)

```python
import unittest

class TestMath(unittest.TestCase):
    def test_addition(self):
        self.assertEqual(2 + 2, 4)

    def test_greater(self):
        self.assertTrue(5 > 3)

    def test_panic(self):
        with self.assertRaises(ValueError):
            raise ValueError("error")

if __name__ == '__main__':
    unittest.main()
```

---

### Rust

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_addition() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_greater() {
        assert!(5 > 3);
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("error");
    }
}
```

**実行:**
```bash
# Python
python test.py

# Rust
cargo test
```

---

## テストマクロまとめ

### assert!

```rust
assert!(condition);
assert!(condition, "message");
```

**用途:** 条件が true か確認

---

### assert_eq!

```rust
assert_eq!(left, right);
assert_eq!(left, right, "message");
```

**用途:** 2つの値が等しいか確認

---

### assert_ne!

```rust
assert_ne!(left, right);
assert_ne!(left, right, "message");
```

**用途:** 2つの値が異なるか確認

---

## テスト属性まとめ

### #[test]

```rust
#[test]
fn test_name() { ... }
```

**意味:** この関数はテスト関数

---

### #[should_panic]

```rust
#[test]
#[should_panic]
fn test_panic() { ... }
```

**意味:** パニックするべき

---

### #[should_panic(expected = "...")]

```rust
#[test]
#[should_panic(expected = "error message")]
fn test_panic_message() { ... }
```

**意味:** 特定のメッセージでパニックするべき

---

### #[ignore]

```rust
#[test]
#[ignore]
fn expensive_test() { ... }
```

**意味:** デフォルトではスキップ（`cargo test -- --ignored` で実行）

---

## まとめ

### テストの書き方

```
✅ #[test] 属性でテスト関数を定義
✅ assert! で条件を確認
✅ assert_eq! / assert_ne! で値を比較
✅ #[should_panic] でパニックを期待
✅ Result<T, E> でエラー処理を柔軟に
```

---

### 基本パターン

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // 基本的なテスト
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // パニックを期待
    #[test]
    #[should_panic]
    fn it_panics() {
        panic!("Expected panic");
    }

    // Result を使う
    #[test]
    fn with_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("failed"))
        }
    }
}
```
