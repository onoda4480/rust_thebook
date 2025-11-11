# Chapter 7: パッケージ、クレート、モジュール - Q&A

## Q1: パッケージとクレートの違い

**質問:** パッケージとクレートって何が違うの？

**回答:** **パッケージはクレートを含む箱**です

### パッケージ

```
パッケージ = プロジェクト全体
- Cargo.toml で定義
- 1つ以上のクレートを含む
```

### クレート

```
クレート = コンパイル単位
- バイナリクレート（実行可能）
- ライブラリクレート（ライブラリ）
```

---

### 実例

```
restaurant/                    ← パッケージ
├── Cargo.toml
└── src/
    ├── lib.rs                ← ライブラリクレート
    └── main.rs               ← バイナリクレート
```

**まとめ:**
```
パッケージ > クレート

1パッケージ = 最大1ライブラリクレート + 複数バイナリクレート
```

---

## Q2: Cargo の自動認識の仕組み

**質問:** `src/main.rs` と `src/lib.rs` が自動で認識されるのはなぜ？

**回答:** **Cargo の規約**です

### 規約による自動認識

| ファイルパス | 自動認識される内容 |
|---|---|
| `src/main.rs` | バイナリクレートのルート |
| `src/lib.rs` | ライブラリクレートのルート |
| `src/bin/*.rs` | 追加のバイナリクレート |

---

### 例：複数のバイナリ

```
my_project/
├── Cargo.toml
└── src/
    ├── lib.rs               # ライブラリ
    ├── main.rs              # メインバイナリ (cargo run)
    └── bin/
        ├── tool1.rs         # cargo run --bin tool1
        └── tool2.rs         # cargo run --bin tool2
```

---

### まとめ

```
✅ src/main.rs → 自動的にバイナリクレート
✅ src/lib.rs → 自動的にライブラリクレート
✅ src/bin/*.rs → 追加のバイナリクレート
✅ Cargo.toml に記述不要（規約ベース）
```

---

## Q3: mod と use の違い

**質問:** `mod` と `use` って何が違うの？

**回答:** **役割が全く違います**

### mod: モジュールを定義/宣言

```rust
// モジュールを定義
mod front_of_house {
    pub fn add() {}
}

// または別ファイルを読み込む
mod front_of_house;  // front_of_house.rs を読み込む
```

**役割:** モジュールをモジュールツリーに追加

---

### use: パスをスコープに持ち込む

```rust
use crate::front_of_house::hosting;

fn main() {
    hosting::add();  // 短く書ける
}
```

**役割:** 長いパスを短縮

---

### まとめ

```
mod = モジュールの定義/宣言（ファイル読み込み）
use = パスの短縮（便利にする）

順序:
1. mod でモジュールを定義
2. use でパスを短縮
```

---

## Q4: 絶対パスと相対パスの使い分け

**質問:** 絶対パス（`crate::`）と相対パスはどう使い分ける？

**回答:** **絶対パスが推奨**されることが多いです

### 絶対パス（推奨）

```rust
use crate::front_of_house::hosting;
```

**利点:**
- コードを移動しても動く
- 依存関係が明確

---

### 相対パス

```rust
use front_of_house::hosting;  // 現在位置から
use super::parent_function;    // 親モジュールから
```

**利点:**
- 短く書ける
- 同じモジュール内で完結

**欠点:**
- コードを移動すると壊れる可能性

---

### 使い分け

```
✅ トップレベルから使う → 絶対パス (crate::)
✅ 同じモジュール内で使う → 相対パス
✅ 親モジュールを参照 → super::
```

---

## Q5: なぜ front_of_house は pub じゃないのにアクセスできる？

**質問:** `mod front_of_house` は `pub` がないのに、なぜ `eat_at_restaurant` からアクセスできるの？

**回答:** **兄弟モジュールは pub なしでアクセス可能**です

### コード例

```rust
// src/lib.rs
mod front_of_house {        // pub なし
    pub mod hosting {
        pub fn add() {}
    }
}

pub fn eat_at_restaurant() {
    // ✅ OK！兄弟モジュールだから
    front_of_house::hosting::add();
}
```

---

### プライバシールール

| 関係 | pub 必要？ | 理由 |
|---|---|---|
| **子 → 親** | ❌ 不要 | 親は常に見える |
| **親 → 子** | ✅ 必要 | カプセル化 |
| **兄弟同士** | ❌ 不要 | 同じスコープ |

---

### まとめ

```
front_of_house と eat_at_restaurant は兄弟
→ pub なしでアクセス可能

しかし hosting と add は front_of_house の子
→ pub が必要
```

---

## Q6: super:: は Python の .. と同じ？

**質問:** `super::` は Python の `..` と同じ？

**回答:** **その通りです！**

### Rust の super::

```rust
mod parent {
    fn parent_fn() {}

    mod child {
        fn call_parent() {
            super::parent_fn();  // 親モジュールの関数
        }
    }
}
```

---

### Python の ..

```python
# myproject/parent/child.py
from .. import parent_function  # 親ディレクトリの関数
```

---

### 対応表

| Rust | Python | 説明 |
|---|---|---|
| `crate::` | プロジェクトルート | 絶対パス |
| `self::` | `.` | 現在のモジュール/ディレクトリ |
| `super::` | `..` | 親モジュール/ディレクトリ |

---

### まとめ

```
✅ super:: = 親モジュールを参照
✅ Python の .. と同じ概念
✅ 相対的なパス指定
```

---

## Q7: pub use は Python の何に相当する？

**質問:** `pub use` で再公開するのは、Python だと何？

**回答:** **`__init__.py` でのインポート**に相当します

### Rust の pub use

```rust
// src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add() {}
    }
}

// 再公開
pub use crate::front_of_house::hosting;

// 外部コード
// restaurant::hosting::add();
// (front_of_house を経由しない)
```

---

### Python の __init__.py

```python
# restaurant/__init__.py
from .front_of_house import hosting  # 再公開

# 外部コード
from restaurant import hosting
hosting.add()
# (front_of_house を経由しない)
```

---

### 目的

```
✅ 内部構造を隠す
✅ 外部APIをシンプルにする
✅ パスを短くする
```

---

### まとめ

```
Rust:   pub use で再公開
Python: __init__.py でインポート

どちらも同じ目的：
外部から使いやすいAPIを提供
```

---

## Q8: enum はデフォルトで公開？

**質問:** enum のバリアントはデフォルトで公開されている？

**回答:** **`pub enum` にすると全バリアントが自動的に公開**されます

### Enum の公開

```rust
pub enum Appetizer {
    Soup,   // 自動的に pub
    Salad,  // 自動的に pub
}
```

**理由:** バリアントが非公開だと使いにくいから

---

### Struct との違い

```rust
pub struct Breakfast {
    pub toast: String,        // 個別に pub が必要
    seasonal_fruit: String,   // 非公開
}
```

**Struct はフィールドごとに `pub` が必要**

---

### まとめ

```
pub enum → 全バリアントが公開（自動）
pub struct → フィールドは個別に pub が必要
```

---

## Q9: enum のバリアントを個別に公開できる？

**質問:** enum の一部のバリアントだけ公開できる？

**回答:** **できません**

### Enum は全か無か

```rust
pub enum Color {
    Red,    // 公開
    Green,  // 公開
    Blue,   // 公開
}

// ❌ 一部だけ公開はできない
```

**理由:** enum の設計思想として、全バリアントを扱えることが前提

---

### 代替案：Wrapper パターン

```rust
// 内部 enum（非公開）
enum ColorInternal {
    Red,
    Green,
    Blue,
    Secret,  // 隠したい
}

// 公開 enum
pub enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn to_internal(&self) -> ColorInternal {
        match self {
            Color::Red => ColorInternal::Red,
            Color::Green => ColorInternal::Green,
            Color::Blue => ColorInternal::Blue,
        }
    }
}
```

---

### まとめ

```
❌ enum のバリアントを個別に公開できない
✅ 全バリアント公開 or 全バリアント非公開
✅ 代替案：Wrapper パターン
```

---

## Q10: 同名の型が衝突したらどうする？

**質問:** `std::fmt::Result` と `std::io::Result` を両方使いたい時はどうする？

**回答:** **エイリアス（`as`）を使います**

### 問題：名前の衝突

```rust
use std::fmt::Result;
use std::io::Result;  // ❌ エラー！Result が重複

fn function1() -> Result {}   // どっちのResult？
```

---

### 解決策1: エイリアス

```rust
use std::fmt::Result;
use std::io::Result as IoResult;  // エイリアス

fn function1() -> Result {}      // fmt::Result
fn function2() -> IoResult<()> {} // io::Result
```

---

### 解決策2: フルパス

```rust
fn function1() -> std::fmt::Result {}
fn function2() -> std::io::Result<()> {}
```

---

### 解決策3: 片方だけ use

```rust
use std::io::Result;

fn function1() -> std::fmt::Result {}  // フルパス
fn function2() -> Result<()> {}        // use で短縮
```

---

### まとめ

```
名前衝突の解決法:
1. as でエイリアス（推奨）
2. フルパスで書く
3. 片方だけ use
```

---

## Q11: mod.rs と module.rs の違い

**質問:** `mod.rs` と `module.rs` はどう違う？

**回答:** **役割が違います**

### パターン1: サブモジュールなし

```
src/
├── lib.rs
└── module.rs           # 単一ファイル
```

```rust
// src/lib.rs
mod module;  // module.rs を読み込む
```

---

### パターン2: サブモジュールあり

```
src/
├── lib.rs
└── module/
    ├── mod.rs          # モジュールのルート
    └── submodule.rs    # サブモジュール
```

```rust
// src/lib.rs
mod module;  // module/mod.rs を読み込む

// src/module/mod.rs
pub mod submodule;  // submodule.rs を読み込む
```

---

### Rust 2018 以降の新スタイル

```
src/
├── lib.rs
├── module.rs           # モジュールのルート
└── module/
    └── submodule.rs
```

```rust
// src/lib.rs
mod module;

// src/module.rs
pub mod submodule;
```

**`mod.rs` が不要になった！**

---

### まとめ

```
module.rs    = 単一ファイルモジュール
module/mod.rs = サブモジュールがある場合のルート

Rust 2018+:
module.rs でもサブモジュールを持てる（推奨）
```

---

## Q12: main.rs で lib.rs を使う方法

**質問:** `main.rs` で `lib.rs` の関数を使いたい。`mod lib;` でいい？

**回答:** **❌ `mod lib;` はNG！クレート名を使います**

### 間違った方法

```rust
// src/main.rs
mod lib;  // ❌ エラー！

use lib::function;
```

**エラー:**
```
warning: found module declaration for lib.rs
= note: lib.rs is the root of this crate's library target
```

---

### 正しい方法

```rust
// src/main.rs
use my_crate::function;  // ✅ クレート名を使う

fn main() {
    function();
}
```

**クレート名は `Cargo.toml` で定義:**
```toml
[package]
name = "my_crate"
```

---

### 実例

```rust
// Cargo.toml
[package]
name = "restaurant"

// src/lib.rs
pub fn eat_at_restaurant() {
    println!("Welcome!");
}

// src/main.rs
use restaurant::eat_at_restaurant;  // クレート名 restaurant

fn main() {
    eat_at_restaurant();
}
```

---

### まとめ

```
❌ mod lib;
✅ use crate_name::...

lib.rs はライブラリのルート
→ クレート名でアクセス
```

---

## Q13: ファイルが見つからないエラー

**質問:** `error[E0583]: file not found for module` が出る。どうすれば？

**回答:** **ファイルの配置が間違っています**

### エラー例

```
error[E0583]: file not found for module `front_of_house`
--> src/lib.rs:1:1
  |
1 | mod front_of_house;
  | ^^^^^^^^^^^^^^^^^^^
  |
  = help: to create the module `front_of_house`, create file
          "src/front_of_house.rs" or "src/front_of_house/mod.rs"
```

---

### 原因と解決策

#### 原因1: ファイルがない

```
src/
└── lib.rs              # mod front_of_house; を書いた

❌ front_of_house.rs がない！
```

**解決策:**
```
src/
├── lib.rs
└── front_of_house.rs   # ✅ ファイルを作成
```

---

#### 原因2: ファイルの場所が間違っている

```
src/
├── lib.rs
└── modules/
    └── front_of_house.rs   # ❌ 場所が違う
```

**解決策:**
```
src/
├── lib.rs
└── front_of_house.rs       # ✅ src 直下
```

---

#### 原因3: サブモジュールの配置ミス

```
src/
├── lib.rs
├── front_of_house.rs       # mod.rs がない
└── front_of_house/
    └── hosting.rs          # ❌ サブモジュールが読み込まれない
```

**解決策:**
```
src/
├── lib.rs
└── front_of_house/
    ├── mod.rs              # ✅ ルートファイル
    └── hosting.rs
```

---

### まとめ

```
mod module_name; を書いたら:
✅ src/module_name.rs を作成
または
✅ src/module_name/mod.rs を作成
```

---

## Q14: 複数のバイナリを持つプロジェクト

**質問:** 1つのプロジェクトで複数の実行ファイルを作りたい。どうする？

**回答:** **`src/bin/` ディレクトリを使います**

### ディレクトリ構造

```
my_project/
├── Cargo.toml
└── src/
    ├── lib.rs              # 共通ライブラリ
    ├── main.rs             # メインバイナリ
    └── bin/
        ├── tool1.rs        # ツール1
        └── tool2.rs        # ツール2
```

---

### 実行方法

```bash
# メインバイナリ
cargo run

# tool1
cargo run --bin tool1

# tool2
cargo run --bin tool2
```

---

### コード例

```rust
// src/lib.rs
pub fn shared_function() {
    println!("共有関数");
}

// src/main.rs
use my_project::shared_function;

fn main() {
    println!("メインバイナリ");
    shared_function();
}

// src/bin/tool1.rs
use my_project::shared_function;

fn main() {
    println!("ツール1");
    shared_function();
}
```

---

### まとめ

```
✅ src/bin/*.rs = 追加のバイナリ
✅ cargo run --bin <name> で実行
✅ 共通コードは src/lib.rs に
```

---

## Q15: グロブ演算子（*）はいつ使う？

**質問:** `use std::collections::*;` ってどんな時に使うの？

**回答:** **慎重に使うべき**です

### グロブ演算子とは

```rust
use std::collections::*;  // 全ての公開要素をインポート

let map = HashMap::new();  // HashMap はどこから？
let set = HashSet::new();  // HashSet はどこから？
```

---

### 問題点

```rust
use std::collections::*;
use std::io::*;

// Result はどっち？
// - std::io::Result
// - 他の Result
```

**名前の由来が分かりにくい**

---

### 使ってもいい場面

#### 1. テストコード

```rust
#[cfg(test)]
mod tests {
    use super::*;  // 親モジュールの全てをインポート

    #[test]
    fn test_something() {
        // テスト
    }
}
```

#### 2. Prelude パターン

```rust
// src/prelude.rs
pub use crate::Error;
pub use crate::Result;
pub use crate::Config;

// 外部コード
use my_crate::prelude::*;  // よく使う型をまとめて
```

---

### まとめ

```
❌ 通常コード：避ける（由来が不明確）
✅ テストコード：OK（super::* が便利）
✅ Prelude：OK（明示的なパターン）
```

---

## Q16: Python のパッケージ管理と比較

**質問:** Rust のモジュールシステムは Python と何が違う？

**回答:** **コンパイル時にチェックされる点が大きく違います**

### Python

```python
# 実行時エラー
import nonexistent_module  # 実行するまで分からない

from module import function
function()  # 存在しなければ実行時エラー
```

**特徴:**
- 実行時にチェック
- 柔軟（動的）
- エラーは実行してから分かる

---

### Rust

```rust
// コンパイル時エラー
mod nonexistent_module;  // コンパイル時に検出

use crate::module::function;
function();  // 存在しなければコンパイルエラー
```

**特徴:**
- コンパイル時にチェック
- 厳格（静的）
- エラーは実行前に分かる

---

### 対応表

| 概念 | Python | Rust |
|---|---|---|
| **パッケージ** | `__init__.py` のあるディレクトリ | Cargo.toml のプロジェクト |
| **モジュール** | `.py` ファイル | `mod` 宣言 |
| **インポート** | `import` / `from ... import` | `use` |
| **公開** | デフォルト公開 | デフォルト非公開（`pub`） |
| **チェック** | 実行時 | コンパイル時 |

---

### まとめ

```
Python: 柔軟だがエラーは実行時
Rust:   厳格だがエラーはコンパイル時

Rust の方が安全だが、学習コストは高い
```

---

## 重要な概念のまとめ

### パッケージとクレート

```
パッケージ = プロジェクト全体
クレート = コンパイル単位（バイナリ or ライブラリ）
```

---

### mod と use

```
mod = モジュールの定義/宣言
use = パスの短縮
```

---

### プライバシー

```
デフォルト = 非公開
pub = 公開
親 → 子: pub が必要
子 → 親: 常にアクセス可
兄弟同士: pub 不要
```

---

### パス

```
crate:: = 絶対パス（推奨）
module:: = 相対パス
super:: = 親モジュール
self:: = 現在のモジュール
```

---

### ファイル配置

```
単一ファイル: src/module.rs
サブモジュール: src/module/mod.rs + サブモジュール
Rust 2018+: src/module.rs + src/module/*.rs
```

---

### 公開の違い

```
pub struct → フィールドは個別に pub
pub enum → 全バリアントが自動的に pub
```

---

### 再公開

```
pub use = 外部APIをシンプルにする
Python の __init__.py と同じ概念
```
