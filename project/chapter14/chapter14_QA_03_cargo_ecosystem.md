# 第14章 Q&A (3/3): Cargoエコシステム

## Q1: `extern crate` って何？今も使う？

### 質問の背景

```rust
extern crate add_one;

fn main() {
    add_one::add_one(10);
}
```

`extern crate` というキーワードが出てきた。これは何？

### 答え

**`extern crate` は他のクレートを使うための宣言ですが、Rust 2018以降は基本的に不要です。**

### Rust 2015（古い）

```rust
// 外部クレートを使う時は extern crate が必要
extern crate serde;
extern crate add_one;

fn main() {
    add_one::add_one(10);
}
```

### Rust 2018以降（現在）

```rust
// extern crate 不要！
// Cargo.toml に書けば自動で使える

fn main() {
    add_one::add_one(10);  // そのまま使える
}
```

```toml
# Cargo.toml
[dependencies]
add-one = { path = "../add-one" }
```

### なぜ不要になったか

**Rust 2018のモジュールシステム改善:**
- Cargo.tomlに書けば自動でインポート
- よりシンプルで分かりやすい
- 書くべきコードが減る

### まだ使う場合

#### 1. クレート名とパッケージ名が違う時

```toml
# Cargo.toml
[dependencies]
actix-web = "4.0"
```

```rust
// クレート名が actix_web（ハイフン → アンダースコア）
use actix_web::App;

// extern crate は不要（自動で変換される）
```

#### 2. マクロをインポートする時（特殊ケース）

```rust
// 一部の古いマクロでは必要な場合も
#[macro_use]
extern crate serde_derive;
```

ただし、最近のクレートは `use` で済む：

```rust
// 現在の推奨方法
use serde::Deserialize;
```

### `extern` の他の用途

#### FFI (Foreign Function Interface)

```rust
// C言語の関数を呼ぶ
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("abs(-3) = {}", abs(-3));
    }
}
```

これは `extern crate` とは別物で、今も使います。

### Pythonとの比較

**Python:**
```python
# インポートが常に必要
import json
from datetime import datetime
```

**Rust (2015):**
```rust
// 外部クレートは宣言が必要
extern crate serde;
use serde::Serialize;
```

**Rust (2018+):**
```rust
// Pythonと同じように直接 use できる
use serde::Serialize;
```

### まとめ

- `extern crate` = 外部クレートの宣言（古い方法）
- Rust 2018以降は基本的に不要
- Cargo.tomlに書けば自動でインポート
- FFIの `extern "C"` は別物（今も使う）

---

## Q2: `cargo install` って Python の poetry と同じ？

### 質問の背景

`cargo install` でツールをインストールできると聞いたが、Pythonのpoetryと同じような感じ？

### 答え

**`cargo install` は `pipx` に近いです。poetryとは違います。**

### 役割の違い

| Rust | Python | 用途 |
|------|--------|------|
| `cargo install` | `pipx install` | **グローバル**にCLIツールをインストール |
| `Cargo.toml [dependencies]` | `poetry add` / `pip install` | **プロジェクト**の依存関係管理 |

### cargo install の使い方

```bash
# グローバルにCLIツールをインストール
cargo install ripgrep

# インストール先
~/.cargo/bin/rg

# 使える！
rg "pattern" .
```

**特徴:**
- バイナリクレート（実行可能なプログラム）のみ
- システム全体で使えるツールをインストール
- プロジェクトの依存関係ではない

### Cargo.toml の依存関係

```toml
# Cargo.toml
[dependencies]
serde = "1.0"
```

```bash
# プロジェクトをビルド
cargo build  # serdeがダウンロードされる
```

**特徴:**
- ライブラリクレートを依存関係として追加
- このプロジェクトでのみ使える
- プロジェクトごとに管理

### Pythonとの詳細な比較

#### グローバルツールのインストール

**Python:**
```bash
# pipx（推奨）
pipx install black

# pip（非推奨、環境を汚染）
pip install black
```

**Rust:**
```bash
# cargo install
cargo install cargo-watch
```

#### プロジェクトの依存関係

**Python:**
```bash
# poetry
poetry add requests

# pip
pip install requests
```

**Rust:**
```bash
# cargo add（cargo-editが必要）
cargo add serde

# または手動でCargo.tomlに追加
```

### なぜ cargo install は poetry ではないか

**poetry の役割:**
1. 依存関係の管理
2. 仮想環境の管理
3. パッケージのビルドと公開

**cargo の役割:**
1. ビルドシステム（`cargo build`）
2. 依存関係の管理（`Cargo.toml`）
3. パッケージの公開（`cargo publish`）
4. **+ ツールのインストール**（`cargo install`）

**つまり:**
- `cargo` = `poetry` + `pipx` + `setuptools` + `build`
- Rustは1つのツールで完結
- Pythonは複数のツールが必要

### 実例

#### Python

```bash
# 開発環境のセットアップ
poetry install           # 依存関係をインストール

# CLIツールのインストール
pipx install black       # グローバルツール
pipx install flake8
```

#### Rust

```bash
# 開発環境のセットアップ
cargo build              # 依存関係を自動でインストール

# CLIツールのインストール
cargo install cargo-watch  # グローバルツール
cargo install cargo-edit
```

### まとめ

- `cargo install` ≠ `poetry`
- `cargo install` = `pipx install`（グローバルツール）
- `Cargo.toml` = `pyproject.toml`（プロジェクト依存関係）
- `cargo` 1つで全部できる

---

## Q3: Cargoのサブコマンドを増やせるってどういうこと？

### 質問の背景

「`cargo-something` をインストールすると `cargo something` として使える」と聞いたが、どういう仕組み？

### 答え

**Cargoは拡張可能に設計されています。命名規則に従ったバイナリをインストールすれば、自動的にサブコマンドとして認識されます。**

### 仕組み

#### 1. 命名規則

```
バイナリ名: cargo-xxx
↓ 自動的に
サブコマンド: cargo xxx
```

#### 2. インストール

```bash
# cargo-watch をインストール
cargo install cargo-watch

# インストール先
~/.cargo/bin/cargo-watch
```

#### 3. 実行

```bash
# cargo watch として実行できる
cargo watch -x run

# 実際には以下を実行している
~/.cargo/bin/cargo-watch -x run
```

### 動作の詳細

**Cargoの動作:**
1. `cargo xxx` が実行される
2. 組み込みコマンドに `xxx` がない
3. `$PATH` から `cargo-xxx` を探す
4. 見つかったら実行する

### 一覧表示

```bash
cargo --list

# 出力例:
# Installed Commands:
#     build
#     run
#     test
#     ...
#     watch    ← cargo-watch
#     edit     ← cargo-edit
```

### 自作サブコマンドの作成

#### ステップ1: プロジェクト作成

```bash
cargo new cargo-hello --bin
cd cargo-hello
```

#### ステップ2: 実装

```rust
// src/main.rs
fn main() {
    println!("Hello from cargo-hello!");
}
```

#### ステップ3: Cargo.toml 設定

```toml
[package]
name = "cargo-hello"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "cargo-hello"  # 重要: cargo-で始まる名前
path = "src/main.rs"
```

#### ステップ4: インストール

```bash
# ローカルにインストール
cargo install --path .

# 使える！
cargo hello
# → "Hello from cargo-hello!"
```

### crates.ioに公開

```bash
# 公開
cargo publish

# 誰でもインストールできる
cargo install cargo-hello

# 誰でも使える
cargo hello
```

### 人気の拡張コマンド

```bash
# コード変更を監視して自動再ビルド
cargo install cargo-watch
cargo watch -x run

# 依存関係を簡単に追加・削除
cargo install cargo-edit
cargo add serde
cargo rm tokio

# マクロ展開を表示
cargo install cargo-expand
cargo expand

# 依存関係のツリー表示
cargo install cargo-tree
cargo tree
```

### Pythonとの比較

**Python:**
- CLIツールは独自のコマンド名
- パッケージマネージャーと統合されていない

```bash
pip install black
black file.py  # blackコマンド

pip install flake8
flake8 file.py  # flake8コマンド
```

**Rust:**
- すべて `cargo` で統一
- 発見しやすい
- 一貫した体験

```bash
cargo install cargo-watch
cargo watch    # cargoサブコマンド

cargo install cargo-edit
cargo add      # cargoサブコマンド
```

### メリット

1. **統一されたCLI体験**: すべて `cargo xxx`
2. **発見しやすい**: `cargo --list` で確認できる
3. **コミュニティ主導**: 誰でも拡張を作れる
4. **配布が簡単**: crates.ioで公開、`cargo install` で完了

### まとめ

- `cargo-xxx` → `cargo xxx` として実行できる
- 命名規則に従うだけで自動認識
- 誰でも拡張を作ってcrates.ioに公開できる
- Rustエコシステムの拡張性の高さ

---

## Q4: crates.io と cargo install と Cargo.toml の関係は？

### 質問の背景

crates.io、cargo install、Cargo.tomlの関係がごちゃごちゃになってきた...

### 答え

**それぞれ役割が違います。整理しましょう。**

### 全体像

```
crates.io (レジストリ)
    ↓ 公開
[開発者がクレートを公開]
    ↓ インストール/依存
[ユーザーが使う]
    ├─ cargo install（グローバルツール）
    └─ Cargo.toml（プロジェクト依存関係）
```

### 1. crates.io - パッケージレジストリ

**役割:** すべてのRustクレートが公開される場所

```bash
# 公開
cargo publish

# 検索
https://crates.io/
```

**特徴:**
- オープンソースのみ
- 無料
- 永久に削除不可

### 2. cargo install - グローバルツール

**役割:** CLIツールをシステム全体にインストール

```bash
# インストール
cargo install ripgrep

# インストール先
~/.cargo/bin/rg

# 使用
rg "pattern" .
```

**対象:**
- ✅ バイナリクレート（src/main.rs）
- ❌ ライブラリクレート（src/lib.rsのみ）

### 3. Cargo.toml - プロジェクト依存関係

**役割:** プロジェクトで使うライブラリを管理

```toml
# Cargo.toml
[dependencies]
serde = "1.0"
tokio = "1.0"
```

```bash
# ビルド時に自動でダウンロード
cargo build
```

**対象:**
- ✅ ライブラリクレート
- ✅ バイナリクレート（まれ）

### 使い分け

| やりたいこと | 使うもの | 例 |
|------------|---------|---|
| クレートを公開 | crates.io | `cargo publish` |
| CLIツールをインストール | `cargo install` | `cargo install ripgrep` |
| プロジェクトにライブラリを追加 | `Cargo.toml` | `serde = "1.0"` |

### 具体例

#### 開発者の視点

```bash
# 1. クレートを作る
cargo new my-library --lib

# 2. crates.ioに公開
cargo publish
```

#### ユーザーの視点（ライブラリの場合）

```toml
# Cargo.toml に追加
[dependencies]
my-library = "1.0"
```

```bash
cargo build  # 自動でダウンロードされる
```

#### ユーザーの視点（CLIツールの場合）

```bash
# グローバルにインストール
cargo install my-tool

# どこからでも実行できる
my-tool --help
```

### ワークスペースでの依存関係共有

```toml
# workspace/Cargo.toml
[workspace]
members = ["crate1", "crate2"]

# crate1/Cargo.toml
[dependencies]
serde = "1.0"

# crate2/Cargo.toml
[dependencies]
serde = "1.0"  # 同じバージョン
```

**結果:**
- `serde` は1回だけダウンロード
- ワークスペース全体で共有
- Cargo.lockで統一

### Pythonとの完全な比較

| Rust | Python | 役割 |
|------|--------|------|
| crates.io | PyPI | パッケージレジストリ |
| `cargo publish` | `twine upload` | パッケージ公開 |
| `cargo install` | `pipx install` | グローバルツール |
| `Cargo.toml [dependencies]` | `pyproject.toml [dependencies]` | プロジェクト依存関係 |
| `cargo build` | `poetry install` | 依存関係インストール |

### まとめ

1. **crates.io**: パッケージが公開される場所
2. **cargo install**: システム全体で使うCLIツール
3. **Cargo.toml**: プロジェクトで使うライブラリ

**覚え方:**
- 公開 → `crates.io`
- グローバルツール → `cargo install`
- プロジェクト依存 → `Cargo.toml`

前のQ&A: [chapter14_QA_02_crates_io_operations.md](chapter14_QA_02_crates_io_operations.md) - crates.io操作
