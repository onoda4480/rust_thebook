# Chapter 7-3: モジュールを別ファイルに分割

## ファイル分割の基本

### なぜファイルを分割するのか？

**大きなモジュールを管理しやすくするため**

```
❌ 1つの巨大ファイル
✅ 複数の小さなファイル
```

---

## ファイル配置のルール

### パターン1: 単一ファイルモジュール

```
src/
├── lib.rs
└── front_of_house.rs
```

```rust
// src/lib.rs
mod front_of_house;  // front_of_house.rs を読み込む

pub use crate::front_of_house::hosting;

// src/front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

**用途:** サブモジュールがない場合

---

### パターン2: サブモジュールあり（古いスタイル）

```
src/
├── lib.rs
└── front_of_house/
    ├── mod.rs           # front_of_house モジュールのルート
    └── hosting.rs       # サブモジュール
```

```rust
// src/lib.rs
mod front_of_house;

// src/front_of_house/mod.rs
pub mod hosting;

// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}
```

**特徴:** `mod.rs` がモジュールのルート

---

### パターン3: サブモジュールあり（新しいスタイル - Rust 2018+）

```
src/
├── lib.rs
├── front_of_house.rs
└── front_of_house/
    └── hosting.rs
```

```rust
// src/lib.rs
mod front_of_house;

// src/front_of_house.rs
pub mod hosting;

// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}
```

**特徴:** `mod.rs` が不要（推奨）

---

## 実践例：レストランプロジェクト

### ディレクトリ構造

```
restaurant/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── main.rs
    └── front_of_house/
        ├── mod.rs
        └── hosting.rs
```

---

### src/lib.rs

```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

**役割:**
- ライブラリクレートのルート
- `front_of_house` モジュールを宣言
- `hosting` を再公開

---

### src/front_of_house/mod.rs

```rust
pub mod hosting;
```

**役割:**
- `front_of_house` モジュールの定義
- `hosting` サブモジュールを宣言

---

### src/front_of_house/hosting.rs

```rust
pub fn add_to_waitlist() {
    println!("Added to waitlist");
}
```

**役割:**
- `hosting` モジュールの実装

---

### src/main.rs

```rust
use restaurant::eat_at_restaurant;

fn main() {
    eat_at_restaurant();
}
```

**役割:**
- バイナリクレートのルート
- ライブラリクレートを使用

---

## モジュールツリーとファイルの対応

### モジュールツリー

```
crate (src/lib.rs)
└── front_of_house (src/front_of_house/mod.rs)
    └── hosting (src/front_of_house/hosting.rs)
        └── add_to_waitlist
```

---

### ファイル対応表

| モジュールパス | ファイルパス |
|---|---|
| `crate` | `src/lib.rs` |
| `crate::front_of_house` | `src/front_of_house/mod.rs` |
| `crate::front_of_house::hosting` | `src/front_of_house/hosting.rs` |

---

## mod 宣言のルール

### ルール1: mod は親ファイルで宣言

```rust
// src/lib.rs
mod front_of_house;  // ← ここで宣言

// src/front_of_house/mod.rs
pub mod hosting;     // ← ここで宣言

// src/front_of_house/hosting.rs
// mod 宣言は不要（ファイル自体がモジュール）
```

---

### ルール2: mod 宣言は1箇所だけ

```rust
// ❌ 間違い：複数箇所で宣言
// src/lib.rs
mod front_of_house;

// src/main.rs
mod front_of_house;  // ❌ エラー！

// ✅ 正しい：use で参照
// src/main.rs
use restaurant::front_of_house;
```

---

## よくあるエラーと解決策

### エラー1: ファイルが見つからない

```
error[E0583]: file not found for module `front_of_house`
```

**原因:** ファイルの配置が間違っている

**解決策:**
```
src/
├── lib.rs
└── front_of_house.rs  または front_of_house/mod.rs
```

---

### エラー2: mod.rs と module.rs が両方ある

```
src/
├── lib.rs
├── front_of_house.rs       # ファイル
└── front_of_house/
    └── mod.rs              # ❌ 衝突！
```

**解決策:** どちらか一方にする

---

### エラー3: lib.rs を mod で読み込もうとする

```rust
// src/main.rs
mod lib;  // ❌ エラー！

// ✅ 正しい
use restaurant::function;  // クレート名を使う
```

---

## 新旧スタイルの比較

### 旧スタイル（Rust 2015）

```
src/
└── module/
    ├── mod.rs           # モジュールのルート
    └── submodule.rs
```

**特徴:**
- `mod.rs` が必須
- Python の `__init__.py` に似ている

---

### 新スタイル（Rust 2018+）

```
src/
├── module.rs            # モジュールのルート
└── module/
    └── submodule.rs
```

**特徴:**
- `mod.rs` 不要
- よりシンプル
- **推奨**

---

## 大規模プロジェクトの構成例

### ディレクトリ構造

```
my_project/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── config.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   └── product.rs
│   ├── services/
│   │   ├── mod.rs
│   │   ├── user_service.rs
│   │   └── product_service.rs
│   └── utils/
│       ├── mod.rs
│       └── helpers.rs
└── tests/
    └── integration_test.rs
```

---

### src/lib.rs

```rust
pub mod config;
pub mod models;
pub mod services;
pub mod utils;

// 再公開
pub use models::{User, Product};
pub use services::{UserService, ProductService};
```

---

### src/models/mod.rs

```rust
pub mod user;
pub mod product;

// 再公開
pub use user::User;
pub use product::Product;
```

---

### src/models/user.rs

```rust
pub struct User {
    pub id: u64,
    pub name: String,
}

impl User {
    pub fn new(id: u64, name: String) -> Self {
        User { id, name }
    }
}
```

---

## テストファイルの配置

### ユニットテスト

```rust
// src/models/user.rs
pub struct User {
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;  // 親モジュールをインポート

    #[test]
    fn test_user() {
        let user = User {
            name: String::from("Alice"),
        };
        assert_eq!(user.name, "Alice");
    }
}
```

**場所:** 実装と同じファイル

---

### 統合テスト

```
tests/
└── integration_test.rs
```

```rust
// tests/integration_test.rs
use my_project::User;

#[test]
fn test_user_creation() {
    let user = User::new(1, String::from("Alice"));
    assert_eq!(user.id, 1);
}
```

**場所:** `tests/` ディレクトリ

---

## まとめ

### ファイル配置のルール

```
単一ファイル:
src/module.rs

サブモジュール（旧）:
src/module/mod.rs + サブモジュール

サブモジュール（新・推奨）:
src/module.rs + src/module/*.rs
```

---

### mod 宣言

```
✅ 親ファイルで1回だけ宣言
✅ mod module_name; でファイルを読み込む
✅ ファイル名 = モジュール名
❌ lib.rs は mod で読み込まない
```

---

### ベストプラクティス

```
✅ Rust 2018+ の新スタイルを使う
✅ モジュールごとにファイルを分ける
✅ pub use で外部APIを整理
✅ テストは実装と同じファイル
```

---

### ファイル対応

```
mod module_name;

↓ 以下のいずれかを探す

1. src/module_name.rs
2. src/module_name/mod.rs
```

---

### Python との対応

| Rust | Python |
|---|---|
| `mod module;` | `import module` |
| `src/module.rs` | `module.py` |
| `src/module/mod.rs` | `module/__init__.py` |
| `pub use` | `__init__.py` で再公開 |

---

## 実行とテスト

```bash
# ビルド
cargo build

# 実行
cargo run

# テスト
cargo test

# 特定のバイナリ実行
cargo run --bin tool_name
```

---

## 次のステップ

第七章で学んだモジュールシステムは、大規模プロジェクトを整理するための基礎です。

**重要なポイント:**
- パッケージとクレートの関係
- モジュール階層とパス
- 公開制御（`pub`）
- ファイル分割

これらの知識を使って、整理された保守しやすいコードを書きましょう！
