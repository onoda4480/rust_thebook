# 第12章 Q&A (1/3): 基礎編

## Q1: args[0] には何が入っている？なぜプログラム名が含まれる？

### A: バイナリのパスが入っている。C言語の慣習から。

---

### args[0] の内容

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
```

**実行:**
```bash
cargo run -- searchstring example.txt
```

**出力:**
```
["target/debug/minigrep", "searchstring", "example.txt"]
  ↑                       ↑                ↑
  args[0]                 args[1]          args[2]
```

---

### なぜプログラム名が含まれる？

**歴史的経緯:**
- C言語の `main(int argc, char *argv[])` の慣習
- `argv[0]` にプログラム名が入る
- Rustもこの慣習に従っている

---

### いつ args[0] を使う？

#### 1. エラーメッセージ

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <query> <filename>", args[0]);
        //                   ^^^^^^^
        //                   プログラム名を表示
        process::exit(1);
    }
}
```

**出力:**
```
Usage: target/debug/minigrep <query> <filename>
```

---

#### 2. プログラムの動作を変える

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    // プログラム名によって動作を変える
    if args[0].contains("grep") {
        // grep モード
    } else if args[0].contains("cat") {
        // cat モード
    }
}
```

**Unix の例:**
- `gzip` と `gunzip` は同じプログラム
- 呼ばれた名前によって動作を変える

---

### minigrep で args[0] を使わない理由

```rust
let query = &args[1];      // 検索文字列
let filename = &args[2];   // ファイル名
```

**理由:**
- プログラム名は必要ない
- 実際の引数だけが重要
- `args[1]` と `args[2]` だけを使う

---

## Q2: Primitive Obsession（組み込み型強迫観念）って何？

### A: 組み込み型だけを使い、カスタム型を作らないアンチパターン

---

### 悪い例（Primitive Obsession）

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];      // String
    let filename = &args[2];   // String

    run(query, filename);
}

fn run(query: &str, filename: &str) {
    // query と filename の関係が明確でない
}
```

**問題点:**
- `query` と `filename` がバラバラ
- 関係性が不明確
- 引数の順序を間違えやすい

---

### 良い例（Config 構造体）

```rust
pub struct Config {
    pub query: String,
    pub filename: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config {
        query: args[1].clone(),
        filename: args[2].clone(),
    };

    run(config);
}

fn run(config: Config) {
    // query と filename が Config として関連付けられている
}
```

**メリット:**
- 関連するデータをまとめる
- 意味が明確
- 型安全

---

### 実例1: 住所

**悪い例:**
```rust
fn print_address(zip: &str, prefecture: &str, city: &str, street: &str) {
    println!("{} {} {} {}", zip, prefecture, city, street);
}
```

**良い例:**
```rust
struct Address {
    zip: String,
    prefecture: String,
    city: String,
    street: String,
}

fn print_address(address: &Address) {
    println!("{} {} {} {}",
        address.zip, address.prefecture, address.city, address.street);
}
```

---

### 実例2: 座標

**悪い例:**
```rust
fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    // x1 と y1、x2 と y2 の関係が不明確
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}
```

**良い例:**
```rust
struct Point {
    x: f64,
    y: f64,
}

fn distance(p1: &Point, p2: &Point) -> f64 {
    // Point として関連付けられている
    ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt()
}
```

---

### minigrep の Config

```rust
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
```

**メリット:**
- 設定をまとめて管理
- 新しいフィールドを追加しやすい
- 関数の引数が増えない

---

### Python との比較

**Python（悪い例）:**
```python
def run(query, filename, case_sensitive):
    # 引数が増えると管理が大変
    pass
```

**Python（良い例）:**
```python
class Config:
    def __init__(self, query, filename, case_sensitive):
        self.query = query
        self.filename = filename
        self.case_sensitive = case_sensitive

def run(config):
    pass
```

---

## Q3: clone() を使ってもいい？所有権が難しいんだけど...

### A: はい、初めは clone() を使ってOK！最適化は後で。

---

### The Book の方針

> 「多くのRustaceanは、まず最初に動作するコードを書き、それから最適化する」

**つまり:**
- 困ったら `clone()` を使う
- まず動くコードを書く
- 後で最適化する

---

### clone() を使う例

```rust
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();    // ← clone() を使う
        let filename = args[2].clone(); // ← clone() を使う

        Ok(Config { query, filename })
    }
}
```

**理由:**
- `args[1]` は `&String`（借用）
- `Config` は所有する `String` が必要
- `clone()` で新しい `String` を作る

---

### clone() なしで書くと？

```rust
// ❌ コンパイルエラー
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let query = &args[1];     // &String
        let filename = &args[2];  // &String

        Ok(Config { query, filename })  // ❌ Config は String が必要
    }
}
```

**エラー:**
```
error: expected `String`, found `&String`
```

---

### clone() のコスト

```rust
let s = String::from("hello");
let s2 = s.clone();  // メモリをコピー
```

**コスト:**
- メモリのコピーが発生
- 小さい文字列なら問題ない
- 大きいデータやループ内では避ける

---

### clone() を使うべき場合

#### ✅ OK: 初学者

```rust
let query = args[1].clone();
```

**理由:** 所有権の学習中は簡単な方法を使う

---

#### ✅ OK: プロトタイピング

```rust
let data = original.clone();
// まず動かしてから最適化
```

---

#### ✅ OK: 1回だけの操作

```rust
fn main() {
    let config = Config {
        query: args[1].clone(),  // 1回だけなので問題ない
        filename: args[2].clone(),
    };
}
```

---

### clone() を避けるべき場合

#### ❌ NG: ループ内

```rust
for i in 0..1000000 {
    let s = large_string.clone();  // 毎回コピー！遅い
    process(s);
}
```

**代わりに:**
```rust
for i in 0..1000000 {
    process(&large_string);  // 借用を渡す
}
```

---

#### ❌ NG: 大きいデータ

```rust
let huge_vec = vec![0; 1_000_000];
let copy = huge_vec.clone();  // 100万要素をコピー！
```

**代わりに:**
```rust
let reference = &huge_vec;  // 借用を使う
```

---

### 最適化は第13章で

```rust
// 第12章（今）
let query = args[1].clone();  // OK

// 第13章（後で）
// クロージャやイテレータを使ってより効率的に書く
```

---

### まとめ

```
clone() を使ってもいい？
→ はい！困ったら clone() を使ってOK

いつ使う？
✅ 初学者の学習中
✅ プロトタイピング
✅ 1回だけの操作
✅ 小さいデータ

いつ避ける？
❌ ループ内
❌ 大きいデータ
❌ パフォーマンスが重要な場所

The Book の方針:
「まず動くコードを書く、後で最適化する」
```

---

### Python との比較

**Python:**
```python
# Python は自動的に参照カウント
s1 = "hello"
s2 = s1  # コピーではなく参照

# 明示的にコピーする場合
import copy
s2 = copy.copy(s1)
```

**Rust:**
```rust
// Rust は明示的
let s1 = String::from("hello");
let s2 = s1;         // ムーブ（所有権移動）
let s3 = s1.clone(); // コピー
```
