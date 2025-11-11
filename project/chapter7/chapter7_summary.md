# Chapter 7: パッケージ、クレート、モジュール - まとめ

## この章で学んだこと

1. **パッケージとクレート**
2. **モジュールシステム**
3. **パスとスコープ**
4. **pub による公開制御**
5. **use によるスコープへの持ち込み**
6. **モジュールを別ファイルに分割**

---

## 1. パッケージとクレート

### パッケージとは？

**1つ以上のクレートを提供する機能のまとまり**

```
パッケージ
├── Cargo.toml        # パッケージ定義
└── クレート（1つ以上）
```

---

### クレートとは？

**コンパイラが一度に処理するコードの最小単位**

| クレートの種類 | 説明 | ファイル |
|---|---|---|
| **バイナリクレート** | 実行可能なプログラム | `src/main.rs` |
| **ライブラリクレート** | 他のプログラムで使えるコード | `src/lib.rs` |

---

### Cargo の規約

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

**ルール:**
- パッケージには**最大1つ**のライブラリクレート
- **複数**のバイナリクレートを持てる

---

### パッケージの例

#### バイナリのみ

```
my_project/
├── Cargo.toml
└── src/
    └── main.rs          # バイナリクレート
```

#### ライブラリとバイナリ

```
my_project/
├── Cargo.toml
└── src/
    ├── lib.rs           # ライブラリクレート
    └── main.rs          # バイナリクレート
```

#### 複数のバイナリ

```
my_project/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── main.rs
    └── bin/
        ├── tool1.rs     # cargo run --bin tool1
        └── tool2.rs     # cargo run --bin tool2
```

---

## 2. モジュールシステム

### モジュールとは？

**コードを整理し、名前空間を提供する仕組み**

```rust
mod module_name {
    // モジュールの中身
}
```

---

### モジュール階層

```rust
// src/lib.rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
    }
}
```

**階層構造:**
```
crate (ルート)
└── front_of_house
    ├── hosting
    │   ├── add_to_waitlist
    │   └── seat_at_table
    └── serving
        ├── take_order
        └── serve_order
```

---

## 3. パスとスコープ

### パスの種類

| パスの種類 | 構文 | 説明 |
|---|---|---|
| **絶対パス** | `crate::module::item` | クレートルートから |
| **相対パス** | `module::item` | 現在位置から |
| **親モジュール** | `super::item` | 親モジュールから |
| **現在モジュール** | `self::item` | 現在のモジュールから |

---

### 絶対パスの例

```rust
// src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 絶対パス
    crate::front_of_house::hosting::add_to_waitlist();
}
```

---

### 相対パスの例

```rust
pub fn eat_at_restaurant() {
    // 相対パス（front_of_house は兄弟モジュール）
    front_of_house::hosting::add_to_waitlist();
}
```

---

### super:: の使用例

```rust
fn serve_order() {}

mod back_of_house {
    fn cook_order() {}

    fn fix_incorrect_order() {
        cook_order();           // 同じモジュール内
        super::serve_order();   // 親モジュールの関数
    }
}
```

**Python との対応:**
```
super::  = Python の ..（親ディレクトリ）
self::   = Python の .（現在のディレクトリ）
crate::  = プロジェクトルート
```

---

## 4. pub による公開制御

### デフォルトは非公開

```rust
mod front_of_house {
    mod hosting {                    // 非公開モジュール
        fn add_to_waitlist() {}      // 非公開関数
    }
}

// ❌ エラー！モジュールも関数も非公開
```

---

### pub で公開

```rust
mod front_of_house {
    pub mod hosting {                // 公開モジュール
        pub fn add_to_waitlist() {}  // 公開関数
    }
}

// ✅ OK！両方公開されている
```

**重要:** モジュールを公開しても、中身は自動的に公開されない

---

### プライバシールール

#### 親 → 子

```rust
mod parent {
    fn parent_fn() {
        // ❌ 子モジュールの非公開要素にアクセスできない
    }

    mod child {
        fn private_fn() {}
    }
}
```

#### 子 → 親

```rust
mod parent {
    fn parent_fn() {}  // 非公開でも子からアクセス可

    mod child {
        fn child_fn() {
            super::parent_fn();  // ✅ OK
        }
    }
}
```

#### 兄弟同士

```rust
mod sibling1 {
    pub fn public_fn() {}
    fn private_fn() {}
}

mod sibling2 {
    fn use_sibling() {
        super::sibling1::public_fn();   // ✅ OK（公開）
        // super::sibling1::private_fn(); // ❌ エラー（非公開）
    }
}
```

**まとめ:**
- 親 → 子: `pub` が必要
- 子 → 親: 常にアクセス可能
- 兄弟同士: `pub` が必要

---

### Struct の公開

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,        // 公開フィールド
        seasonal_fruit: String,   // 非公開フィールド
    }
}
```

**ポイント:** フィールドごとに公開/非公開を指定

---

### Enum の公開

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,     // 自動的に公開
        Salad,    // 自動的に公開
    }
}
```

**ポイント:** `pub enum` にすると**全バリアントが自動的に公開**

---

## 5. use によるスコープへの持ち込み

### use の基本

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// パスを use でスコープに持ち込む
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();  // 短く書ける！
}
```

---

### use のベストプラクティス

#### 関数の場合

```rust
// ✅ 推奨：親モジュールまで
use crate::front_of_house::hosting;
hosting::add_to_waitlist();

// ❌ 非推奨：関数を直接インポート
use crate::front_of_house::hosting::add_to_waitlist;
add_to_waitlist();  // どこから来たか分かりにくい
```

#### 構造体・Enum の場合

```rust
// ✅ 推奨：型を直接インポート
use std::collections::HashMap;
let map = HashMap::new();
```

---

### as によるエイリアス

```rust
use std::fmt::Result;
use std::io::Result as IoResult;  // エイリアス

fn function1() -> Result {}        // fmt::Result
fn function2() -> IoResult<()> {}  // io::Result
```

---

### pub use による再公開

```rust
// src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 再公開：外部のコードが短いパスで使える
pub use crate::front_of_house::hosting;

// 外部コード
// restaurant::hosting::add_to_waitlist();
// (front_of_house を経由しなくてOK)
```

**Python との対応:**
```python
# restaurant/__init__.py
from .front_of_house import hosting  # 再公開
```

---

### 外部クレートの使用

```rust
// Cargo.toml
[dependencies]
rand = "0.8.5"

// src/main.rs
use rand::Rng;  // 外部クレート

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

---

### Glob 演算子

```rust
// すべての公開要素をインポート
use std::collections::*;

let map = HashMap::new();
let set = HashSet::new();
```

**注意:** どこから来たか分かりにくくなるため、使用は慎重に

---

## 6. モジュールを別ファイルに分割

### ファイル配置のルール

#### パターン1: 単一ファイル

```
src/
├── lib.rs
└── front_of_house.rs
```

```rust
// src/lib.rs
mod front_of_house;  // front_of_house.rs を読み込む

// src/front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

---

#### パターン2: サブモジュールあり

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

---

### 新しいスタイル（Rust 2018+）

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

**ポイント:** `mod.rs` が不要になった（よりシンプル）

---

### 完全な例

#### ディレクトリ構造

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

#### src/lib.rs

```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

#### src/front_of_house/mod.rs

```rust
pub mod hosting;
```

#### src/front_of_house/hosting.rs

```rust
pub fn add_to_waitlist() {
    println!("Added to waitlist");
}
```

#### src/main.rs

```rust
use restaurant::eat_at_restaurant;

fn main() {
    eat_at_restaurant();
}
```

**実行:**
```bash
cargo run
# 出力: Added to waitlist
```

---

## Python との対応表

| Rust | Python | 説明 |
|------|--------|------|
| **パッケージ** | プロジェクトディレクトリ | 複数のクレート/モジュール |
| **クレート** | パッケージ | 独立したコンパイル単位 |
| **`mod module`** | ディレクトリ/`.py` ファイル | モジュール定義 |
| **`pub`** | 公開（デフォルト） | 明示的に公開 |
| **`use`** | `import` / `from ... import` | スコープに持ち込む |
| **`pub use`** | `__init__.py` で再公開 | 外部APIを整理 |
| **`crate::`** | プロジェクトルート | 絶対パス |
| **`super::`** | `..` | 親モジュール |
| **`self::`** | `.` | 現在のモジュール |

---

## ライブラリとバイナリの使い分け

### main.rs で lib.rs を使う

```rust
// src/lib.rs
pub fn library_function() {
    println!("ライブラリの関数");
}

// src/main.rs
use my_crate::library_function;  // クレート名を使う

fn main() {
    library_function();
}
```

**重要:** `lib.rs` は `mod lib;` ではなく**クレート名**でインポート

---

## 重要な概念

### 1. クレートルートとモジュールツリー

```
crate (ルート)
├── mod1
│   ├── submod1
│   └── submod2
└── mod2
    └── submod3
```

**ポイント:** 全てのモジュールは1つのツリーを形成

---

### 2. プライバシー境界

```
公開ルール:
- デフォルトは非公開
- pub で明示的に公開
- 親は子の非公開要素にアクセスできない
- 子は親の要素に常にアクセス可能
```

---

### 3. パスの解決

```rust
// 絶対パス（推奨：移動に強い）
crate::module::item

// 相対パス（現在位置に依存）
module::item

// 親モジュール
super::item
```

---

### 4. use のイディオム

```rust
// 関数：親モジュールまで
use std::collections::HashMap;

// 型：型自体を
use std::fmt::Result;

// 名前衝突：エイリアス
use std::io::Result as IoResult;
```

---

## よくあるエラーと解決策

### エラー1: モジュールが見つからない

```
error[E0583]: file not found for module `module_name`
```

**原因:**
- ファイルが正しい場所にない
- `mod` 宣言がない

**解決策:**
```
src/
├── lib.rs
└── module_name.rs  または module_name/mod.rs
```

---

### エラー2: 非公開要素へのアクセス

```
error[E0603]: module `hosting` is private
```

**原因:** `pub` が不足している

**解決策:**
```rust
pub mod hosting {        // モジュールを公開
    pub fn add() {}      // 関数も公開
}
```

---

### エラー3: lib.rs を mod で読み込もうとする

```
warning: found module declaration for lib.rs
```

**原因:** `mod lib;` を使っている

**解決策:**
```rust
// ❌ 間違い
mod lib;

// ✅ 正しい
use my_crate::function;  // クレート名を使う
```

---

## まとめ

### パッケージとクレート

```
✅ パッケージ = 1つ以上のクレート
✅ クレート = バイナリ or ライブラリ
✅ Cargo.toml でパッケージを定義
✅ src/main.rs = バイナリクレート
✅ src/lib.rs = ライブラリクレート
```

---

### モジュール

```
✅ mod でモジュールを定義
✅ 階層構造で整理
✅ 名前空間を提供
✅ デフォルトは非公開
```

---

### パス

```
✅ crate:: = 絶対パス
✅ module:: = 相対パス
✅ super:: = 親モジュール
✅ self:: = 現在のモジュール
```

---

### 公開制御

```
✅ pub で明示的に公開
✅ モジュールと中身は別々に公開
✅ struct のフィールドは個別に公開
✅ enum のバリアントは自動的に公開
```

---

### use

```
✅ パスをスコープに持ち込む
✅ as でエイリアス
✅ pub use で再公開
✅ 外部クレートも use でインポート
```

---

### ファイル分割

```
✅ mod 宣言でファイルを読み込む
✅ module.rs または module/mod.rs
✅ サブモジュールは module/ 配下
```

---

## 次のステップ

第七章で学んだモジュールシステムは、Rust プロジェクトを整理するための基礎です。

次の章では、コレクション（ベクタ、文字列、ハッシュマップ）など、さらに実践的なトピックに進みます！
