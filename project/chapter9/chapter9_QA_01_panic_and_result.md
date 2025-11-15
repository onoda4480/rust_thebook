# Chapter 9 Q&A Part 1: panic! と Result の基本

## Q1: panic! と Result の違いは？

**質問:** `panic!` と `Result` は何が違うの？どう使い分ける？

**回答:** **回復可能かどうかの違いです。**

---

### panic!

```rust
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("0で割れません！");  // プログラム停止
    }
    a / b
}
```

**特徴:**
- プログラムが停止する
- 回復不可能
- エラーメッセージを表示

---

### Result

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("0で割れません"));  // エラーを返す
    }
    Ok(a / b)
}

// 呼び出し側でエラー処理
match divide(10, 0) {
    Ok(result) => println!("{}", result),
    Err(e) => println!("エラー: {}", e),  // 処理を続行できる
}
```

**特徴:**
- エラーを値として返す
- 呼び出し側で処理できる
- 回復可能

---

### 使い分け

| 状況 | 使うべき | 理由 |
|------|----------|------|
| テストコード | `panic!` | 失敗したら止まるべき |
| 論理的エラー | `panic!` | 起こってはいけない |
| ファイル操作 | `Result` | 失敗は予期される |
| ユーザー入力 | `Result` | 不正な入力はありうる |
| ネットワーク | `Result` | 接続失敗はありうる |

---

### Python との比較

#### Python

```python
# 例外を投げる（回復可能）
def divide(a, b):
    if b == 0:
        raise ZeroDivisionError("0で割れません")
    return a / b

try:
    result = divide(10, 0)
except ZeroDivisionError as e:
    print(f"エラー: {e}")
```

#### Rust

```rust
// Result を返す（回復可能）
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("0で割れません"));
    }
    Ok(a / b)
}

match divide(10, 0) {
    Ok(result) => println!("{}", result),
    Err(e) => println!("エラー: {}", e),
}
```

---

## Q2: unwrap と expect の違いは？

**質問:** `unwrap()` と `expect()` は何が違うの？

**回答:** **エラーメッセージが違います。**

---

### unwrap()

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
    // エラー時: デフォルトのメッセージ
}
```

**エラー時の出力:**
```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }'
```

**問題:** エラーの原因が分かりにくい

---

### expect()

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")
        .expect("hello.txt を開けませんでした");
    // エラー時: カスタムメッセージ
}
```

**エラー時の出力:**
```
thread 'main' panicked at 'hello.txt を開けませんでした: Os { code: 2, kind: NotFound, message: "No such file or directory" }'
```

**利点:** エラーの原因が明確

---

### 推奨

```rust
// ✅ 推奨
let f = File::open("config.txt")
    .expect("設定ファイルが見つかりません");

// ❌ 非推奨
let f = File::open("config.txt").unwrap();
```

**理由:**
- `expect()` はエラーの文脈が分かる
- デバッグしやすい
- コードの意図が明確

---

## Q3: バックトレースって何？

**質問:** バックトレースとは？どう使うの？

**回答:** **エラーが発生するまでの関数呼び出しの履歴です。**

---

### バックトレースの有効化

```bash
RUST_BACKTRACE=1 cargo run
```

---

### 例

```rust
fn main() {
    let v = vec![1, 2, 3];
    let element = v[99];  // panic!
}
```

**出力:**
```
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:3:19
stack backtrace:
   0: rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::panic_bounds_check
   ...
   6: panic_example::main
             at ./src/main.rs:3:19  ← ここが問題箇所
   7: core::ops::function::FnOnce::call_once
...
```

---

### 読み方

```
1. 自分のコードを探す
   → panic_example::main at ./src/main.rs:3:19

2. ファイルと行番号を確認
   → src/main.rs の 3行目、19文字目

3. コードを見る
   → let element = v[99];

4. 原因を特定
   → 範囲外アクセス

5. 修正
   → v.get(99) を使うなど
```

---

## Q4: Result<T, E> の T と E って何？

**質問:** `Result<T, E>` の `T` と `E` は何？

**回答:** **ジェネリック型パラメータです。**

---

### T = 成功時の型

```rust
fn read_number() -> Result<i32, String> {
    //                      ^^^
    //                      成功時は i32
    Ok(42)
}
```

---

### E = エラー時の型

```rust
fn read_number() -> Result<i32, String> {
    //                           ^^^^^^
    //                           エラー時は String
    Err(String::from("エラー"))
}
```

---

### 具体例

```rust
use std::fs::File;
use std::io;

fn open_file() -> Result<File, io::Error> {
    //                    ^^^^  ^^^^^^^^
    //                    T     E
    //                    成功   エラー
    File::open("hello.txt")
}

// 使用
match open_file() {
    Ok(file) => {
        // file は File 型
        println!("ファイルを開きました");
    }
    Err(error) => {
        // error は io::Error 型
        println!("エラー: {:?}", error);
    }
}
```

---

### まとめ

```
Result<T, E>

T: 成功時に返す値の型
E: エラー時に返す値の型

例:
Result<i32, String>     → 成功: i32、エラー: String
Result<File, io::Error> → 成功: File、エラー: io::Error
Result<(), String>      → 成功: ()、エラー: String
```

---

## Q5: ErrorKind って何？

**質問:** `error.kind()` の `ErrorKind` とは？

**回答:** **エラーの種類を表す列挙型です。**

---

### 定義

```rust
pub enum ErrorKind {
    NotFound,           // ファイルが見つからない
    PermissionDenied,   // 権限がない
    ConnectionRefused,  // 接続拒否
    AlreadyExists,      // すでに存在する
    WouldBlock,         // ブロックされる
    InvalidInput,       // 無効な入力
    TimedOut,           // タイムアウト
    // ... 他にもたくさん
}
```

---

### 使用例

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            // エラーの種類で分岐
            match error.kind() {
                ErrorKind::NotFound => {
                    println!("ファイルが見つかりません");
                    return;
                }
                ErrorKind::PermissionDenied => {
                    println!("権限がありません");
                    return;
                }
                other_error => {
                    panic!("その他のエラー: {:?}", other_error);
                }
            }
        }
    };
}
```

---

### Python との比較

#### Python

```python
import errno

try:
    f = open("hello.txt")
except FileNotFoundError:
    print("ファイルが見つかりません")
except PermissionError:
    print("権限がありません")
except IOError as e:
    print(f"その他のエラー: {e}")
```

#### Rust

```rust
match File::open("hello.txt") {
    Ok(file) => { },
    Err(error) => {
        match error.kind() {
            ErrorKind::NotFound => {
                println!("ファイルが見つかりません");
            }
            ErrorKind::PermissionDenied => {
                println!("権限がありません");
            }
            _ => {
                println!("その他のエラー");
            }
        }
    }
}
```

---

## まとめ

### panic! vs Result

```
panic!:
✅ 回復不可能
✅ プログラム停止
✅ テスト・プロトタイプ用

Result:
✅ 回復可能
✅ エラーを値として返す
✅ 本番コード用
```

---

### unwrap vs expect

```
unwrap():
❌ エラーメッセージが不明確
❌ 避けるべき

expect("理由"):
✅ エラーメッセージが明確
✅ 推奨
```

---

### バックトレース

```bash
RUST_BACKTRACE=1 cargo run

読み方:
1. 自分のコードを探す
2. 該当行を確認
3. 原因を特定
4. 修正
```

---

### Result<T, E>

```
T: 成功時の型
E: エラー時の型

ErrorKind: エラーの種類
→ error.kind() で取得
→ match で分岐
```
