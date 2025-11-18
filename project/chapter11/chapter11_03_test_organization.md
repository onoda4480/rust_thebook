# 第11章 まとめ (3/3): テストの構成

## テストの種類

### 2種類のテスト

| テスト | 場所 | 目的 |
|--------|------|------|
| **単体テスト** | src/ 内 | 個々の関数やメソッドをテスト |
| **結合テスト** | tests/ ディレクトリ | ライブラリ全体をテスト |

---

## 単体テスト (Unit Tests)

### 基本構造

```rust
// src/lib.rs
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
}
```

**特徴:**
- テスト対象のコードと同じファイルに書く
- `#[cfg(test)]` で囲む
- プライベート関数もテストできる

---

### #[cfg(test)] の意味

```rust
#[cfg(test)]
mod tests {
    // cargo test の時だけコンパイルされる
}
```

**効果:**
- `cargo test` の時のみコンパイル
- `cargo build` では除外される
- バイナリサイズが小さくなる

---

### プライベート関数のテスト

```rust
// src/lib.rs
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
//  ^^^^^^ プライベート関数
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
        // ✅ プライベート関数もテストできる
    }
}
```

---

## 結合テスト (Integration Tests)

### ディレクトリ構造

```
project/
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/
    ├── integration_test.rs
    └── another_test.rs
```

**ルール:**
- `tests/` ディレクトリに配置
- 各ファイルが独立したクレート
- `#[cfg(test)]` 不要

---

### 基本的な結合テスト

```rust
// src/lib.rs
pub fn add_two(a: i32) -> i32 {
    a + 2
}
```

```rust
// tests/integration_test.rs
use my_project;  // ライブラリをインポート

#[test]
fn it_adds_two() {
    assert_eq!(4, my_project::add_two(2));
}
```

**実行:**
```bash
cargo test
```

**出力:**
```
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running tests/integration_test.rs

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

### 特定の結合テストだけ実行

```bash
# integration_test.rs のテストのみ実行
cargo test --test integration_test
```

---

## 結合テスト用のヘルパーモジュール

### 問題: 共通コードの配置

```
tests/
├── integration_test.rs
└── common.rs  ❌ これだとテストとして実行される
```

**実行:**
```bash
cargo test
```

**出力:**
```
running 0 tests
test result: ok.

     Running tests/common.rs

running 0 tests
test result: ok.

     Running tests/integration_test.rs
```

`common.rs` がテストとして実行されてしまう

---

### 解決: サブディレクトリを使う

```
tests/
├── integration_test.rs
└── common/
    └── mod.rs  ✅ テストとして実行されない
```

```rust
// tests/common/mod.rs
pub fn setup() {
    // ヘルパー関数
}
```

```rust
// tests/integration_test.rs
mod common;

#[test]
fn it_works() {
    common::setup();
    // テスト...
}
```

**ルール:**
- `tests/common/mod.rs` はテストとして実行されない
- ヘルパー関数を配置できる

---

## バイナリクレートと結合テスト

### 問題: バイナリクレートは結合テストできない

```
project/
├── src/
│   └── main.rs  ❌ 結合テストできない
└── tests/
    └── integration_test.rs
```

**理由:** `main.rs` の関数は外部から呼べない

---

### 解決: lib.rs + main.rs パターン

```
project/
├── src/
│   ├── lib.rs   ✅ ロジック（テスト可能）
│   └── main.rs  ✅ lib.rs を呼ぶだけ
└── tests/
    └── integration_test.rs  ✅ lib.rs をテスト
```

---

#### src/lib.rs（ロジック）

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn run() {
    println!("Result: {}", add_two(3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two() {
        assert_eq!(4, add_two(2));
    }
}
```

---

#### src/main.rs（薄いラッパー）

```rust
use my_project;

fn main() {
    my_project::run();
}
```

**ポイント:**
- main.rs は薄い（ロジックなし）
- ロジックは lib.rs にある
- lib.rs は結合テストできる

---

#### tests/integration_test.rs（結合テスト）

```rust
use my_project;

#[test]
fn test_add_two() {
    assert_eq!(4, my_project::add_two(2));
}
```

---

## テスト構成のベストプラクティス

### 単体テスト

```rust
// src/lib.rs
pub fn public_function() { ... }

fn private_function() { ... }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public() {
        assert_eq!(public_function(), ...);
    }

    #[test]
    fn test_private() {
        assert_eq!(private_function(), ...);
    }
}
```

**用途:**
- 個々の関数のロジックをテスト
- プライベート関数もテスト
- 実装の詳細をテスト

---

### 結合テスト

```rust
// tests/integration_test.rs
use my_project;

#[test]
fn test_public_api() {
    // 公開 API のみテスト
    assert_eq!(my_project::public_function(), ...);
}
```

**用途:**
- 公開 API をテスト
- ライブラリ全体の動作をテスト
- 外部から使う時の動作をテスト

---

## ディレクトリ構造例

### 完全な例

```
my_project/
├── Cargo.toml
├── src/
│   ├── lib.rs          # 公開 API + 単体テスト
│   ├── main.rs         # エントリーポイント（薄い）
│   └── utils.rs        # 内部モジュール + 単体テスト
└── tests/
    ├── integration_test.rs    # 結合テスト
    ├── another_test.rs        # 別の結合テスト
    └── common/
        └── mod.rs             # テスト用ヘルパー
```

---

## Python との比較

### Python

```
my_project/
├── my_project/
│   ├── __init__.py
│   └── module.py
└── tests/
    ├── test_unit.py
    └── test_integration.py
```

**実行:**
```bash
python -m pytest tests/
```

---

### Rust

```
my_project/
├── src/
│   └── lib.rs          # 単体テストを含む
└── tests/
    └── integration_test.rs
```

**実行:**
```bash
cargo test
```

---

## まとめ

### テストの種類

| 種類 | 場所 | #[cfg(test)] | 用途 |
|------|------|--------------|------|
| **単体テスト** | src/ 内 | 必要 | 個々の関数、プライベート関数も |
| **結合テスト** | tests/ | 不要 | 公開 API、ライブラリ全体 |

---

### ディレクトリ構造

```
単体テスト:
  src/lib.rs の中に #[cfg(test)] mod tests

結合テスト:
  tests/integration_test.rs（独立したファイル）

ヘルパー:
  tests/common/mod.rs（テストとして実行されない）
```

---

### バイナリクレートのテスト

```
src/lib.rs  ← ロジック（テスト可能）
src/main.rs ← lib.rs を呼ぶだけ（薄い）
tests/      ← lib.rs をテスト
```

---

### ベストプラクティス

```
✅ 単体テスト: 実装の詳細をテスト
✅ 結合テスト: 公開 API をテスト
✅ ヘルパー: tests/common/mod.rs に配置
✅ バイナリ: lib.rs + main.rs パターン
✅ プライベート関数: 単体テストでテスト
```
