# 第14章まとめ (3/3): ワークスペースとCargoの拡張

## 1. ワークスペースとは

**複数のクレートを1つのプロジェクトで管理する仕組み**

### ディレクトリ構造

```
workspace/
├── Cargo.toml       # ワークスペースのルート
├── Cargo.lock       # 共有される依存関係のロック
├── crate1/
│   ├── Cargo.toml   # crate1の設定
│   └── src/
│       └── lib.rs
└── crate2/
    ├── Cargo.toml   # crate2の設定
    └── src/
        └── main.rs
```

### ワークスペースのCargo.toml

```toml
[workspace]
resolver = "2"  # 依存関係の解決方法（2021エディション以降は"2"）

members = [
    "crate1",
    "crate2",
]
```

**重要:** ワークスペースのルートには `[dependencies]` セクションを書けない！

### メンバークレートのCargo.toml

```toml
# crate2/Cargo.toml
[package]
name = "crate2"
version = "0.1.0"
edition = "2021"

[dependencies]
crate1 = { path = "../crate1" }  # 同じワークスペース内のクレート
```

## 2. ワークスペースのメリット

### 1. 依存関係の共有

**問題:**
- 複数のクレートで同じ依存関係を使う
- 別々にダウンロードすると無駄

**解決:**
```toml
# crate1/Cargo.toml
[dependencies]
serde = "1.0"

# crate2/Cargo.toml
[dependencies]
serde = "1.0"  # 同じバージョン
```

**結果:**
- ✅ `serde` は1回だけダウンロードされる
- ✅ ワークスペース全体で同じバージョンを使用
- ✅ Cargo.lockで統一が保証される
- ✅ ディスク容量の節約

### 2. ビルドの効率化

```bash
# ワークスペース全体をビルド
cargo build

# 特定のクレートをビルド
cargo build -p crate2

# 特定のクレートを実行
cargo run -p crate2
```

### 3. テストの一括実行

```bash
# ワークスペース全体のテストを実行
cargo test

# 特定のクレートのテストを実行
cargo test -p crate1
```

## 3. Pythonとの比較

### Python (モノレポ)

```
project/
├── pyproject.toml  # プロジェクト全体の設定
├── package1/
│   ├── __init__.py
│   └── module.py
└── package2/
    ├── __init__.py
    └── module.py
```

**特徴:**
- 依存関係は共有されない（各環境にインストール）
- モノレポツール（Poetry, Pipenv）でもワークスペースのような機能は限定的

### Rust (ワークスペース)

```
workspace/
├── Cargo.toml
├── crate1/
└── crate2/
```

**特徴:**
- ✅ 依存関係が自動的に共有される
- ✅ Cargoがネイティブでサポート
- ✅ ビルドキャッシュも共有

## 4. ワークスペースの注意点

### ❌ ルートには依存関係を書けない

```toml
# ❌ エラー！
[workspace]
members = ["crate1", "crate2"]

[dependencies]  # ← これは書けない
serde = "1.0"
```

エラー:
```
error: this virtual manifest specifies a `dependencies` section, which is not allowed
```

### ✅ 各メンバーに書く

```toml
# crate1/Cargo.toml
[dependencies]
serde = "1.0"

# crate2/Cargo.toml
[dependencies]
serde = "1.0"  # 同じバージョンを書く
```

### resolver の設定

```toml
[workspace]
resolver = "2"  # Rust 2021エディションでは必須
```

**resolver のバージョン:**
- `"1"` - 古い依存関係解決方法
- `"2"` - Rust 2018/2021での推奨

## 5. Cargoの拡張機能

### カスタムサブコマンド

**仕組み:**
- `cargo-xxx` という名前のバイナリを作る
- `~/.cargo/bin/` にインストールする
- `cargo xxx` として実行できる

### 例: cargo-watch の作成

```bash
# 1. cargo-watch をインストール
cargo install cargo-watch

# 2. cargo watch として実行できる
cargo watch -x run
```

### 命名規則

```
バイナリ名: cargo-mycommand
↓
サブコマンド: cargo mycommand
```

### 一覧表示

```bash
cargo --list

# 出力例:
#     build
#     run
#     test
#     watch    ← cargo-watch をインストールした
#     edit     ← cargo-edit をインストールした
```

### 自作サブコマンドの公開

```toml
# Cargo.toml
[package]
name = "cargo-mycommand"  # cargo-で始まる名前

[[bin]]
name = "cargo-mycommand"  # バイナリ名
path = "src/main.rs"
```

```bash
# crates.ioに公開
cargo publish

# 誰でもインストールできる
cargo install cargo-mycommand

# 使える！
cargo mycommand
```

### Pythonとの比較

**Python:**
- CLIツールは独自のコマンド名を持つ
- `pip install black` → `black` コマンド
- パッケージマネージャーと統合されていない

**Rust:**
- `cargo install cargo-watch` → `cargo watch`
- ✅ すべて `cargo` で統一
- ✅ 発見しやすい（`cargo --list`）
- ✅ エコシステムが統一されている

## 6. ライブラリクレート vs バイナリクレート

### ライブラリクレート

**定義:**
- `src/lib.rs` を持つ
- 他のプロジェクトから依存関係として使われる

**公開:**
```bash
cargo publish
```

**使用:**
```toml
[dependencies]
my-library = "1.0"
```

### バイナリクレート

**定義:**
- `src/main.rs` を持つ
- 実行可能なプログラム

**公開:**
```bash
cargo publish
```

**使用:**
```bash
cargo install my-tool
```

### 両方を持つクレート

```
my-crate/
├── Cargo.toml
└── src/
    ├── lib.rs   # ライブラリ機能
    └── main.rs  # CLIツール
```

**使用例:**
```toml
# ライブラリとして使う
[dependencies]
clap = "4.0"
```

```bash
# CLIツールとしてインストール
cargo install clap
```

## まとめ

1. **ワークスペース**: 複数のクレートを1つのプロジェクトで管理
   - 依存関係の共有
   - ビルドの効率化
   - ルートには `[dependencies]` を書けない

2. **依存関係の共有**: 同じバージョンは1回だけダウンロード

3. **Cargoの拡張**: `cargo-xxx` で `cargo xxx` サブコマンドを追加
   - 統一されたCLI体験
   - コミュニティが自由に拡張できる

4. **クレートの種類**:
   - ライブラリ（lib.rs）: `cargo add` で使う
   - バイナリ（main.rs）: `cargo install` で使う
   - 両方持つことも可能

前のファイル: [chapter14_02_crates_io.md](chapter14_02_crates_io.md) - crates.ioへの公開
