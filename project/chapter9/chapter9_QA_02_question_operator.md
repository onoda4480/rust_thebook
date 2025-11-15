# Chapter 9 Q&A Part 2: ? 演算子

## Q1: ? 演算子は何をしている？

**質問:** `?` 演算子は内部で何をしているの？

**回答:** **`match` による早期リターンの省略形です。**

---

### ? を使わない場合

```rust
fn read_file() -> Result<String, io::Error> {
    let mut f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(e) => return Err(e),  // エラーなら関数を抜ける
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),  // エラーなら関数を抜ける
    }
}
```

---

### ? を使った場合

```rust
fn read_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;  // エラーなら return Err(e)
    let mut s = String::new();
    f.read_to_string(&mut s)?;  // エラーなら return Err(e)
    Ok(s)
}
```

---

### 展開

```rust
// これ
let f = File::open("hello.txt")?;

// は、これと同じ
let f = match File::open("hello.txt") {
    Ok(file) => file,
    Err(e) => return Err(e),
};
```

---

## Q2: なぜ main で ? が使えないの？

**質問:** `main()` 関数で `?` が使えないのはなぜ？

**回答:** **`main()` の戻り値が `()` だからです。**

---

### 問題のコード

```rust
fn main() {
    let f = File::open("hello.txt")?;  // ❌ エラー
    //                             ^
    //                             return Err(e) を実行
    //                             でも main の戻り値は ()
}
```

**エラー:**
```
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option`
```

---

### 理由

```rust
// ? は内部で return Err(e) をする
let f = File::open("hello.txt")?;

// 展開すると
let f = match File::open("hello.txt") {
    Ok(file) => file,
    Err(e) => return Err(e),  // ← Err(e) を返す
    //        ^^^^^^^^^^^^^^
    //        main の戻り値は () なので返せない！
};
```

---

### 解決策1: match を使う

```rust
fn main() {
    let f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(e) => {
            println!("エラー: {:?}", e);
            return;
        }
    };
}
```

---

### 解決策2: main を Result を返すように変更

```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    //       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //       Result を返すように変更

    let f = File::open("hello.txt")?;  // ✅ OK

    Ok(())  // 成功時は Ok(()) を返す
}
```

---

## Q3: ? は Result と Option の両方で使える？

**質問:** `?` は `Result` だけ？`Option` でも使える？

**回答:** **両方で使えます！**

---

### Result での使用

```rust
fn read_file() -> Result<String, io::Error> {
    let contents = fs::read_to_string("file.txt")?;
    //                                           ^
    //                                           Result<String, io::Error>
    Ok(contents)
}
```

---

### Option での使用

```rust
fn get_first_char(s: &str) -> Option<char> {
    let first = s.chars().nth(0)?;
    //                           ^
    //                           Option<char>
    //                           None なら return None
    Some(first.to_uppercase().next()?)
}

// 使用
match get_first_char("hello") {
    Some(c) => println!("{}", c),  // 'H'
    None => println!("空文字列"),
}
```

---

### 混在はできない

```rust
fn mixed() -> Result<String, io::Error> {
    let s = "hello";
    let first = s.chars().nth(0)?;  // ❌ エラー
    //                           ^
    //                           Option<char> を返す
    //                           でも関数は Result を返す

    Ok(String::from("ok"))
}
```

**エラー:**
```
error[E0277]: the `?` operator can only be applied to values that implement `Try`
```

---

### 解決策：変換する

```rust
fn mixed() -> Result<String, String> {
    let s = "hello";
    let first = s.chars().nth(0)
        .ok_or(String::from("空文字列"))?;
    //  ^^^^^^^^^^^^^^^^^^^^^^^^^^
    //  Option → Result に変換

    Ok(first.to_string())
}
```

---

## Q4: ? の後ろに . でメソッドチェーンできる？

**質問:** `?` の後ろにメソッドを繋げられる？

**回答:** **できます！**

---

### 基本的な使い方

```rust
fn read_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    //                    ^                         ^
    //                    ?                         ?
    Ok(s)
}
```

---

### メソッドチェーン

```rust
fn read_and_uppercase() -> Result<String, io::Error> {
    let contents = fs::read_to_string("hello.txt")?
        .to_uppercase();
    //  ^^^^^^^^^^
    //  ? の後にメソッドチェーン

    Ok(contents)
}
```

---

### 複雑な例

```rust
fn process_file() -> Result<Vec<String>, io::Error> {
    let lines = fs::read_to_string("data.txt")?
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect();

    Ok(lines)
}
```

---

## Q5: ? とエラーの型変換

**質問:** エラーの型が違う時、`?` はどうやって変換しているの？

**回答:** **`From` トレイトによる自動変換です。**

---

### 例

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    //             ^^^^^^^^^^^^^^^^^^^^^^^^^
    //             Result<File, io::Error>

    let mut s = String::new();
    file.read_to_string(&mut s)?;
    //  ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //  Result<usize, io::Error>
    //  usize を無視して ? で伝播

    Ok(s)
}
```

---

### 型変換の例

```rust
use std::num::ParseIntError;
use std::fs;
use std::io;

// io::Error と ParseIntError の両方を扱う
fn read_number_from_file() -> Result<i32, Box<dyn std::error::Error>> {
    //                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //                                どんなエラーでもOK

    let contents = fs::read_to_string("number.txt")?;
    //             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //             Result<String, io::Error>
    //             → Box<dyn Error> に自動変換

    let num = contents.trim().parse()?;
    //        ^^^^^^^^^^^^^^^^^^^^^^^^
    //        Result<i32, ParseIntError>
    //        → Box<dyn Error> に自動変換

    Ok(num)
}
```

---

## まとめ

### ? 演算子の動作

```
result?

展開:
match result {
    Ok(value) => value,
    Err(e) => return Err(e),
}

Result<T, E> の場合:
- Ok(value) → value を取り出す
- Err(e) → return Err(e) で早期リターン

Option<T> の場合:
- Some(value) → value を取り出す
- None → return None で早期リターン
```

---

### 使える条件

```
関数の戻り値が:
✅ Result<T, E>
✅ Option<T>

関数の戻り値が:
❌ () や i32 などの通常の型

例外:
✅ main() を Result を返すように変更すれば使える
```

---

### メソッドチェーン

```rust
// ✅ OK: ? の後にメソッド
let result = File::open("file.txt")?.read_to_string(&mut s)?;

// ✅ OK: ? の後にドット
let uppercase = fs::read_to_string("file.txt")?.to_uppercase();
```

---

### 型変換

```
Box<dyn Error> を使えば:
✅ 異なるエラー型を統一できる
✅ From トレイトで自動変換
```

---

### ベストプラクティス

```
✅ ? を積極的に使う
✅ main() を Result<(), Box<dyn Error>> にする
✅ メソッドチェーンで簡潔に
❌ unwrap() や expect() を多用しない
```
