# 第11章 まとめ (2/3): テストの実行

## 基本的なテスト実行

### すべてのテストを実行

```bash
cargo test
```

**出力例:**
```
running 3 tests
test tests::it_works ... ok
test tests::test_add ... ok
test tests::test_panic ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 並列実行と逐次実行

### デフォルト: 並列実行

```bash
cargo test
```

**特徴:**
- テストは並列（同時）に実行される
- 高速
- テスト間で状態を共有してはいけない

---

### 逐次実行（1つずつ）

```bash
cargo test -- --test-threads=1
```

**用途:**
- テストが状態を共有する場合
- ファイルI/Oを使う場合
- デバッグ時

---

### 例: ファイルを使うテスト

```rust
use std::fs;

#[test]
fn test_write_file() {
    fs::write("test.txt", "hello").unwrap();
    let contents = fs::read_to_string("test.txt").unwrap();
    assert_eq!(contents, "hello");
    fs::remove_file("test.txt").unwrap();
}

#[test]
fn test_read_file() {
    fs::write("test.txt", "world").unwrap();
    let contents = fs::read_to_string("test.txt").unwrap();
    assert_eq!(contents, "world");
    fs::remove_file("test.txt").unwrap();
}
```

**並列実行:**
```bash
cargo test  # ❌ 競合する可能性
```

**逐次実行:**
```bash
cargo test -- --test-threads=1  # ✅ OK
```

---

## 標準出力の表示

### デフォルト: 成功したテストの出力は非表示

```rust
#[test]
fn test_with_println() {
    println!("This will not be shown if test passes");
    assert_eq!(2 + 2, 4);
}
```

**実行:**
```bash
cargo test
```

**結果:** `println!` の出力は表示されない

---

### すべての出力を表示

```bash
cargo test -- --show-output
```

**結果:** 成功したテストの `println!` も表示される

---

### 使用例

```rust
fn prints_and_returns_10(a: i32) -> i32 {
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
```

**通常の実行:**
```bash
cargo test
```

**出力:**
```
test tests::this_test_will_pass ... ok
test tests::this_test_will_fail ... FAILED

failures:
---- tests::this_test_will_fail stdout ----
I got the value 8
```

失敗したテストの出力のみ表示される

---

**すべて表示:**
```bash
cargo test -- --show-output
```

**出力:**
```
test tests::this_test_will_pass ... ok
---- tests::this_test_will_pass stdout ----
I got the value 4

test tests::this_test_will_fail ... FAILED
---- tests::this_test_will_fail stdout ----
I got the value 8
```

成功したテストの出力も表示される

---

## 特定のテストを実行

### 名前でフィルタリング

```bash
cargo test test_name
```

**例:**

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, 2 + 2);
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, 3 + 2);
    }

    #[test]
    fn one_hundred() {
        assert_eq!(100, 100);
    }
}
```

**1つのテストだけ実行:**
```bash
cargo test one_hundred
```

**出力:**
```
running 1 test
test tests::one_hundred ... ok
```

---

### 複数のテストを実行（前方一致）

```bash
cargo test add
```

**出力:**
```
running 2 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok
```

**意味:** 名前に "add" を含むテストを実行

---

### モジュール名でフィルタリング

```rust
#[cfg(test)]
mod math_tests {
    #[test]
    fn test_add() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(5 - 3, 2);
    }
}

#[cfg(test)]
mod string_tests {
    #[test]
    fn test_concat() {
        assert_eq!("hello".to_string() + "world", "helloworld");
    }
}
```

**math_tests モジュールのみ実行:**
```bash
cargo test math_tests
```

---

## テストを無視する

### #[ignore] 属性

```rust
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // 時間のかかるテスト
    // 通常はスキップしたい
}
```

**通常の実行:**
```bash
cargo test
```

**出力:**
```
running 2 tests
test expensive_test ... ignored
test it_works ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
```

---

### 無視されたテストだけを実行

```bash
cargo test -- --ignored
```

**出力:**
```
running 1 test
test expensive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out
```

---

### すべてのテストを実行（無視されたものも含む）

```bash
cargo test -- --include-ignored
```

---

## テスト実行オプションまとめ

### 基本コマンド

```bash
# すべてのテストを実行
cargo test

# ヘルプを表示
cargo test --help
cargo test -- --help
```

---

### 実行制御

```bash
# 並列実行のスレッド数を指定
cargo test -- --test-threads=1
cargo test -- --test-threads=4

# すべての出力を表示
cargo test -- --show-output
```

---

### フィルタリング

```bash
# 特定の名前のテストのみ
cargo test test_name

# 名前に含まれるテスト（前方一致）
cargo test prefix

# 無視されたテストのみ
cargo test -- --ignored

# すべて（無視されたテストも含む）
cargo test -- --include-ignored
```

---

### 組み合わせ

```bash
# 特定のテストを逐次実行で、出力を表示
cargo test test_name -- --test-threads=1 --show-output

# 無視されたテストを逐次実行
cargo test -- --ignored --test-threads=1
```

---

## テスト実行の流れ

### 1. コンパイル

```bash
cargo test
```

**内部:**
1. `#[cfg(test)]` のコードをコンパイル
2. テストバイナリを生成

---

### 2. テスト実行

**デフォルト:**
- すべてのテストを並列実行
- 成功したテストの出力は非表示

---

### 3. 結果表示

```
running 3 tests
test tests::test1 ... ok
test tests::test2 ... ok
test tests::test3 ... FAILED

failures:

---- tests::test3 stdout ----
thread 'tests::test3' panicked at 'assertion failed: ...'

failures:
    tests::test3

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## Python との比較

### Python (unittest)

```bash
# すべてのテストを実行
python -m unittest

# 特定のテストを実行
python -m unittest test_module.TestClass.test_method

# 詳細出力
python -m unittest -v
```

---

### Rust

```bash
# すべてのテストを実行
cargo test

# 特定のテストを実行
cargo test test_name

# 詳細出力
cargo test -- --show-output
```

---

## まとめ

### 基本コマンド

```bash
# すべて実行
cargo test

# 特定のテスト
cargo test test_name

# 逐次実行
cargo test -- --test-threads=1

# 出力表示
cargo test -- --show-output

# 無視されたテストのみ
cargo test -- --ignored
```

---

### よく使うパターン

```bash
# デバッグ時（逐次、出力表示）
cargo test -- --test-threads=1 --show-output

# 特定のモジュールをデバッグ
cargo test module_name -- --test-threads=1 --show-output

# CI/CD で全テスト実行（無視されたものも含む）
cargo test -- --include-ignored
```

---

### テスト実行のベストプラクティス

```
✅ 通常は並列実行（デフォルト）
✅ デバッグ時は逐次実行 + 出力表示
✅ 時間のかかるテストは #[ignore]
✅ CI/CD では --include-ignored
✅ 特定の機能のテストは名前でフィルタリング
```
