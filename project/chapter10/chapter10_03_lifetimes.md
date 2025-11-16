# 第10章 まとめ (3/3): ライフタイム

## ライフタイムとは

**定義:** 参照が有効である期間をコンパイラに伝える注釈

**重要:** ライフタイム注釈は生きる時間を**変えるものではない**。コンパイラに**説明するだけ**。

---

## なぜライフタイムが必要？

### ダングリングポインタを防ぐ

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x;  // ❌ エラー！x はスコープを抜ける
    }
    println!("{}", r);  // r は無効な参照を指している
}
```

**エラー:**
```
error[E0597]: `x` does not live long enough
```

**Rust の安全性:** コンパイル時に検出

---

## ライフタイム注釈の構文

### 基本構文

```rust
&i32        // 普通の参照
&'a i32     // 明示的なライフタイム 'a 付きの参照
&'a mut i32 // 明示的なライフタイム 'a 付きの可変参照
```

---

### 関数シグネチャでのライフタイム

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//        ^^^^   ^^^^^^^     ^^^^^^^      ^^^^^^^
//        'a を定義   'a タグ    'a タグ      'a タグ
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

**意味:** 「戻り値のライフタイムは、x と y のうち短い方と同じ」

---

## ライフタイムの仕組み

### 例: 正常なケース

```rust
fn main() {
    let string1 = String::from("long string");
    let string2 = String::from("short");

    let result = longest(string1.as_str(), string2.as_str());
    println!("{}", result);  // ✅ OK
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

**ライフタイム:**
```
string1 ━━━━━━━━━━━━━━━━━━━━━━━━┓
                                  ┃
string2 ━━━━━━━━━━━━━━━━━━━━━━━━┫ ← 'a（両方有効）
                                  ┃
result                            ┃
                                  ┃
println!("{}", result)  ← OK     ┃
                                  ┃
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

---

### 例: エラーケース

```rust
fn main() {
    let string1 = String::from("long string");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }  // ← string2 が破棄される
    println!("{}", result);  // ❌ エラー
}
```

**ライフタイム:**
```
string1 ━━━━━━━━━━━━━━━━━━━━━━━━┓
                                  ┃
    {                             ┃
    string2 ━━━━━━━┓              ┃
                    ┃              ┃
    result = ...    ┃              ┃
    }  ← string2 破棄┛              ┃
                                  ┃
    println!(result)  ← ❌        ┃
                                  ┃
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

**エラー:**
```
error[E0597]: `string2` does not live long enough
```

---

## 構造体でのライフタイム

### 参照を持つ構造体

```rust
struct ImportantExcerpt<'a> {
//                     ^^^^
//                     ライフタイム 'a を定義
    part: &'a str,
//        ^^^^^^^
//        'a タグ付き参照
}
```

**意味:** 「ImportantExcerpt のインスタンスは、part が参照しているデータより長生きできない」

---

### 使用例

```rust
fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("Excerpt: {}", i.part);  // ✅ OK
}
```

**ライフタイム:**
```
novel ━━━━━━━━━━━━━━━━━━━━━━━━━━┓
    ↑                             ┃
    │                             ┃
first_sentence ━━━━━━━━━━━━━━━━━┫ ← 'a
    ↑                             ┃
    │                             ┃
i (ImportantExcerpt) ━━━━━━━━━━━┫
                                  ┃
println!(...) ← OK               ┃
                                  ┃
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

---

## ライフタイム省略規則

**重要:** 多くの場合、ライフタイム注釈は自動的に推論される

### 規則1: 各引数に独自のライフタイム

```rust
fn foo(x: &str, y: &str)
// ↓ 自動的に
fn foo<'a, 'b>(x: &'a str, y: &'b str)
```

---

### 規則2: 引数が1つなら、戻り値も同じライフタイム

```rust
fn first_word(s: &str) -> &str
// ↓ 自動的に
fn first_word<'a>(s: &'a str) -> &'a str
```

**だから書かなくていい！**

---

### 規則3: メソッドで &self があれば、戻り値は &self と同じ

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
    //                          ^^^^^               ^^^^^      ^^^^
    //                          'a                  'b         'a（自動）
        println!("Attention: {}", announcement);
        self.part
    }
}
```

**だから書かなくていい！**

---

## ライフタイム注釈が必要な場合

### 複数の参照を受け取って参照を返す

```rust
// ❌ エラー
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

// ✅ OK
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

---

### 構造体が参照を持つ

```rust
// ❌ エラー
struct ImportantExcerpt {
    part: &str,
}

// ✅ OK
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

---

## ライフタイムとジェネリック

### 両方を使う

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
//                              ^^^^^^
//                              'a: ライフタイムパラメータ
//                              T: 型パラメータ
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() { x } else { y }
}
```

**ポイント:** ライフタイムもジェネリックの一種なので、同じ `< >` に書く

---

## 静的ライフタイム 'static

### 'static とは

```rust
let s: &'static str = "I have a static lifetime.";
//     ^^^^^^^^^^^^
//     プログラム全体で有効
```

**特徴:**
- 文字列リテラルは自動的に `'static`
- プログラムの実行中ずっと有効

---

### 使用例

```rust
fn returns_static() -> &'static str {
    "This string lives for the entire program"
}
```

---

## ローカル変数への参照は返せない

### ❌ ダメな例

```rust
fn dangle<'a>() -> &'a str {
    let s = String::from("hello");
    &s  // ❌ エラー！s はスコープを抜ける
}
```

**エラー:**
```
error[E0515]: cannot return reference to local variable `s`
```

---

### ✅ 正しい方法

```rust
// 方法1: 所有権を返す
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // ✅ OK
}

// 方法2: 静的ライフタイム
fn static_str() -> &'static str {
    "hello"  // ✅ OK
}
```

---

## ライフタイムの考え方

### ライフタイム = タグ・目印

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
//        ^^^^   ^^          ^^           ^^
//        'a というタグを定義
//              ↓           ↓            ↓
//           タグ'a      タグ'a       タグ'a
```

**意味:** 「全部同じ 'a タグ = 同じライフタイムグループ」

---

### コンパイラへの説明書

```
ライフタイム注釈 = コンパイラへの説明書

「この戻り値は、これらの引数と同じ期間有効です」
と説明する

実際の寿命は変わらない！
```

---

## Python との比較

### Python（ガベージコレクション）

```python
def longest(x, y):
    if len(x) > len(y):
        return x
    else:
        return y

# ライフタイムの概念なし
# ガベージコレクションが自動管理
```

---

### Rust（ライフタイム）

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ライフタイムをコンパイル時にチェック
// ダングリングポインタを防ぐ
```

---

## まとめ

### ライフタイムとは

```
✅ 参照が有効である期間をコンパイラに伝える注釈
✅ 生きる時間を変えるものではない
✅ タグ・目印のようなもの
✅ コンパイラへの説明書
```

---

### ライフタイムの目的

```
✅ ダングリングポインタを防ぐ
✅ メモリ安全性を保証
✅ コンパイル時に検証
✅ 実行時のガベージコレクション不要
```

---

### いつ必要？

```
✅ 複数の参照を受け取って参照を返す関数
✅ 構造体が参照を持つ場合
✅ コンパイラが推論できない場合

❌ 引数が1つの関数（自動推論）
❌ メソッドで &self から返す場合（自動推論）
❌ 所有権を返す場合（参照ではない）
```

---

### 基本構文

```rust
// 関数
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { ... }

// 構造体
struct Name<'a> {
    field: &'a str,
}

// メソッド
impl<'a> Name<'a> {
    fn method(&self) -> &str { ... }  // 自動推論
}

// ジェネリックと併用
fn function<'a, T>(x: &'a str, value: T) -> &'a str
where
    T: Display,
{
    ...
}
```

---

### ライフタイム省略規則

```
1. 各引数に独自のライフタイム
2. 引数が1つ → 戻り値も同じライフタイム
3. メソッド(&self) → 戻り値は &self のライフタイム

→ 多くの場合、書かなくていい
→ コンパイラが教えてくれる
```

---

### 重要な概念

```
✅ ライフタイム = ジェネリックの一種
✅ 'a は任意の名前（慣習的に 'a, 'b, ... を使う）
✅ 'static = プログラム全体で有効
✅ ローカル変数への参照は返せない
✅ 所有権を返すか、静的ライフタイムを使う
```
