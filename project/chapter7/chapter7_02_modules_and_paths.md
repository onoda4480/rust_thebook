# Chapter 7-2: モジュールとパス

## モジュールシステム

### モジュールとは？

**コードを整理し、名前空間を提供する仕組み**

```rust
mod module_name {
    // モジュールの中身
}
```

**役割:**
- コードの整理
- 名前空間の提供
- カプセル化（公開/非公開制御）

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

## パスとスコープ

### パスの種類

| パスの種類 | 構文 | 説明 | 例 |
|---|---|---|---|
| **絶対パス** | `crate::` | クレートルートから | `crate::front_of_house::hosting` |
| **相対パス** | `module` | 現在位置から | `front_of_house::hosting` |
| **親モジュール** | `super::` | 親モジュールから | `super::serve_order` |
| **現在モジュール** | `self::` | 現在のモジュールから | `self::hosting` |

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

**利点:**
- コードを移動しても動く
- 依存関係が明確

---

### 相対パスの例

```rust
pub fn eat_at_restaurant() {
    // 相対パス（front_of_house は兄弟モジュール）
    front_of_house::hosting::add_to_waitlist();
}
```

**利点:**
- 短く書ける
- 同じモジュール内で完結

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

### 実践例：レストラン

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}
        fn serve_order() {}

        fn take_payment() {
            // 同じ親の下のモジュール
            super::hosting::seat_at_table();
        }
    }
}

pub fn eat_at_restaurant() {
    // 絶対パス
    crate::front_of_house::hosting::add_to_waitlist();

    // 相対パス
    front_of_house::hosting::seat_at_table();
}
```

---

## pub による公開制御

### デフォルトは非公開

```rust
mod front_of_house {
    mod hosting {                    // 非公開モジュール
        fn add_to_waitlist() {}      // 非公開関数
    }
}

pub fn eat_at_restaurant() {
    // ❌ エラー！モジュールも関数も非公開
    front_of_house::hosting::add_to_waitlist();
}
```

---

### pub で公開

```rust
mod front_of_house {
    pub mod hosting {                // ✅ 公開モジュール
        pub fn add_to_waitlist() {}  // ✅ 公開関数
    }
}

pub fn eat_at_restaurant() {
    // ✅ OK！両方公開されている
    front_of_house::hosting::add_to_waitlist();
}
```

**重要:** モジュールを公開しても、中身は自動的に公開されない

---

### プライバシールール

#### ルール1: 親 → 子

```rust
mod parent {
    fn parent_fn() {
        // ❌ 子モジュールの非公開要素にアクセスできない
        child::private_fn();
    }

    mod child {
        fn private_fn() {}
        pub fn public_fn() {}  // これならOK
    }
}
```

---

#### ルール2: 子 → 親

```rust
mod parent {
    fn parent_fn() {}  // 非公開でも子からアクセス可

    mod child {
        fn child_fn() {
            super::parent_fn();  // ✅ OK！親は常に見える
        }
    }
}
```

---

#### ルール3: 兄弟同士

```rust
mod sibling1 {
    pub fn public_fn() {}
    fn private_fn() {}
}

mod sibling2 {
    fn use_sibling() {
        // ✅ OK（公開されている）
        super::sibling1::public_fn();

        // ❌ エラー（非公開）
        // super::sibling1::private_fn();
    }
}
```

---

### まとめ：プライバシールール

| 関係 | pub 必要？ | 理由 |
|---|---|---|
| **子 → 親** | ❌ 不要 | 親は常に見える |
| **親 → 子** | ✅ 必要 | カプセル化 |
| **兄弟同士** | ✅ 必要 | 明示的な公開 |

**ただし:** 同じモジュール内の兄弟は `pub` なしでアクセス可能

---

## Struct と Enum の公開

### Struct の公開

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,        // ✅ 公開フィールド
        seasonal_fruit: String,   // ❌ 非公開フィールド
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");  // ✅ OK（公開フィールド）
    // meal.seasonal_fruit = String::from("blueberries");  // ❌ エラー（非公開）
}
```

**ポイント:** フィールドごとに公開/非公開を指定

---

### Enum の公開

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,   // 自動的に公開
        Salad,  // 自動的に公開
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;   // ✅ OK
    let order2 = back_of_house::Appetizer::Salad;  // ✅ OK
}
```

**ポイント:** `pub enum` にすると**全バリアントが自動的に公開**

---

### Struct vs Enum の違い

```rust
// Struct：フィールドは個別に pub が必要
pub struct Point {
    pub x: i32,      // 個別に pub
    pub y: i32,      // 個別に pub
}

// Enum：pub enum で全バリアントが公開
pub enum Direction {
    North,   // 自動的に pub
    South,   // 自動的に pub
    East,    // 自動的に pub
    West,    // 自動的に pub
}
```

**理由:** Enum はバリアントが使えないと意味がないため

---

## use によるスコープへの持ち込み

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
hosting::add_to_waitlist();  // どこから来たか分かりやすい

// ❌ 非推奨：関数を直接インポート
use crate::front_of_house::hosting::add_to_waitlist;
add_to_waitlist();  // どこから来たか分かりにくい
```

---

#### 構造体・Enum の場合

```rust
// ✅ 推奨：型を直接インポート
use std::collections::HashMap;
let map = HashMap::new();

// ❌ 冗長
use std::collections;
let map = collections::HashMap::new();
```

---

### as によるエイリアス

```rust
use std::fmt::Result;
use std::io::Result as IoResult;  // エイリアス

fn function1() -> Result {        // fmt::Result
    // ...
    Ok(())
}

fn function2() -> IoResult<()> {  // io::Result
    // ...
    Ok(())
}
```

**使い所:** 名前が衝突する時

---

### 複数のインポート

```rust
// 個別にインポート
use std::io;
use std::cmp::Ordering;

// または一緒に
use std::{io, cmp::Ordering};

// ネストしたパス
use std::io::{self, Write};
// これは以下と同じ:
// use std::io;
// use std::io::Write;
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

**使ってもいい場面:**
- テストコード（`use super::*;`）
- Prelude パターン

---

## pub use による再公開

### 再公開とは？

**内部モジュール構造を隠して、外部APIをシンプルにする**

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

---

### 実践例

#### Before: 再公開なし

```rust
// 外部コード
use restaurant::front_of_house::hosting;
hosting::add_to_waitlist();
```

**問題:** 内部構造が露出している

---

#### After: 再公開あり

```rust
// src/lib.rs
pub use crate::front_of_house::hosting;

// 外部コード
use restaurant::hosting;
hosting::add_to_waitlist();
```

**利点:** 内部構造を隠せる

---

### Python との対応

#### Rust の pub use

```rust
// src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add() {}
    }
}

pub use crate::front_of_house::hosting;
```

---

#### Python の __init__.py

```python
# restaurant/__init__.py
from .front_of_house import hosting  # 再公開

# 外部コード
from restaurant import hosting
hosting.add()
```

---

## 外部クレートの使用

### 依存関係の追加

```toml
# Cargo.toml
[dependencies]
rand = "0.8.5"
```

---

### コードでの使用

```rust
use rand::Rng;  // 外部クレート

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret: {}", secret_number);
}
```

**ポイント:** 外部クレートも `use` でインポート

---

### 標準ライブラリ

```rust
use std::collections::HashMap;
use std::io;
use std::fs::File;
```

**ポイント:** 標準ライブラリも外部クレートと同じ扱い（ただし Cargo.toml に記述不要）

---

## まとめ

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
crate::   = 絶対パス（クレートルートから）
module::  = 相対パス（現在位置から）
super::   = 親モジュール
self::    = 現在のモジュール
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

### プライバシールール

```
親 → 子: pub が必要（カプセル化）
子 → 親: 常にアクセス可能
兄弟同士: pub が必要
```

---

### use

```
✅ パスをスコープに持ち込む
✅ as でエイリアス
✅ pub use で再公開
✅ 関数は親モジュールまで、型は直接
```

---

### ベストプラクティス

```
✅ 絶対パス（crate::）を推奨
✅ 関数は親モジュールまで use
✅ 型は直接 use
✅ 名前衝突は as で解決
✅ pub use で外部APIを整理
```
