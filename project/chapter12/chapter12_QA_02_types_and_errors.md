# 第12章 Q&A (2/3): 型とエラー処理編

## Q1: &'static って何？&' と static はセット？

### A: &' はライフタイム注釈で、'static は「プログラム全体で有効」という特殊なライフタイム

---

### ライフタイムの復習（第10章）

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//         ^^^^     ^^          ^^           ^^
//         ライフタイム注釈
    if x.len() > y.len() { x } else { y }
}
```

**意味:**
- `'a` は任意のライフタイム名
- 引数と戻り値の参照が同じライフタイムを持つ

---

### &'static の分解

```rust
&'static str
↑ ↑      ↑
│ │      └─ 型（文字列スライス）
│ └─ ライフタイム（static）
└─ 参照
```

**構造:**
- `&'`: 参照 + ライフタイム注釈の構文
- `static`: 特殊なライフタイム名
- `str`: 型

---

### 'static ライフタイムとは

```rust
let s: &'static str = "hello";
```

**意味:**
- プログラムの**全期間**有効
- プログラムが終了するまで存在する
- 通常は文字列リテラルやグローバル変数

---

### 文字列リテラルは &'static str

```rust
fn main() {
    let s = "Hello, world!";  // 型: &'static str
    //      ^^^^^^^^^^^^^^^
    //      プログラム全体で有効
}
```

**理由:**
- 文字列リテラルはバイナリに埋め込まれる
- プログラム全体で有効

---

### minigrep での使用例

```rust
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        //                                         ^^^^^^^^^^^^^
        //                                         エラーメッセージ
        if args.len() < 3 {
            return Err("not enough arguments");
            //         ^^^^^^^^^^^^^^^^^^^^^^^^
            //         &'static str（文字列リテラル）
        }

        Ok(Config { ... })
    }
}
```

**なぜ &'static str？**
- エラーメッセージは文字列リテラル
- プログラム全体で有効
- 関数の戻り値として安全に返せる

---

### 他のライフタイムとの比較

#### 通常のライフタイム（'a）

```rust
fn get_first<'a>(list: &'a [String]) -> &'a str {
    &list[0]
}
```

**意味:**
- `list` が有効な間だけ有効
- 関数が終わったら参照は使えなくなるかも

---

#### 'static ライフタイム

```rust
fn get_greeting() -> &'static str {
    "Hello"  // プログラム全体で有効
}
```

**意味:**
- いつまでも有効
- 関数が終わっても使える

---

### &' と static は別物

```
✅ 正しい理解:
&'static
↑ ↑
│ └─ static はライフタイム名の1つ
└─ &' は参照+ライフタイム注釈の構文

他のライフタイムの例:
&'a
&'b
&'lifetime
```

**つまり:**
- `&'` と `static` はセットではない
- `'static` は数あるライフタイムの1つ
- `'a`, `'b`, `'static` などが使える

---

### 'static を使う場面

#### 1. 文字列リテラル

```rust
let s: &'static str = "hello";
```

---

#### 2. エラーメッセージ

```rust
fn validate(x: i32) -> Result<(), &'static str> {
    if x < 0 {
        return Err("negative value");  // &'static str
    }
    Ok(())
}
```

---

#### 3. const 定数

```rust
const MAX_SIZE: &'static str = "maximum size exceeded";
```

---

### Python との比較

**Python:**
```python
# Python には明示的なライフタイムの概念はない
def get_message():
    return "Hello"  # 文字列リテラル
```

**Rust:**
```rust
fn get_message() -> &'static str {
    "Hello"  // 明示的にライフタイムを指定
}
```

---

## Q2: unwrap_or_else() って何？クロージャとの組み合わせは？

### A: Result が Err の時にクロージャを実行するメソッド

---

### unwrap_or_else の基本

```rust
let result: Result<i32, &str> = Err("error");

let value = result.unwrap_or_else(|err| {
    println!("Error: {}", err);
    0  // デフォルト値
});
```

**動作:**
- `Ok(value)` → `value` を返す
- `Err(err)` → クロージャを実行して、その結果を返す

---

### クロージャの構文

```rust
|err| {
 ^^^^
 パラメータ
    println!("Error: {}", err);
    process::exit(1);
}
```

**構造:**
- `|param|`: パラメータ
- `{ ... }`: 実行する処理

**似ている構文:**
```javascript
// JavaScript
(err) => {
    console.log(err);
}
```

```python
# Python
lambda err: print(err)
```

---

### minigrep での使用例

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        //                                          ^^^^^
        //                                          Err の値が渡される
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
}
```

**動作:**
1. `Config::new(&args)` が `Result` を返す
2. `Ok(config)` なら `config` を取り出す
3. `Err(err)` ならクロージャを実行

---

### 具体例

#### ケース1: 成功（Ok）

```rust
let args = vec![
    String::from("minigrep"),
    String::from("query"),
    String::from("file.txt"),
];

let config = Config::new(&args).unwrap_or_else(|err| {
    eprintln!("{}", err);
    process::exit(1);
});
// config には Config 構造体が入る
// クロージャは実行されない
```

---

#### ケース2: 失敗（Err）

```rust
let args = vec![
    String::from("minigrep"),
    // 引数が足りない
];

let config = Config::new(&args).unwrap_or_else(|err| {
    //                                          ^^^^
    //                                          "not enough arguments"
    eprintln!("Problem parsing arguments: {}", err);
    // 出力: Problem parsing arguments: not enough arguments
    process::exit(1);
    // プログラムが終了
});
```

---

### 他の unwrap 系メソッドとの比較

#### unwrap()

```rust
let config = Config::new(&args).unwrap();
// Ok → config を返す
// Err → パニック（エラーメッセージなし）
```

---

#### expect()

```rust
let config = Config::new(&args).expect("Failed to parse arguments");
// Ok → config を返す
// Err → パニック（カスタムメッセージ付き）
```

---

#### unwrap_or()

```rust
let value = result.unwrap_or(default_value);
// Ok → value を返す
// Err → default_value を返す（関数は実行しない）
```

---

#### unwrap_or_else()

```rust
let value = result.unwrap_or_else(|err| {
    // カスタム処理
    compute_default_value()
});
// Ok → value を返す
// Err → クロージャを実行して結果を返す
```

---

### process::exit(1) とは

```rust
use std::process;

process::exit(1);
//             ^
//             終了コード
```

**終了コード:**
- `0`: 成功
- `1` (または0以外): エラー

**効果:**
- プログラムを即座に終了
- OS に終了コードを返す

---

### match で書いた場合の比較

#### unwrap_or_else 版

```rust
let config = Config::new(&args).unwrap_or_else(|err| {
    eprintln!("Problem: {}", err);
    process::exit(1);
});
```

---

#### match 版（同じ意味）

```rust
let config = match Config::new(&args) {
    Ok(config) => config,
    Err(err) => {
        eprintln!("Problem: {}", err);
        process::exit(1);
    }
};
```

---

### まとめ

```
unwrap_or_else():
✅ Err の時にクロージャを実行
✅ カスタムエラー処理ができる
✅ process::exit() でプログラムを終了

クロージャ:
|param| { body }
✅ param: パラメータ
✅ body: 実行する処理

比較:
unwrap()         → パニック
expect()         → パニック（メッセージ付き）
unwrap_or()      → デフォルト値
unwrap_or_else() → クロージャ実行
```

---

## Q3: Box&lt;dyn Error&gt; って何？複雑すぎない？

### A: 「任意のエラー型」を返せる便利な型。3つの要素からなる。

---

### Box&lt;dyn Error&gt; の分解

```rust
Box<dyn Error>
^^^
│  ヒープに確保するスマートポインタ

Box<dyn Error>
    ^^^
    │  動的ディスパッチ（実行時に型を決定）

Box<dyn Error>
        ^^^^^
        │  標準エラートレイト
```

---

### 1. Box&lt;T&gt; とは

```rust
let x = Box::new(5);  // i32 をヒープに確保
```

**役割:**
- データをヒープに確保
- スマートポインタ
- サイズが不明な型を扱える

**詳細は第15章で学ぶ**

---

### 2. dyn とは

```rust
dyn Error
^^^
動的ディスパッチ
```

**意味:**
- 実行時に型を決定
- 異なる型を統一的に扱える

**静的 vs 動的:**
```rust
// 静的（コンパイル時に型が決まる）
fn process(x: i32) { }

// 動的（実行時に型が決まる）
fn process(x: &dyn Display) { }
```

**詳細は第17章で学ぶ**

---

### 3. Error トレイトとは

```rust
use std::error::Error;
```

**役割:**
- 標準エラートレイト
- すべてのエラー型が実装すべきトレイト

**実装している型:**
- `std::io::Error`
- `std::fmt::Error`
- カスタムエラー型

---

### なぜ Box&lt;dyn Error&gt; が必要？

#### 問題: 異なるエラー型を返したい

```rust
pub fn run(config: Config) -> Result<(), ???> {
    //                                   ^^^
    //                                   何を返す？

    // File::open() は io::Error を返す
    let mut f = File::open(config.filename)?;

    // read_to_string() も io::Error を返す
    f.read_to_string(&mut contents)?;

    Ok(())
}
```

---

#### 解決: Box&lt;dyn Error&gt;

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //                                   ^^^^^^^^^^^^^^^^
    //                                   任意のエラー型

    let mut f = File::open(config.filename)?;  // io::Error
    f.read_to_string(&mut contents)?;          // io::Error

    Ok(())
}
```

**メリット:**
- 異なるエラー型を統一的に扱える
- `?` 演算子が使える

---

### 具体例

```rust
use std::fs::File;
use std::io::Read;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // File::open は io::Error を返す可能性がある
    let mut f = File::open(config.filename)?;
    //                                      ^
    //                                      io::Error を Box<dyn Error> に変換

    let mut contents = String::new();
    // read_to_string も io::Error を返す可能性がある
    f.read_to_string(&mut contents)?;
    //                              ^
    //                              io::Error を Box<dyn Error> に変換

    Ok(())
}
```

---

### 他のアプローチとの比較

#### アプローチ1: 具体的なエラー型

```rust
pub fn run(config: Config) -> Result<(), std::io::Error> {
    // io::Error しか返せない
    let mut f = File::open(config.filename)?;
    Ok(())
}
```

**制限:**
- `io::Error` 以外のエラーを返せない

---

#### アプローチ2: カスタムエラー型

```rust
enum MyError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    // ...
}

pub fn run(config: Config) -> Result<(), MyError> {
    // すべてのエラーを MyError に変換
}
```

**複雑:**
- エラー型ごとに変換が必要

---

#### アプローチ3: Box&lt;dyn Error&gt;（推奨）

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 任意のエラー型を返せる
    let mut f = File::open(config.filename)?;
    Ok(())
}
```

**シンプル:**
- 変換不要
- 柔軟

---

### まとめ

```
Box<dyn Error>:
✅ 任意のエラー型を返せる
✅ 異なるエラー型を統一的に扱える
✅ ? 演算子が使える

3つの要素:
1. Box<T>: ヒープに確保（第15章で詳しく）
2. dyn: 動的ディスパッチ（第17章で詳しく）
3. Error: 標準エラートレイト

使い方:
Result<(), Box<dyn Error>>
   ↑    ↑   ↑
   OK   Err  任意のエラー型
```

---

### 今は詳細を理解しなくてOK

```
第12章（今）:
「Box<dyn Error> は便利な型」
という理解でOK

第15章（後で）:
Box<T> の詳細を学ぶ

第17章（後で）:
dyn（動的ディスパッチ）の詳細を学ぶ
```
