# 第14章まとめ (2/3): crates.ioへの公開

## 1. crates.io とは

**Rustの公式パッケージレジストリ** - すべてのRustクレートが公開される場所

### 特徴

- ✅ 完全に無料
- ✅ オープンソースのみ
- ✅ ライブラリもバイナリも公開可能
- ❌ プライベートクレートは公開不可
- ❌ 一度公開したら削除不可（yankは可能）

### Pythonとの比較

| | Rust | Python |
|---|------|--------|
| レジストリ | crates.io | PyPI |
| 公開コマンド | `cargo publish` | `twine upload` |
| インストール | `cargo add` | `pip install` |
| 削除 | 不可（yankのみ） | 可能 |

## 2. クレートの公開手順

### ステップ1: アカウント作成とログイン

```bash
# 1. crates.ioでアカウント作成（GitHubアカウントでログイン）

# 2. APIトークンを取得
#    https://crates.io/settings/tokens で New Token

# 3. ログイン（非推奨な方法）
cargo login <token>

# 3. ログイン（推奨される方法）
cargo login
# ↑ トークンをプロンプトで入力
```

**重要:** トークンはシェル履歴に残らないように注意！

### ステップ2: Cargo.toml の設定

```toml
[package]
name = "my-awesome-crate"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
description = "短い説明文（必須）"
license = "MIT OR Apache-2.0"  # ライセンス（必須）
repository = "https://github.com/user/repo"  # 推奨
documentation = "https://docs.rs/my-awesome-crate"  # 自動生成される
homepage = "https://example.com"  # オプション
keywords = ["cli", "tool"]  # 最大5個
categories = ["command-line-utilities"]  # https://crates.io/categories

[dependencies]
```

**必須フィールド:**
- `name` - クレート名（一意、早い者勝ち）
- `version` - バージョン（セマンティックバージョニング）
- `description` - 説明文
- `license` - ライセンス

**推奨されるライセンス:**
- `MIT OR Apache-2.0` - Rustエコシステムの標準

### ステップ3: 公開

```bash
# ビルドとテストの確認
cargo build --release
cargo test

# 公開前の確認（パッケージング）
cargo package

# 公開
cargo publish
```

### 公開時のチェック項目

- ✅ すべてのテストが通る
- ✅ ドキュメントが書かれている
- ✅ Cargo.tomlが正しく設定されている
- ✅ シークレット情報が含まれていない
- ✅ クレート名が適切

## 3. バージョン管理とセマンティックバージョニング

### セマンティックバージョニング (SemVer)

```
MAJOR.MINOR.PATCH
```

**ルール:**
- `MAJOR` - 互換性のない変更
- `MINOR` - 後方互換性のある機能追加
- `PATCH` - 後方互換性のあるバグ修正

**例:**
```
0.1.0  → 初回リリース
0.1.1  → バグ修正
0.2.0  → 新機能追加（互換性あり）
1.0.0  → 安定版リリース
2.0.0  → 互換性のない変更
```

### バージョン更新

```toml
# Cargo.toml
version = "0.2.0"  # バージョンを上げる
```

```bash
cargo publish  # 新しいバージョンを公開
```

**重要:**
- 同じバージョンは上書きできない
- 必ず新しいバージョン番号が必要

## 4. cargo yank - バージョンの取り下げ

### yank とは

**特定のバージョンを新規利用不可にする**（削除ではない）

```bash
# バージョンを取り下げる
cargo yank --vers 1.0.1

# 取り下げを解除する
cargo yank --vers 1.0.1 --undo
```

### yank の効果

**影響あり:**
- ❌ 新しいプロジェクトでは使えなくなる
- ❌ `cargo update` で選択されなくなる

**影響なし:**
- ✅ 既存のプロジェクト（Cargo.lockがある）は影響を受けない
- ✅ データは残る（docs.rsなども残る）
- ✅ 完全削除はできない

### 使用場面

- 重大なバグが見つかった時
- セキュリティ脆弱性がある時
- 間違ったバージョンを公開してしまった時

### 削除 vs yank

| | yank | 削除 |
|---|------|------|
| 新規利用 | ❌ 不可 | - |
| 既存プロジェクト | ✅ 影響なし | - |
| データ | ✅ 残る | ❌ 削除 |
| crates.io | ❌ **削除不可** | - |

**重要:** crates.ioに一度公開したクレートは**永久に削除できません**

## 5. cargo install - CLIツールのインストール

### 基本的な使い方

```bash
# ツールをインストール
cargo install ripgrep

# インストール先
~/.cargo/bin/rg  # 実行可能ファイル
```

### バイナリクレートのみ

**インストールできるもの:**
- ✅ バイナリクレート（src/main.rs がある）
- ❌ ライブラリクレート（src/lib.rs のみ）

### 人気のツール

```bash
# 高速なgrep
cargo install ripgrep

# ファイル変更の監視
cargo install cargo-watch

# 依存関係の管理
cargo install cargo-edit

# マクロ展開の表示
cargo install cargo-expand
```

### Pythonとの比較

| Rust | Python | 用途 |
|------|--------|------|
| `cargo install` | `pipx install` | CLIツールをグローバルにインストール |
| `Cargo.toml [dependencies]` | `poetry add` / `pip install` | プロジェクトの依存関係 |

## セキュリティの注意点

### シークレット情報を公開しない

```rust
// ❌ 絶対にやってはいけない！
const API_KEY: &str = "sk-1234567890abcdef";
const DATABASE_URL: &str = "postgres://user:password@localhost/db";
```

### 安全な方法

```rust
// ✅ 環境変数を使う
use std::env;

fn main() {
    let api_key = env::var("API_KEY").expect("API_KEY not set");
}
```

### .gitignore に追加

```
.env
config.json
secrets.toml
*.key
```

### もし公開してしまったら

1. **トークンを即座に無効化**（再発行）
2. パスワードを変更
3. 被害を確認
4. **削除できない**ので予防が最重要

## まとめ

1. **crates.io**: Rustの公式パッケージレジストリ、完全に公開
2. **公開手順**: Cargo.toml設定 → `cargo publish`
3. **バージョニング**: SemVer（MAJOR.MINOR.PATCH）
4. **cargo yank**: 削除ではなく取り下げ、既存プロジェクトは影響なし
5. **cargo install**: CLIツールをグローバルにインストール
6. **セキュリティ**: シークレット情報を絶対に公開しない

次のファイル: [chapter14_03_workspaces.md](chapter14_03_workspaces.md) - ワークスペース
