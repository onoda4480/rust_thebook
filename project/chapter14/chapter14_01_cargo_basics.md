# 第14章まとめ (1/3): Cargoの基本機能

## 1. リリースプロファイル

Cargoには異なるビルド設定を持つプロファイルがあります。

### dev プロファイル（開発用）

```toml
[profile.dev]
opt-level = 0  # 最適化なし（デフォルト）
```

**使用:**
```bash
cargo build      # devプロファイル
cargo run        # devプロファイル
```

**特徴:**
- コンパイルが速い
- 実行速度は遅い
- デバッグ情報が含まれる

### release プロファイル（本番用）

```toml
[profile.release]
opt-level = 3  # 最大限の最適化（デフォルト）
```

**使用:**
```bash
cargo build --release
cargo run --release
```

**特徴:**
- コンパイルが遅い
- 実行速度が速い
- バイナリサイズが最適化される

### opt-level の値

| レベル | 説明 |
|--------|------|
| 0 | 最適化なし（dev デフォルト） |
| 1 | 基本的な最適化 |
| 2 | より多くの最適化 |
| 3 | 最大限の最適化（release デフォルト） |

### Pythonとの比較

**Python:**
- デバッグビルドと最適化ビルドの区別がない
- `python -O` で最適化（pyc生成）程度

**Rust:**
- dev と release で明確に分離
- パフォーマンスの差が大きい

## 2. ドキュメンテーションコメント

### `///` - アイテムのドキュメント

関数、構造体、モジュールなどの**前**に書きます。

```rust
/// 2つの数値を足し合わせます。
///
/// # Examples
///
/// ```
/// let result = my_crate::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

**Markdown が使える:**
- `#` 見出し
- コードブロック
- リスト

**一般的なセクション:**
- `# Examples` - 使用例
- `# Panics` - パニックする条件
- `# Errors` - エラーが返る条件
- `# Safety` - unsafeコードの安全性

### `//!` - モジュール/クレートのドキュメント

ファイルの**先頭**に書きます。

```rust
//! # My Crate
//!
//! クレート全体の説明をここに書きます。
//!
//! ## 使い方
//!
//! ```
//! use my_crate::add;
//! ```

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### ドキュメント生成

```bash
cargo doc           # ドキュメント生成
cargo doc --open    # 生成して自動的にブラウザで開く
```

生成先: `target/doc/`

### Pythonとの比較

**Python:**
```python
def add(a, b):
    """2つの数値を足し合わせます。

    Args:
        a: 1つ目の数値
        b: 2つ目の数値

    Returns:
        合計値
    """
    return a + b
```

**Rust:**
```rust
/// 2つの数値を足し合わせます。
///
/// # Arguments
///
/// * `a` - 1つ目の数値
/// * `b` - 2つ目の数値
///
/// # Returns
///
/// 合計値
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## 3. pub use による再エクスポート

APIを使いやすくするために、内部構造を隠して公開します。

### 問題: 深いモジュール構造

```rust
// lib.rs
pub mod kinds {
    pub enum PrimaryColor {
        Red, Yellow, Blue,
    }
}

pub mod utils {
    use crate::kinds::*;
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) { }
}
```

**ユーザーの使い方（面倒）:**
```rust
use my_crate::kinds::PrimaryColor;
use my_crate::utils::mix;
```

### 解決: pub use で再エクスポート

```rust
// lib.rs
pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds {
    pub enum PrimaryColor {
        Red, Yellow, Blue,
    }
    pub enum SecondaryColor {
        Orange, Green, Purple,
    }
}

pub mod utils {
    use crate::kinds::*;
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // ...
    }
}
```

**ユーザーの使い方（簡単）:**
```rust
use my_crate::{PrimaryColor, mix};

let color = mix(PrimaryColor::Red, PrimaryColor::Yellow);
```

### メリット

- ✅ 内部構造を自由に変更できる
- ✅ ユーザーに分かりやすいAPI
- ✅ ドキュメントがシンプルになる

### Pythonとの比較

**Python (`__init__.py`):**
```python
# mypackage/__init__.py
from .kinds import PrimaryColor
from .utils import mix

__all__ = ['PrimaryColor', 'mix']
```

**Rust (`lib.rs`):**
```rust
pub use kinds::PrimaryColor;
pub use utils::mix;
```

どちらも同じ目的: パッケージのルートから直接インポートできるようにする

## まとめ

1. **リリースプロファイル**: dev（開発）と release（本番）で最適化レベルを使い分け
2. **ドキュメント**: `///` と `//!` でMarkdown形式のドキュメントを書く
3. **pub use**: 内部構造を隠して使いやすいAPIを提供

次のファイル: [chapter14_02_crates_io.md](chapter14_02_crates_io.md) - crates.ioへの公開
