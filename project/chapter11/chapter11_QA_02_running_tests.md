# 第11章 Q&A (2/3): テストの実行

## Q1: テストはなぜデフォルトで並列実行される？逐次実行すべき場合は？

### A: 高速化のため。状態を共有するテストは逐次実行すべき。

---

### デフォルト: 並列実行

```bash
cargo test
```

**メリット:**
- 高速
- 複数のCPUコアを活用

**前提:**
- テスト間で状態を共有しない
- 各テストが独立している

---

### 逐次実行が必要な場合

```bash
cargo test -- --test-threads=1
```

**使うべき場合:**

---

#### 1. ファイルI/O

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

**問題:** 並列実行すると同じファイルを同時に操作して競合

**解決:**
```bash
cargo test -- --test-threads=1
```

---

#### 2. グローバル状態の変更

```rust
static mut COUNTER: i32 = 0;

#[test]
fn test_increment() {
    unsafe {
        COUNTER = 0;
        COUNTER += 1;
        assert_eq!(COUNTER, 1);
    }
}

#[test]
fn test_decrement() {
    unsafe {
        COUNTER = 0;
        COUNTER -= 1;
        assert_eq!(COUNTER, -1);
    }
}
```

**問題:** 並列実行すると COUNTER が競合

**解決:**
```bash
cargo test -- --test-threads=1
```

---

#### 3. データベース接続

```rust
#[test]
fn test_insert() {
    let conn = setup_database();
    conn.execute("INSERT INTO users VALUES (1, 'Alice')");
    assert_eq!(conn.count_users(), 1);
    cleanup_database();
}

#[test]
fn test_delete() {
    let conn = setup_database();
    conn.execute("DELETE FROM users WHERE id = 1");
    assert_eq!(conn.count_users(), 0);
    cleanup_database();
}
```

**問題:** 並列実行すると同じDBを同時に操作

**解決:**
```bash
cargo test -- --test-threads=1
```

---

### まとめ

```
並列実行（デフォルト）:
✅ テストが独立している
✅ 高速

逐次実行:
✅ ファイルI/O
✅ グローバル状態の変更
✅ データベース操作
✅ デバッグ時
```

---

## Q2: --show-output オプションはいつ使う？

### A: テストの動作を詳しく見たい時、特にデバッグ時

---

### デフォルトの動作

```rust
#[test]
fn test_with_output() {
    println!("Starting test");
    let x = 2 + 2;
    println!("Calculated: {}", x);
    assert_eq!(x, 4);
    println!("Test passed");
}
```

**実行:**
```bash
cargo test
```

**出力:**
```
test test_with_output ... ok
```

**問題:** println! の出力が見えない

---

### --show-output を使う

```bash
cargo test -- --show-output
```

**出力:**
```
test test_with_output ... ok
---- test_with_output stdout ----
Starting test
Calculated: 4
Test passed
```

**メリット:** すべての出力が見える

---

### 使うべき場合

#### 1. デバッグ時

```rust
#[test]
fn test_complex_logic() {
    let data = load_data();
    println!("Loaded {} items", data.len());

    let processed = process(data);
    println!("Processed: {:?}", processed);

    assert_eq!(processed.len(), 10);
}
```

**実行:**
```bash
cargo test test_complex_logic -- --show-output
```

---

#### 2. 中間値の確認

```rust
#[test]
fn test_calculation() {
    let a = 10;
    let b = 20;
    println!("a = {}, b = {}", a, b);

    let sum = a + b;
    println!("sum = {}", sum);

    let result = sum * 2;
    println!("result = {}", result);

    assert_eq!(result, 60);
}
```

---

#### 3. テストの進行状況

```rust
#[test]
fn test_steps() {
    println!("Step 1: Initialize");
    let mut data = vec![];

    println!("Step 2: Add items");
    data.push(1);
    data.push(2);

    println!("Step 3: Verify");
    assert_eq!(data.len(), 2);
}
```

---

### まとめ

```
--show-output を使うべき場合:
✅ デバッグ時
✅ 中間値を確認したい
✅ テストの進行状況を見たい
✅ なぜ失敗したか詳しく知りたい

使い方:
cargo test -- --show-output
cargo test test_name -- --show-output
```

---

## Q3: 特定のテストだけ実行するには？

### A: テスト名でフィルタリング

---

### 1つのテストのみ実行

```rust
#[test]
fn test_add() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_subtract() {
    assert_eq!(5 - 3, 2);
}

#[test]
fn test_multiply() {
    assert_eq!(3 * 4, 12);
}
```

**特定のテストのみ:**
```bash
cargo test test_add
```

**出力:**
```
running 1 test
test test_add ... ok
```

---

### 複数のテストを実行（前方一致）

```bash
cargo test test
```

**出力:**
```
running 3 tests
test test_add ... ok
test test_multiply ... ok
test test_subtract ... ok
```

**意味:** 名前に "test" を含むすべてのテスト

---

### モジュール名でフィルタリング

```rust
#[cfg(test)]
mod math_tests {
    #[test]
    fn test_add() { ... }

    #[test]
    fn test_subtract() { ... }
}

#[cfg(test)]
mod string_tests {
    #[test]
    fn test_concat() { ... }

    #[test]
    fn test_split() { ... }
}
```

**math_tests のみ:**
```bash
cargo test math_tests
```

**出力:**
```
running 2 tests
test math_tests::test_add ... ok
test math_tests::test_subtract ... ok
```

---

### 結合テストをフィルタリング

```bash
# 特定の結合テストファイルのみ
cargo test --test integration_test

# 結合テスト内の特定のテスト
cargo test --test integration_test test_name
```

---

### まとめ

```
特定のテストを実行:
cargo test test_name        # 1つのテスト
cargo test prefix           # 前方一致
cargo test module_name      # モジュール単位
cargo test --test file_name # 結合テストファイル単位
```

---

## Q4: テストのコンパイル時間を短縮するには？

### A: 複数の方法がある

---

### 1. 特定のテストのみ実行

```bash
# 全テストをコンパイル
cargo test

# 特定のテストのみ（コンパイルは全体）
cargo test test_name
```

**注意:** フィルタリングしてもコンパイルは全体される

---

### 2. #[ignore] を活用

```rust
#[test]
fn quick_test() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn slow_test() {
    // 時間のかかるテスト
}
```

**開発中:**
```bash
cargo test  # quick_test のみ実行
```

**CI/CD:**
```bash
cargo test -- --include-ignored  # 全テスト実行
```

---

### 3. モジュールを分割

```rust
// src/lib.rs
pub mod math;
pub mod string;

#[cfg(test)]
mod tests {
    // 基本的なテストのみ
}
```

```rust
// src/math.rs
#[cfg(test)]
mod tests {
    // math モジュールのテスト
}
```

**math のみテスト:**
```bash
cargo test math
```

---

### 4. 結合テストを分割

```
tests/
├── math_test.rs      # 数学関数のテスト
├── string_test.rs    # 文字列関数のテスト
└── io_test.rs        # I/O のテスト
```

**math のみ:**
```bash
cargo test --test math_test
```

---

### まとめ

```
コンパイル時間短縮:
✅ 特定のテストのみ実行（フィルタリング）
✅ #[ignore] で重いテストをスキップ
✅ モジュールを分割
✅ 結合テストを分割
✅ 開発中は軽いテストのみ実行
```

---

## Q5: cargo test と cargo test -- の違いは？

### A: -- の後ろはテストバイナリへのオプション

---

### cargo test へのオプション

```bash
cargo test [CARGO OPTIONS]
```

**例:**
```bash
# ヘルプ
cargo test --help

# 結合テストを指定
cargo test --test integration_test

# ベンチマークモード
cargo test --bench

# リリースモード
cargo test --release
```

---

### テストバイナリへのオプション

```bash
cargo test -- [TEST BINARY OPTIONS]
```

**例:**
```bash
# スレッド数を指定
cargo test -- --test-threads=1

# 出力を表示
cargo test -- --show-output

# 無視されたテストを実行
cargo test -- --ignored

# ヘルプ
cargo test -- --help
```

---

### 両方を組み合わせ

```bash
cargo test [CARGO OPTIONS] -- [TEST BINARY OPTIONS]
```

**例:**
```bash
# 結合テストを逐次実行で出力表示
cargo test --test integration_test -- --test-threads=1 --show-output

# リリースモードで無視されたテストを実行
cargo test --release -- --ignored
```

---

### 区別の仕方

```
cargo test
  ↑ Cargo へのコマンド

cargo test -- --show-output
           ↑↑ 区切り
              ↑ テストバイナリへのオプション
```

---

### よく使うパターン

```bash
# デバッグ用（逐次、出力表示）
cargo test -- --test-threads=1 --show-output

# 特定のテストをデバッグ
cargo test test_name -- --show-output

# CI/CD（全テスト、無視されたものも含む）
cargo test -- --include-ignored

# 結合テストを詳細表示
cargo test --test integration_test -- --show-output
```

---

### まとめ

```
cargo test [NAME]
  ↑ テスト名でフィルタリング（Cargo が処理）

cargo test -- [OPTIONS]
              ↑ テストバイナリへのオプション

組み合わせ:
cargo test [NAME] -- [OPTIONS]
```
