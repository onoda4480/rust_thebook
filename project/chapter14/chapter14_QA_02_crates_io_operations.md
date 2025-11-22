# 第14章 Q&A (2/3): crates.io操作とセキュリティ

## Q1: API Tokenの Scopes って何？

### 質問の背景

crates.ioでAPIトークンを発行する時に「Scopes」を選ぶ画面が出る。これは何？

### 答え

**Scopesは、APIトークンに与える権限の範囲のことです。**

セキュリティの観点から、**必要最小限のスコープだけを選ぶ**のがベストプラクティスです。

### 主なスコープ

| スコープ | 説明 | 使用場面 |
|---------|------|---------|
| `publish-new` | 新しいクレートを公開 | 初めてクレートを公開する時 |
| `publish-update` | 既存のクレートを更新 | クレートの新バージョンを公開する時 |
| `yank` | クレートのバージョンをyank | 問題のあるバージョンを取り下げる時 |

### 推奨される使い方

#### 初めて公開する場合

```
Scopes:
  ☑ publish-new
  ☑ publish-update
```

#### 既存クレートを更新するだけの場合

```
Scopes:
  ☐ publish-new
  ☑ publish-update
```

### なぜスコープが重要か

**セキュリティ:**
- トークンが漏洩しても、選んだスコープの範囲でしか悪用されない
- 例: `publish-update` のみなら、新しいクレートは作られない

**ベストプラクティス:**
- 定期的にトークンを更新する
- 用途ごとに別のトークンを発行する
- 不要になったトークンは削除する

### Pythonとの比較

**Python (PyPI):**
- APIトークンにスコープの概念がある
- プロジェクトごとにトークンを発行できる
- 全体的な権限管理

**Rust (crates.io):**
- スコープで細かく権限を制御
- セキュリティ重視の設計

### まとめ

- Scopes = トークンの権限範囲
- 必要最小限のスコープを選ぶ
- `publish-new` + `publish-update` が一般的

---

## Q2: `cargo yank` って削除じゃなくて取り下げ？

### 質問の背景

`cargo yank` は「バージョンを下げる」ものだと思ったが、実際には違う？

### 答え

**`cargo yank` は削除でもバージョンダウンでもなく、「取り下げ」です。**

### yank の正確な動作

```bash
cargo yank --vers 1.0.1
```

**効果:**
- ❌ 新しいプロジェクトでは使えなくなる
- ❌ `cargo update` で選択されなくなる
- ✅ 既存のプロジェクト（Cargo.lockがある）は影響を受けない
- ✅ データは残る（完全削除されない）
- ✅ yankを解除できる（`--undo`）

### 削除 vs yank の違い

| | yank | 削除 |
|---|------|------|
| 新規利用 | ❌ 不可 | - |
| 既存プロジェクト | ✅ 影響なし | ❌ エラー |
| データ | ✅ 残る | ❌ 削除 |
| 解除 | ✅ 可能 | ❌ 不可 |
| crates.io | ❌ **削除不可** | - |

### 具体例

#### プロジェクトA（既存）

```toml
# Cargo.toml
[dependencies]
my-crate = "1.0.1"
```

```toml
# Cargo.lock（すでにロックされている）
[[package]]
name = "my-crate"
version = "1.0.1"  # ← yankされても使える
```

**結果:** ✅ 問題なく動作し続ける

#### プロジェクトB（新規）

```toml
# Cargo.toml
[dependencies]
my-crate = "1.0"  # 1.0.xの最新を使う
```

```bash
$ cargo build
error: no matching package named `my-crate` found
```

**理由:** 1.0.1がyankされているので、他のバージョンを探すが見つからない

### yankを解除する

```bash
# yankを解除
cargo yank --vers 1.0.1 --undo
```

**結果:** 再び新規プロジェクトで使えるようになる

### 使用場面

- 重大なバグが見つかった
- セキュリティ脆弱性がある
- 間違ったバージョンを公開してしまった

### Pythonとの比較

**Python (PyPI):**
- パッケージの削除が可能（条件付き）
- 削除すると既存プロジェクトも影響を受ける可能性

**Rust (crates.io):**
- 削除は不可、yankのみ
- 既存プロジェクトは保護される
- より安定したエコシステム

### まとめ

- yank = 取り下げ（削除ではない）
- 新規利用不可、既存は影響なし
- crates.ioからの完全削除は不可能
- `--undo` で解除可能

---

## Q3: トークンをハードコードしたらヤバい？

### 質問の背景

もしAPIトークンやシークレット情報をコードに書いてcrates.ioに公開してしまったら、どうなる？

### 答え

**超危険です！一度公開したら永久に削除できません。**

### なぜ危険か

1. **削除不可**: crates.ioに公開したら永久に残る
2. **誰でも見られる**: すべてのコードが公開される
3. **履歴も残る**: Gitの履歴も含めて公開される
4. **自動スキャン**: ボットが常にトークンを探している

### やってはいけない例

```rust
// ❌ 絶対にやってはいけない！
const API_KEY: &str = "sk-1234567890abcdef";
const DATABASE_URL: &str = "postgres://user:password@localhost/db";
const AWS_SECRET: &str = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY";
```

### 安全な方法

#### 1. 環境変数を使う

```rust
// ✅ 安全
use std::env;

fn main() {
    let api_key = env::var("API_KEY")
        .expect("API_KEY environment variable not set");

    // api_keyを使用
}
```

```bash
# 実行時に環境変数を設定
API_KEY=sk-1234567890abcdef cargo run
```

#### 2. 設定ファイルを使う

```rust
// ✅ 安全
use std::fs;

fn main() {
    let config = fs::read_to_string(".env")
        .expect("Failed to read .env file");

    // configをパース
}
```

```.gitignore
# .gitignoreに追加
.env
config.json
secrets.toml
*.key
credentials.json
```

#### 3. Cargo.toml の exclude

```toml
[package]
exclude = [
    ".env",
    "secrets/*",
    "*.key",
]
```

### もし公開してしまったら

1. **トークンを即座に無効化**
   - crates.ioのトークン → 削除して再発行
   - AWS/GCP → キーをローテーション
   - GitHub → トークンを再発行

2. **パスワードを変更**
   - データベースのパスワード
   - APIのシークレット

3. **被害を確認**
   - ログを確認
   - 不正なアクセスがないか確認

4. **対策を実施**
   - 環境変数に移行
   - .gitignoreに追加
   - pre-commit hookで検出

### Pythonとの比較

**Python (.pypirc):**
```ini
# ~/.pypirc（ローカルにのみ保存）
[pypi]
username = __token__
password = pypi-...
```

**Rust (~/.cargo/credentials):**
```toml
# ~/.cargo/credentials（ローカルにのみ保存）
[registry]
token = "crates-io-..."
```

どちらもトークンをローカルファイルに保存し、コードには含めない。

### 予防策

#### Git の pre-commit hook

```bash
#!/bin/sh
# .git/hooks/pre-commit

if git diff --cached | grep -i "api_key\|secret\|password"; then
    echo "Warning: Possible secret detected!"
    exit 1
fi
```

#### ツールを使う

```bash
# git-secrets（AWSが提供）
brew install git-secrets
git secrets --install
git secrets --register-aws

# gitleaks
brew install gitleaks
gitleaks detect
```

### まとめ

- ❌ トークンのハードコードは絶対NG
- ✅ 環境変数を使う
- ✅ .gitignoreに追加
- ❌ 公開したら削除不可
- ✅ 予防が最重要

---

## Q4: `cargo login <token>` の警告は何？

### 質問の背景

```bash
$ cargo login sk-...
warning: `cargo login <token>` is deprecated in favor of reading `<token>` from stdin
```

この警告は何？

### 答え

**コマンドライン引数にトークンを書く方法は非推奨です。セキュリティ上の理由からです。**

### 問題点

```bash
# ❌ 非推奨
cargo login sk-1234567890abcdef
```

**危険な理由:**
- シェルの履歴にトークンが残る
- `history` コマンドで見られる
- プロセスリストで見られる可能性

### 推奨される方法

#### 方法1: プロンプトで入力

```bash
# ✅ 推奨
cargo login
# ↓ プロンプトが表示される
# please paste the token found on https://crates.io/me below
# ここでトークンを貼り付ける
```

#### 方法2: 標準入力から渡す

```bash
# ✅ 安全
echo "sk-1234567890abcdef" | cargo login
```

#### 方法3: ファイルから読み込む

```bash
# ✅ 安全
cat ~/.crates-token | cargo login
```

### シェル履歴の確認

```bash
# 履歴を確認
history | grep "cargo login"

# 履歴から削除（bash）
history -d <行番号>

# 履歴から削除（zsh）
fc -R
```

### Pythonとの比較

**Python (twine):**
```bash
# ❌ 非推奨
twine upload --password pypi-...

# ✅ 推奨（環境変数）
export TWINE_PASSWORD=pypi-...
twine upload

# ✅ 推奨（.pypirc）
twine upload  # ~/.pypircから読み込む
```

**Rust (cargo):**
```bash
# ❌ 非推奨
cargo login sk-...

# ✅ 推奨
cargo login  # プロンプトで入力
```

### まとめ

- ❌ `cargo login <token>` は非推奨
- ✅ `cargo login` でプロンプト入力
- 理由: シェル履歴に残らないようにするため
- セキュリティ重視の設計

次のQ&A: [chapter14_QA_03_cargo_ecosystem.md](chapter14_QA_03_cargo_ecosystem.md) - Cargoエコシステム
