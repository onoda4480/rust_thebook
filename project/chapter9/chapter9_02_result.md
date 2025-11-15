# Chapter 9-2: Result と回復可能なエラー

## Result 型とは？

**成功またはエラーを表す列挙型**

```rust
enum Result<T, E> {
    Ok(T),   // 成功時の値
    Err(E),  // エラー時の値
}
```

---

## Result の基本

### 成功の場合

```rust
let result: Result<i32, String> = Ok(42);

match result {
    Ok(value) => println!("成功: {}", value),  // 42
    Err(e) => println!("エラー: {}", e),
}
```

---

### エラーの場合

```rust
let result: Result<i32, String> = Err(String::from("失敗"));

match result {
    Ok(value) => println!("成功: {}", value),
    Err(e) => println!("エラー: {}", e),  // 失敗
}
```

---

## ファイル操作での使用

### File::open の戻り値

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    //      ^^^^^^^^^^^^^^^^^^^^^^^^
    //      Result<File, io::Error> を返す
}
```

---

### match でエラー処理

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("ファイルを開けません: {:?}", error);
        }
    };
}
```

---

## エラーの種類による分岐

### ErrorKind を使う

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("ファイルが見つかりません。作成します。");
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("作成失敗: {:?}", e),
                }
            }
            other_error => {
                panic!("その他のエラー: {:?}", other_error);
            }
        },
    };
}
```

---

## unwrap と expect

### unwrap()

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
    // エラー時は panic!
}
```

**動作:**
- `Ok(value)` → `value` を返す
- `Err(e)` → `panic!`

---

### expect()

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")
        .expect("hello.txt を開けませんでした");
    // エラー時はカスタムメッセージで panic!
}
```

**動作:**
- `Ok(value)` → `value` を返す
- `Err(e)` → カスタムメッセージで `panic!`

---

## エラーの伝播

### match を使った伝播

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),  // エラーを返す
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),      // 成功時は文字列を返す
        Err(e) => Err(e),    // エラーを返す
    }
}
```

---

### ? 演算子を使った伝播

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;  // エラーなら早期リターン
    let mut s = String::new();
    f.read_to_string(&mut s)?;  // エラーなら早期リターン
    Ok(s)
}
```

**? 演算子:**
- `Ok(value)` → `value` を取り出す
- `Err(e)` → `return Err(e)` を実行

---

### さらに短縮

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

---

### 最短バージョン

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

---

## ? 演算子の制約

### 戻り値が Result の関数でのみ使用可能

```rust
// ✅ OK
fn read_file() -> Result<String, io::Error> {
    let contents = fs::read_to_string("file.txt")?;
    Ok(contents)
}

// ❌ エラー
fn main() {
    let contents = fs::read_to_string("file.txt")?;
    // main の戻り値が () なので ? は使えない
}
```

---

### main で ? を使う方法

```rust
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("file.txt")?;
    println!("{}", contents);
    Ok(())
}
```

---

## Result のメソッド

### is_ok() / is_err()

```rust
let result: Result<i32, &str> = Ok(42);

if result.is_ok() {
    println!("成功");
}

if result.is_err() {
    println!("エラー");
}
```

---

### unwrap_or()

```rust
let result: Result<i32, &str> = Err("エラー");

let value = result.unwrap_or(0);  // エラー時はデフォルト値
println!("{}", value);  // 0
```

---

### unwrap_or_else()

```rust
let result: Result<i32, &str> = Err("エラー");

let value = result.unwrap_or_else(|e| {
    println!("エラー: {}", e);
    0  // デフォルト値
});
```

---

### map()

```rust
let result: Result<i32, &str> = Ok(42);

let doubled = result.map(|x| x * 2);
// Ok(84)
```

---

### and_then()

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("0で割れません"))
    } else {
        Ok(a / b)
    }
}

let result = Ok(10)
    .and_then(|x| divide(x, 2))
    .and_then(|x| divide(x, 0));
// Err("0で割れません")
```

---

## Option との関係

### Option 型

```rust
enum Option<T> {
    Some(T),  // 値がある
    None,     // 値がない
}
```

---

### Option でも ? が使える

```rust
fn get_first_char(s: &str) -> Option<char> {
    let first = s.chars().nth(0)?;  // None なら早期リターン
    Some(first.to_uppercase().next()?)
}
```

---

## Python との比較

### Python

```python
# 例外を使う
try:
    with open("hello.txt") as f:
        contents = f.read()
        return contents
except FileNotFoundError as e:
    print(f"ファイルが見つかりません: {e}")
    return None
except IOError as e:
    print(f"IOエラー: {e}")
    return None
```

---

### Rust

```rust
// Result を使う
fn read_file() -> Result<String, io::Error> {
    let contents = fs::read_to_string("hello.txt")?;
    Ok(contents)
}

// 呼び出し側
match read_file() {
    Ok(contents) => println!("{}", contents),
    Err(e) => println!("エラー: {:?}", e),
}
```

---

## まとめ

### Result 型

```
enum Result<T, E> {
    Ok(T),   // 成功
    Err(E),  // エラー
}

成功時: Ok(value)
エラー時: Err(error)
```

---

### エラー処理の方法

```rust
// ① match で処理
match result {
    Ok(value) => { },
    Err(e) => { },
}

// ② unwrap (panic! する)
let value = result.unwrap();

// ③ expect (カスタムメッセージで panic!)
let value = result.expect("エラーメッセージ");

// ④ ? 演算子 (エラーを返す)
let value = result?;

// ⑤ unwrap_or (デフォルト値)
let value = result.unwrap_or(default);
```

---

### ? 演算子

```rust
// Before
let value = match result {
    Ok(v) => v,
    Err(e) => return Err(e),
};

// After
let value = result?;

条件:
✅ 関数の戻り値が Result<T, E>
✅ または Option<T>
```

---

### main で ? を使う

```rust
// 通常の main
fn main() {
    // ? は使えない
}

// Result を返す main
fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("file.txt")?;
    Ok(())
}
```

---

### ベストプラクティス

```
✅ expect() を使う (unwrap() より)
✅ ? 演算子を活用
✅ エラーを適切に伝播
✅ match でエラーの種類を分岐
❌ unwrap() を本番コードで使わない
```
