# Chapter 7-1: パッケージとクレート

## パッケージとクレートとは？

### パッケージ

**1つ以上のクレートを提供する機能のまとまり**

```
パッケージ
├── Cargo.toml        # パッケージ定義
└── クレート（1つ以上）
```

---

### クレート

**コンパイラが一度に処理するコードの最小単位**

| クレートの種類 | 説明 | ファイル |
|---|---|---|
| **バイナリクレート** | 実行可能なプログラム | `src/main.rs` |
| **ライブラリクレート** | 他のプログラムで使えるコード | `src/lib.rs` |

---

## Cargo の規約

### 基本ルール

```
パッケージディレクトリ/
├── Cargo.toml
└── src/
    ├── main.rs          # バイナリクレートのルート（自動認識）
    ├── lib.rs           # ライブラリクレートのルート（自動認識）
    └── bin/
        ├── binary1.rs   # 追加のバイナリクレート
        └── binary2.rs   # 追加のバイナリクレート
```

**重要なルール:**
- パッケージには**最大1つ**のライブラリクレート
- **複数**のバイナリクレートを持てる
- `src/main.rs` と `src/lib.rs` は Cargo が自動認識

---

## パッケージの種類

### パターン1: バイナリのみ

```
my_project/
├── Cargo.toml
└── src/
    └── main.rs          # バイナリクレート
```

**用途:** 実行可能プログラム

```bash
cargo run
```

---

### パターン2: ライブラリのみ

```
my_library/
├── Cargo.toml
└── src/
    └── lib.rs           # ライブラリクレート
```

**用途:** 他のプロジェクトで使うライブラリ

```bash
cargo build
```

---

### パターン3: ライブラリとバイナリ

```
my_project/
├── Cargo.toml
└── src/
    ├── lib.rs           # ライブラリクレート
    └── main.rs          # バイナリクレート
```

**用途:** ライブラリ + CLI ツール

```bash
cargo run              # main.rs を実行
cargo test             # lib.rs をテスト
```

---

### パターン4: 複数のバイナリ

```
my_project/
├── Cargo.toml
└── src/
    ├── lib.rs           # 共通ライブラリ
    ├── main.rs          # メインバイナリ
    └── bin/
        ├── tool1.rs     # ツール1
        └── tool2.rs     # ツール2
```

**実行方法:**

```bash
cargo run                  # main.rs
cargo run --bin tool1      # tool1.rs
cargo run --bin tool2      # tool2.rs
```

---

## 実践例

### プロジェクトの作成

```bash
# バイナリクレート
cargo new my_project

# ライブラリクレート
cargo new my_library --lib
```

---

### Cargo.toml

```toml
[package]
name = "my_project"        # クレート名
version = "0.1.0"
edition = "2021"

[dependencies]
# 外部クレート
rand = "0.8.5"
```

**ポイント:** `name` フィールドがクレート名になる

---

### ライブラリとバイナリの併用

#### src/lib.rs（ライブラリ）

```rust
/// ライブラリの公開関数
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("Alice"), "Hello, Alice!");
    }
}
```

---

#### src/main.rs（バイナリ）

```rust
// クレート名を使ってライブラリをインポート
use my_project::greet;

fn main() {
    let message = greet("World");
    println!("{}", message);
}
```

**実行:**

```bash
cargo run
# 出力: Hello, World!

cargo test
# テスト実行
```

---

### 複数のバイナリ例

#### src/lib.rs

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
```

---

#### src/main.rs

```rust
use my_project::{add, subtract};

fn main() {
    println!("Add: {}", add(5, 3));
    println!("Subtract: {}", subtract(5, 3));
}
```

---

#### src/bin/calculator.rs

```rust
use my_project::{add, subtract};

fn main() {
    println!("Calculator Tool");
    println!("10 + 5 = {}", add(10, 5));
    println!("10 - 5 = {}", subtract(10, 5));
}
```

---

#### src/bin/converter.rs

```rust
use my_project::add;

fn main() {
    println!("Converter Tool");
    // 独自の処理
}
```

**実行:**

```bash
cargo run                      # main.rs
cargo run --bin calculator     # calculator.rs
cargo run --bin converter      # converter.rs
```

---

## クレートルート

### クレートルートとは？

**コンパイラがクレートのコンパイルを開始するファイル**

| クレート | ルートファイル |
|---|---|
| バイナリクレート | `src/main.rs` |
| ライブラリクレート | `src/lib.rs` |

**ポイント:** ルートファイルからモジュールツリーが始まる

---

### モジュールツリーの例

```
crate (ルート: src/lib.rs または src/main.rs)
├── mod1
│   ├── submod1
│   └── submod2
└── mod2
    └── submod3
```

全てのモジュールは**1つのツリー**を形成します。

---

## 外部クレートの使用

### 依存関係の追加

```toml
# Cargo.toml
[dependencies]
rand = "0.8.5"
serde = { version = "1.0", features = ["derive"] }
```

---

### コードでの使用

```rust
// src/main.rs または src/lib.rs
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(1..=100);
    println!("Random number: {}", n);
}
```

**ポイント:** 外部クレートも `use` でインポート

---

## Python との対応

### プロジェクト構造の比較

#### Python

```
my_project/               # プロジェクト
├── setup.py
├── my_project/           # パッケージ
│   ├── __init__.py
│   └── module.py
└── tests/
```

---

#### Rust

```
my_project/               # パッケージ
├── Cargo.toml
├── src/                  # クレート
│   ├── lib.rs
│   └── main.rs
└── tests/
```

---

### 対応表

| 概念 | Python | Rust |
|---|---|---|
| **プロジェクト全体** | ディレクトリ | パッケージ |
| **コンパイル単位** | モジュール | クレート |
| **実行ファイル** | `__main__.py` | `src/main.rs` |
| **ライブラリ** | `__init__.py` | `src/lib.rs` |
| **依存管理** | `requirements.txt` | `Cargo.toml` |

---

## まとめ

### パッケージとクレート

```
パッケージ = プロジェクト全体
├── Cargo.toml（定義）
└── クレート（1つ以上）
    ├── バイナリクレート（実行可能）
    └── ライブラリクレート（ライブラリ）
```

---

### 重要なルール

```
✅ 1パッケージ = 最大1ライブラリクレート
✅ 1パッケージ = 複数バイナリクレート可
✅ src/main.rs = 自動的にバイナリクレート
✅ src/lib.rs = 自動的にライブラリクレート
✅ src/bin/*.rs = 追加のバイナリクレート
```

---

### 使い分け

```
バイナリのみ          → 実行プログラム
ライブラリのみ        → 再利用可能なコード
ライブラリ + バイナリ → ライブラリ + CLI
複数バイナリ          → ツール群 + 共通コード
```

---

### Cargo の規約

```
src/main.rs    → メインバイナリ（自動認識）
src/lib.rs     → ライブラリ（自動認識）
src/bin/*.rs   → 追加バイナリ（自動認識）
tests/*.rs     → 統合テスト（自動認識）
```

**ポイント:** Cargo.toml に記述不要（規約ベース）
