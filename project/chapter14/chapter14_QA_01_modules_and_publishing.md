# 第14章 Q&A (1/3): モジュールシステムと公開の準備

## Q1: `use kinds::*;` でエラーが出る。なぜ？

### 質問の背景

```rust
pub mod kinds {
    pub enum PrimaryColor {
        Red, Yellow, Blue,
    }
}

pub mod utils {
    use kinds::*;  // ❌ エラー！
    // error[E0432]: unresolved import `kinds`
}
```

### 答え

**Rust 2018エディション以降、モジュールパスの書き方が変わりました。**

同じクレート内のモジュールを参照する場合は、`crate::` をつける必要があります。

```rust
pub mod utils {
    use crate::kinds::*;  // ✅ 正しい
}
```

### Rust 2015 vs 2018のモジュールシステム

#### Rust 2015（古い）

```rust
// 同じクレート内のモジュール
use kinds::*;           // OK

// 外部クレート
extern crate serde;     // 必要
use serde::Serialize;   // OK
```

#### Rust 2018以降（現在）

```rust
// 同じクレート内のモジュール
use crate::kinds::*;    // crate:: が必要

// 外部クレート
// extern crate 不要！
use serde::Serialize;   // OK
```

### モジュールパスの種類

| パス | 意味 | 例 |
|------|------|---|
| `crate::` | クレートのルートから | `use crate::kinds::*;` |
| `self::` | 現在のモジュールから | `use self::submodule::Item;` |
| `super::` | 親モジュールから | `use super::parent_item;` |
| クレート名 | 外部クレート | `use serde::Serialize;` |

### Pythonとの比較

**Python:**
```python
# mypackage/utils.py
from .kinds import PrimaryColor  # 相対インポート（.）
from mypackage.kinds import PrimaryColor  # 絶対インポート
```

**Rust:**
```rust
// src/utils.rs
use crate::kinds::PrimaryColor;  // 絶対パス（crate::）
use super::kinds::PrimaryColor;  // 相対パス（super::）
```

### まとめ

- ✅ Rust 2018以降は `use crate::kinds::*;`
- ❌ `use kinds::*;` は古い書き方
- `crate::` = Pythonの絶対インポート
- `super::` = Pythonの相対インポート（`..`）

---

## Q2: edition = "2024" でエラーが出る。なぜ？

### 質問の背景

```toml
[package]
name = "my-crate"
edition = "2024"  # ❌ エラー！
```

```
error: failed to parse manifest
  Caused by:
    unsupported edition: 2024
```

### 答え

**Rust 2024エディションはまだ存在しません。**

現在利用可能なエディション：
- `"2015"` - 初期バージョン
- `"2018"` - 2018年リリース（モジュールシステム改善）
- `"2021"` - 2021年リリース（最新の安定版）

```toml
[package]
edition = "2021"  # ✅ 正しい
```

### Rustのエディションとは

**互換性を保ちながら言語を進化させる仕組み**

- 新しいエディションは3年ごと（2015 → 2018 → 2021 → 2024?）
- 異なるエディションのコードは相互運用可能
- 古いコードは動き続ける（破壊的変更なし）

### エディションごとの主な変更

| エディション | 主な変更 |
|-------------|---------|
| 2015 | 初期バージョン |
| 2018 | モジュールシステム改善、`dyn Trait`、NLL |
| 2021 | Disjoint capture、`IntoIterator` for arrays |

### Pythonとの比較

**Python:**
- Python 2 → Python 3（互換性なし）
- `from __future__ import` で新機能を有効化

**Rust:**
- エディション間で完全な互換性
- 新旧のコードが同じプロジェクトで共存可能
- `edition = "2021"` で新しい機能を使う

### まとめ

- ✅ `edition = "2021"` を使う（最新の安定版）
- ❌ `edition = "2024"` はまだ存在しない
- エディションは互換性を保つ仕組み

---

## Q3: クレート名が既に存在する。どうすればいい？

### 質問の背景

```bash
$ cargo publish
error: crate cargo_and_crates@0.1.0 already exists on crates.io index
```

### 答え

**crates.ioのクレート名は一意（ユニーク）です。早い者勝ちです。**

### 解決方法

#### 1. 別の名前に変更する

```toml
[package]
name = "my-unique-name"  # ユニークな名前に変更
```

**ネーミングのコツ:**
- プロジェクト名を含める: `myproject-utils`
- 自分の名前を含める: `hikaruonoda-tools`
- 機能を明確にする: `rust-json-parser`
- 学習用なら明示: `learning-rust-example`

#### 2. 名前を検索する

```bash
# crates.ioで検索
# https://crates.io/search?q=my-crate-name
```

ブラウザで事前に確認するのがおすすめ。

#### 3. プライベートにする

公開しないなら、名前の重複は問題ありません。

```toml
# Cargo.toml
[package]
name = "my-crate"  # 同じ名前でもOK
publish = false    # 公開しない
```

### crates.io の命名ルール

**許可される文字:**
- 小文字の英字
- 数字
- `-`（ハイフン）
- `_`（アンダースコア）

**禁止される文字:**
- 大文字
- スペース
- その他の記号

### Pythonとの比較

**Python (PyPI):**
- 同じくユニークな名前が必要
- 大文字小文字は区別されない（`MyPackage` = `mypackage`）
- ハイフンとアンダースコアは同一視される（`my-package` = `my_package`）

**Rust (crates.io):**
- ユニークな名前が必要
- すべて小文字に正規化される
- ハイフンとアンダースコアは区別される

### まとめ

- ✅ クレート名はユニーク、早い者勝ち
- ✅ 事前にcrates.ioで検索して確認
- ✅ 学習用なら分かりやすい名前にする
- ✅ 公開しないなら `publish = false`

---

## Q4: バイナリクレートとライブラリクレート、どちらを公開できる？

### 質問の背景

「crates.ioはOSSのライブラリを公開できる」と聞いたが、バイナリ（CLIツール）も公開できる？

### 答え

**どちらも公開できます！むしろライブラリの方が多いです。**

### ライブラリクレート（src/lib.rs）

**定義:**
- 他のプロジェクトから依存関係として使われる
- 単独では実行できない

**公開:**
```bash
cargo publish
```

**使用:**
```toml
# 他の人のプロジェクト
[dependencies]
serde = "1.0"
```

**例:**
- `serde` - シリアライゼーション
- `tokio` - 非同期ランタイム
- `clap` - CLIパーサー

### バイナリクレート（src/main.rs）

**定義:**
- 実行可能なプログラム
- CLIツールなど

**公開:**
```bash
cargo publish
```

**使用:**
```bash
# 他の人がインストール
cargo install ripgrep
```

**例:**
- `ripgrep` - 高速grep
- `cargo-watch` - ファイル監視

### 両方を持つクレート

```
my-crate/
├── Cargo.toml
└── src/
    ├── lib.rs   # ライブラリ機能
    └── main.rs  # CLIツール
```

**使用:**
```toml
# ライブラリとして
[dependencies]
clap = "4.0"
```

```bash
# CLIツールとして
cargo install clap
```

### crates.io の統計

| クレートの種類 | 割合 |
|--------------|------|
| ライブラリクレート | 90%以上 |
| バイナリクレート | 10%未満 |
| 両方 | 少数 |

### Pythonとの比較

**Python (PyPI):**
- ライブラリもCLIツールも公開可能
- `pip install` で両方インストール
- CLIツールは `console_scripts` で定義

**Rust (crates.io):**
- ライブラリもバイナリも公開可能
- ライブラリ: `cargo add`
- バイナリ: `cargo install`

### まとめ

- ✅ ライブラリクレート: 公開できる（メイン用途）
- ✅ バイナリクレート: 公開できる（少数派）
- ✅ 両方持つクレート: 公開できる
- crates.ioの90%以上はライブラリクレート

次のQ&A: [chapter14_QA_02_crates_io_operations.md](chapter14_QA_02_crates_io_operations.md) - crates.io操作
