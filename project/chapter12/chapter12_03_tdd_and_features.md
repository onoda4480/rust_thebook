# 第12章 まとめ (3/3): TDDと機能追加

## テスト駆動開発（TDD）

### TDD のサイクル

```
1. テストを書く（Red）
   ↓
2. テストが通る最小限のコードを書く（Green）
   ↓
3. リファクタリング（Refactor）
   ↓
1に戻る
```

---

## search 関数の実装

### 1. テストを先に書く

```rust
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
```

**テスト内容:**
- `"duct"` を検索
- `"safe, fast, productive."` のみマッチ
- `"Duct tape."` はマッチしない（大文字小文字を区別）

---

### 2. 実装

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

**ポイント:**
- `contents.lines()`: 各行を取得
- `line.contains(query)`: 文字列を含むか確認
- `results.push(line)`: マッチした行を追加

---

### ライフタイム注釈

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
//            ^^^^                     ^^                 ^^
//            ライフタイム注釈
```

**意味:**
- 戻り値の参照は `contents` と同じライフタイム
- `contents` が有効な間、戻り値も有効

---

## 大文字小文字を区別しない検索

### テスト

```rust
#[test]
fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(query, contents)
    );
}
```

**テスト内容:**
- `"rUsT"` で検索（大文字小文字混在）
- `"Rust:"` と `"Trust me."` がマッチ

---

### 実装

```rust
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```

---

### to_lowercase() の動作

```rust
let query = "rUsT".to_lowercase();  // "rust"
```

**重要:**
- 新しい `String` を作成（メモリ確保）
- 元の文字列は変更されない（不変）
- `"rUsT"` には小文字の `u`, `t` が含まれていないため、新規作成が必要

---

### なぜ & が必要？

```rust
line.to_lowercase().contains(&query)
//                           ^^^^^^
```

**理由:**
- `query` は `String` 型
- `contains()` は `&str` を期待
- `&` で参照を渡す（自動的に `&str` に変換される）

---

## 環境変数による設定

### Config に case_sensitive を追加

```rust
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,  // ← 追加
}
```

---

### env::var() で環境変数を取得

```rust
use std::env;

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        //                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        //                   環境変数が「ない」場合に true

        Ok(Config { query, filename, case_sensitive })
    }
}
```

**動作:**
- `CASE_INSENSITIVE` が**設定されていない** → `is_err()` = `true` → 大文字小文字を区別
- `CASE_INSENSITIVE` が**設定されている** → `is_err()` = `false` → 大文字小文字を区別しない

---

## if 式で検索関数を切り替え

### run 関数の実装

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
```

**if 式:**
- Rust の `if` は値を返す（式）
- `config.case_sensitive` が `true` → `search()` の結果
- `config.case_sensitive` が `false` → `search_case_insensitive()` の結果

---

## 環境変数の設定方法

### macOS/Linux (zsh/bash)

```bash
# 方法1: セッション全体で有効
export CASE_INSENSITIVE=1
cargo run -- to poem.txt

# 方法2: 1コマンドだけ有効
CASE_INSENSITIVE=1 cargo run -- to poem.txt
```

---

### Windows PowerShell

```powershell
$env:CASE_INSENSITIVE=1
cargo run -- to poem.txt
```

---

### 環境変数の確認と削除

```bash
# 確認
echo $CASE_INSENSITIVE

# 削除
unset CASE_INSENSITIVE
```

---

## 使用例

### 1. 大文字小文字を区別する（デフォルト）

```bash
cargo run -- to poem.txt
```

**結果:**
```
Are you nobody, too?
```

`"to"` のみマッチ（`"To"` はマッチしない）

---

### 2. 大文字小文字を区別しない

```bash
CASE_INSENSITIVE=1 cargo run -- to poem.txt
```

**結果:**
```
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

`"to"`, `"To"` 両方マッチ

---

## テストの実行

```bash
cargo test
```

**出力:**
```
running 2 tests
test test::case_insensitive ... ok
test test::case_sensitive ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 完成したファイル構造

### src/lib.rs

```rust
use std::env;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // 引数解析と環境変数チェック
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ファイル読み込みと検索実行
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // 大文字小文字を区別する検索
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // 大文字小文字を区別しない検索
}

#[cfg(test)]
mod test {
    // テスト
}
```

---

### src/main.rs

```rust
extern crate minigrep;
use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
```

---

## Python との比較

### Python

```python
import sys
import os

def search(query, contents):
    results = []
    for line in contents.splitlines():
        if query in line:
            results.append(line)
    return results

def search_case_insensitive(query, contents):
    query = query.lower()
    results = []
    for line in contents.splitlines():
        if query in line.lower():
            results.append(line)
    return results

def main():
    query = sys.argv[1]
    filename = sys.argv[2]

    with open(filename) as f:
        contents = f.read()

    case_sensitive = 'CASE_INSENSITIVE' not in os.environ

    if case_sensitive:
        results = search(query, contents)
    else:
        results = search_case_insensitive(query, contents)

    for line in results:
        print(line)
```

---

### Rust

```rust
// より型安全で、コンパイル時にエラーチェック
// ライフタイムで参照の安全性を保証
// Result型で明示的なエラー処理
```

---

## まとめ

```
TDD（テスト駆動開発）:
✅ テストを先に書く
✅ 最小限の実装
✅ リファクタリング

検索機能:
✅ search(): 大文字小文字を区別
✅ search_case_insensitive(): 区別しない
✅ to_lowercase() で小文字化
✅ ライフタイム注釈で安全な参照

環境変数:
✅ env::var() で環境変数取得
✅ is_err() で存在チェック
✅ if 式で動作を切り替え

実行方法:
✅ デフォルト: cargo run -- query file.txt
✅ 区別しない: CASE_INSENSITIVE=1 cargo run -- query file.txt
✅ テスト: cargo test
```

---

## 学んだこと

```
✅ TDDの実践
✅ ライフタイム注釈の使い方
✅ to_lowercase() による文字列変換
✅ 環境変数の活用
✅ if 式による条件分岐
✅ 標準出力とエラー出力の分離
✅ lib.rs + main.rs パターン
✅ Result型による適切なエラー処理
```
