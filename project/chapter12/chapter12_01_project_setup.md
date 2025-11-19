# 第12章 まとめ (1/3): プロジェクトのセットアップ

## 概要

この章では、コマンドラインツール「minigrep」を構築します。

**機能:**
- ファイル内で文字列を検索する（`grep` コマンドのミニ版）
- 大文字小文字を区別する/しない検索
- 環境変数による設定

---

## プロジェクト構造

```
minigrep/
├── Cargo.toml
├── src/
│   ├── main.rs      # エントリーポイント
│   └── lib.rs       # ロジック
└── poem.txt         # テスト用ファイル
```

**パターン:**
- `main.rs`: 薄い（引数解析、エラー処理のみ）
- `lib.rs`: ロジック（検索機能、テスト可能）

---

## コマンドライン引数の受け取り

### env::args()

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
```

**ポイント:**
- `args[0]`: プログラム名（バイナリのパス）
- `args[1]`: 第1引数（検索文字列）
- `args[2]`: 第2引数（ファイル名）

---

### 引数の取り出し

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];      // 検索文字列
    let filename = &args[2];   // ファイル名

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
```

**問題点:**
- 引数の数をチェックしていない
- エラー処理がない
→ 後で改善する

---

## ファイルの読み込み

### std::fs を使う

```rust
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
```

**fs::read_to_string:**
- ファイル全体を `String` として読み込む
- `Result<String, Error>` を返す
- エラーは `expect()` でパニック（暫定的）

---

### ファイル例

```
# poem.txt
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.
```

**実行:**
```bash
cargo run -- the poem.txt
```

**出力:**
```
Searching for the
In file poem.txt
With text:
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.
```

---

## Python との比較

### Python

```python
import sys

# コマンドライン引数
args = sys.argv
# args[0]: プログラム名
# args[1], args[2], ...: 引数

# ファイル読み込み
with open(filename, 'r') as f:
    contents = f.read()
```

---

### Rust

```rust
use std::env;
use std::fs;

// コマンドライン引数
let args: Vec<String> = env::args().collect();
// args[0]: プログラム名
// args[1], args[2], ...: 引数

// ファイル読み込み
let contents = fs::read_to_string(filename)
    .expect("Error reading file");
```

---

## 現時点の main.rs（改善前）

```rust
use std::env;
use std::fs;

fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();

    // 引数を取り出す
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    // ファイルを読み込む
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
```

**問題点:**
1. すべてのロジックが `main` にある
2. エラー処理が不十分（`expect` だけ）
3. テストできない
4. 関心の分離ができていない

→ 次のファイルでリファクタリング

---

## まとめ

```
基本的なセットアップ:
✅ env::args() で引数を取得
✅ args[0] はプログラム名
✅ args[1], args[2] が実際の引数
✅ fs::read_to_string() でファイル読み込み
✅ Result型を expect() で処理（暫定的）

次のステップ:
→ エラー処理の改善
→ ロジックの分離
→ Config 構造体の導入
→ lib.rs へのロジック移動
```
