# Chapter 4-2: 参照と借用（References and Borrowing）

## 参照とは？

**「借りる」だけで所有権を取らない**

---

## 基本的な参照

### 問題：所有権を渡すと使えなくなる

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(s1);  // s1がムーブ

    // println!("{}", s1);  // ❌ エラー！s1は使えない
}

fn calculate_length(s: String) -> usize {
    s.len()
}
```

### 解決策：参照を使う

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // &を付けて参照

    println!("{}", s1);  // ✅ OK！s1はまだ使える
}

fn calculate_length(s: &String) -> usize {
    s.len()
}  // sは参照なのでdropしない
```

---

## 参照の仕組み

### メモリ構造

```rust
let s1 = String::from("hello");
let s2 = &s1;  // s1への参照
```

**メモリ:**
```
スタック（s1）      ヒープ
┌────────┐      ┌─┬─┬─┬─┬─┐
│ ptr    │─────→│h│e│l│l│o│
│ len: 5 │      └─┴─┴─┴─┴─┘
│ cap: 5 │           ↑
└────────┘           │
                     │
スタック（s2）         │
┌────────┐          │
│ ptr    │──────────┘
└────────┘
```

**s2は「s1を指すポインタ」**

---

## 借用のルール

### 参照 = 借用

**図書館の例:**
- 所有者（図書館）：本を所有、管理、処分の責任
- 借用者（あなた）：本を借りて読むだけ、返却したら終わり

```rust
let s = String::from("本");  // 図書館（所有者）

read(&s);  // 本を借りる

println!("{}", s);  // 返却後も図書館にある

fn read(book: &String) {
    println!("{}", book);
}  // 返却（本は処分しない）
```

---

## 不変参照と可変参照

### 不変参照（&T）

**読み取り専用**

```rust
fn main() {
    let s = String::from("hello");

    read_only(&s);  // 不変参照

    println!("{}", s);  // OK
}

fn read_only(s: &String) {
    println!("{}", s);  // ✅ 読める
    // s.push_str("!!");  // ❌ 変更できない
}
```

### 可変参照（&mut T）

**読み書き可能**

```rust
fn main() {
    let mut s = String::from("hello");  // mut必須

    change(&mut s);  // 可変参照

    println!("{}", s);  // "hello, world"
}

fn change(s: &mut String) {
    s.push_str(", world");  // ✅ 変更できる
}
```

**必要な3つの条件:**
1. 元の変数が `mut`
2. 渡す時に `&mut`
3. 関数の引数が `&mut`

---

## 借用のルール

### ルール1：不変参照は複数OK

```rust
let s = String::from("hello");

let r1 = &s;  // ✅ OK
let r2 = &s;  // ✅ OK
let r3 = &s;  // ✅ OK

println!("{}, {}, {}", r1, r2, r3);  // 全部OK
```

**理由:** 読み取りだけなら安全

---

### ルール2：可変参照は1つだけ

```rust
let mut s = String::from("hello");

let r1 = &mut s;  // ✅ OK
let r2 = &mut s;  // ❌ エラー！

println!("{}, {}", r1, r2);
```

**エラー:**
```
error[E0499]: cannot borrow `s` as mutable more than once at a time
```

**理由:** データ競合を防ぐ

---

### ルール3：不変と可変は混在不可

```rust
let mut s = String::from("hello");

let r1 = &s;      // ✅ 不変参照
let r2 = &s;      // ✅ 不変参照
let r3 = &mut s;  // ❌ エラー！

println!("{}, {}, {}", r1, r2, r3);
```

**エラー:**
```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
```

**理由:** 読み取り中に変更されると危険

---

## スコープと借用

### スコープを分ける

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;
    println!("{}", r1);
}  // r1のスコープ終了

let r2 = &mut s;  // ✅ OK！r1は終わった
println!("{}", r2);
```

### Non-Lexical Lifetimes（NLL）

**最後の使用で終わる（Rust 2018+）**

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
println!("{}, {}", r1, r2);  // ← r1, r2の最後の使用

let r3 = &mut s;  // ✅ OK！r1, r2は終わった
println!("{}", r3);
```

---

## 借用ルールまとめ

| 状況 | OK? | 理由 |
|---|---|---|
| 不変参照のみ（複数） | ✅ | 読み取りだけなら安全 |
| 可変参照のみ（1つ） | ✅ | 排他的アクセス |
| 可変参照（複数・同時） | ❌ | データ競合のリスク |
| 不変参照 + 可変参照（同時） | ❌ | 読み取り中の変更は危険 |

---

## ダングリングポインタ

### 問題：解放済みメモリへの参照

```rust
fn dangle() -> &String {  // ❌ コンパイルエラー
    let s = String::from("hello");
    &s  // sへの参照を返そうとする
}  // sがdropされる → 参照が無効に！
```

**エラー:**
```
error[E0106]: missing lifetime specifier
this function's return type contains a borrowed value,
but there is no value for it to be borrowed from
```

### 解決策：所有権を返す

```rust
fn no_dangle() -> String {  // ✅ OK
    let s = String::from("hello");
    s  // 所有権を返す
}  // sはムーブ済みなのでdropされない
```

---

## 参照のメリット

### 所有権を渡す（面倒）

```rust
fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);  // タプルで返す

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)  // 所有権を返す（面倒）
}
```

### 参照を使う（シンプル）

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // 参照を渡す

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()  // シンプル！
}
```

---

## まとめ

### 参照の重要ポイント

| 概念 | 説明 |
|---|---|
| **参照** | 所有権を取らない借用 |
| **不変参照（&T）** | 読み取り専用、複数OK |
| **可変参照（&mut T）** | 読み書き可能、1つだけ |
| **dropしない** | 参照は所有者ではない |
| **ダングリング防止** | コンパイラが検出 |

### 借用の3つのルール

```
1. 不変参照は何個でもOK
2. 可変参照は1個だけ
3. 不変と可変は同時に持てない
```

### 安全性の保証

- ✅ ダングリングポインタなし
- ✅ データ競合なし
- ✅ 読み取り中の変更なし
- ✅ 全てコンパイル時チェック
