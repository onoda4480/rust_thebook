# 第12章 まとめ (2/3): リファクタリングとエラー処理

## リファクタリングの目標

**問題点:**
- すべてのロジックが `main` にある
- エラー処理が不十分
- テストできない

**解決策:**
1. Config 構造体を導入
2. エラー処理を改善
3. ロジックを `lib.rs` に分離

---

## Config 構造体の導入

### Primitive Obsession を避ける

**Before（Primitive Obsession）:**
```rust
fn main() {
    let query = &args[1];      // String
    let filename = &args[2];   // String
    // 関連するデータが分散している
}
```

**After（Config 構造体）:**
```rust
pub struct Config {
    pub query: String,
    pub filename: String,
}
```

**メリット:**
- 関連するデータをまとめる
- 意味が明確になる
- 型安全

---

## Config::new() の実装

### エラー処理を含む

```rust
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
```

**ポイント:**
- `Result<Config, &'static str>` を返す
- 引数の数をチェック
- `clone()` で所有権を取得（初学者向けアプローチ）

---

### clone() の使用について

```rust
let query = args[1].clone();
let filename = args[2].clone();
```

**なぜ clone()？**
- 所有権の管理を簡単にするため
- 初学者向けのアプローチ
- このケースではパフォーマンス問題にならない

**The Book の方針:**
> 「まずは動くコードを書き、後で最適化する」

---

## unwrap_or_else() によるエラー処理

### main.rs での使用

```rust
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
}
```

**動作:**
- `Ok(config)` → `config` を取り出す
- `Err(err)` → クロージャを実行

**unwrap_or_else の利点:**
- カスタムエラー処理ができる
- プログラムを適切に終了できる

---

### クロージャの詳細

```rust
|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
}
```

**構造:**
- `|err|`: パラメータ（エラー値）
- `{ ... }`: 実行する処理

**process::exit(1):**
- プログラムを終了
- `1`: エラーコード（0以外 = エラー）

---

## ロジックの分離（lib.rs）

### run 関数

```rust
// lib.rs
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}
```

**ポイント:**
- `Box<dyn Error>`: 任意のエラー型を返せる
- `?` 演算子: エラーを呼び出し元に伝播
- `Ok(())`: 正常終了

---

### Box&lt;dyn Error&gt; とは

```rust
Result<(), Box<dyn Error>>
```

**3つの要素:**
1. **Box&lt;T&gt;**: ヒープに確保するスマートポインタ
2. **dyn**: 動的ディスパッチ（実行時に型を決定）
3. **Error**: 標準エラートレイト

**意味:**
- 「任意のエラー型」を返せる
- 異なるエラー型を統一的に扱える

---

## main.rs と lib.rs の分離

### main.rs（薄い）

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

**役割:**
- 引数の解析
- エラー処理
- プログラムの終了

---

### lib.rs（ロジック）

```rust
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // 引数の検証
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ファイル読み込みと処理
}
```

**役割:**
- ビジネスロジック
- テスト可能
- 再利用可能

---

## 標準エラー出力

### println! vs eprintln!

```rust
// 標準出力（stdout）
println!("Normal output");

// 標準エラー出力（stderr）
eprintln!("Error message");
```

**使い分け:**
- `println!`: 通常の出力
- `eprintln!`: エラーメッセージ

---

### リダイレクトとの組み合わせ

```bash
# 標準出力のみファイルに保存
cargo run -- to poem.txt > output.txt

# エラーメッセージはターミナルに表示される
```

**エラー時:**
```bash
cargo run > output.txt
# Problem parsing arguments: not enough arguments
# ↑ ターミナルに表示される（output.txt には書かれない）
```

---

## エラー処理のベストプラクティス

### Result の伝播

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;  // エラーを伝播
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;          // エラーを伝播
    Ok(())
}
```

**? 演算子:**
- エラー時: `Err` を返す
- 成功時: 値を取り出して続行

---

### main でのエラー処理

```rust
if let Err(e) = minigrep::run(config) {
    eprintln!("Application error: {}", e);
    process::exit(1);
}
```

**パターン:**
1. `run()` を実行
2. エラーなら `eprintln!` でメッセージ出力
3. `process::exit(1)` で終了

---

## Python との比較

### Python

```python
import sys

class Config:
    def __init__(self, query, filename):
        self.query = query
        self.filename = filename

def main():
    if len(sys.argv) < 3:
        print("not enough arguments", file=sys.stderr)
        sys.exit(1)

    config = Config(sys.argv[1], sys.argv[2])

    try:
        with open(config.filename) as f:
            contents = f.read()
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)
```

---

### Rust

```rust
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Config { ... })
    }
}

fn main() {
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
```

---

## まとめ

```
リファクタリング:
✅ Config 構造体で関連データをまとめる
✅ Result型で適切なエラー処理
✅ clone() で初学者にも分かりやすく
✅ unwrap_or_else() でカスタムエラー処理

ロジックの分離:
✅ main.rs: エントリーポイント（薄い）
✅ lib.rs: ビジネスロジック（テスト可能）
✅ Box<dyn Error> で柔軟なエラー型

標準エラー出力:
✅ eprintln! でエラーメッセージ
✅ println! と分けることでリダイレクト可能
✅ process::exit(1) で適切に終了
```
