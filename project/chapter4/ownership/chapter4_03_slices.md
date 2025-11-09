# Chapter 4-3: スライス（Slices）

## スライスとは？

**コレクションの一部への参照**

---

## 文字列スライス（&str）

### 基本的な使い方

```rust
let s = String::from("hello world");

let hello = &s[0..5];   // "hello"への参照
let world = &s[6..11];  // "world"への参照
```

**視覚化:**
```
s: "hello world"
    01234567891011

&s[0..5]:  "hello"
           ↑___↑
           0~4

&s[6..11]: "world"
                 ↑____↑
                 6~10
```

---

## スライスの内部構造

### メモリ構造

```rust
let s = String::from("hello world");
let hello = &s[0..5];
```

**メモリ:**
```
スタック（s）        ヒープ
┌──────────┐      ┌─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┐
│ ptr      │─────→│h│e│l│l│o│ │w│o│r│l│d│
│ len: 11  │      └─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┘
│ capacity │       0 1 2 3 4 5 6 7 8 9 10
└──────────┘

スタック（hello）
┌──────────┐
│ ptr: 0   │ ← ヒープの0番目を指す
│ len: 5   │ ← 長さ5
└──────────┘
```

**スライスは:**
- ポインタ（開始位置）
- 長さ
の2つを持つ

---

## 範囲の書き方

### 基本構文

```rust
let s = String::from("hello world");

&s[start..end]  // startからend-1まで
```

**重要:** `end` は**含まれない**

```
インデックス: 0 1 2 3 4 5
文字:        h e l l o

&s[0..5] =   h e l l o
             ↑_______↑
             0~4（5は含まない）
```

### 省略記法

```rust
let s = String::from("hello world");

// 最初から
&s[0..5]
&s[..5]    // 同じ意味

// 途中から最後まで
&s[6..11]
&s[6..]    // 同じ意味

// 全体
&s[0..11]
&s[..]     // 同じ意味
```

---

## 文字列スライスの型

### `&str` とは？

```rust
let s = String::from("hello");
let slice: &str = &s[0..2];  // 型: &str
```

**`&str` = 文字列スライス型**

---

## 文字列リテラルは `&str`

### リテラルの正体

```rust
let s = "hello";  // 型: &str
```

**メモリ:**
```
プログラムのバイナリ（静的領域）
┌─┬─┬─┬─┬─┐
│h│e│l│l│o│
└─┴─┴─┴─┴─┘
 ↑
 sはここへの参照（&str）
```

**特徴:**
- プログラムに埋め込まれている
- 所有権を持たない（参照）
- 最初から `&str` 型

---

## 関数の引数に `&str` を使う

### Before: `&String`（制限あり）

```rust
fn first_word(s: &String) -> &str {
    // ...
}

// Stringの参照しか渡せない
let my_string = String::from("hello");
first_word(&my_string);  // ✅ OK

let my_literal = "hello";
first_word(my_literal);  // ❌ エラー
```

### After: `&str`（柔軟）

```rust
fn first_word(s: &str) -> &str {
    // ...
}

// Stringの参照も渡せる（自動変換）
let my_string = String::from("hello");
first_word(&my_string);  // ✅ OK

// 文字列リテラルも渡せる
let my_literal = "hello";
first_word(my_literal);  // ✅ OK

// スライスも渡せる
first_word(&my_string[..]);  // ✅ OK
```

**理由:** `&String` → `&str` の自動変換（Deref coercion）

---

## String vs &str

### 型の違い

| 型 | 所有権 | 可変長 | 保存場所 | 使い分け |
|---|---|---|---|---|
| `String` | 所有 | ✅ | ヒープ | データを所有したい時 |
| `&str` | 参照 | ❌ | どこでも | データを読むだけの時 |

### 変換

```rust
// String → &str（自動）
let s = String::from("hello");
let slice: &str = &s;  // 自動変換

// &str → String（明示的）
let literal = "hello";
let s = literal.to_string();  // または
let s = String::from(literal);
```

---

## スライスの実践例

### first_word関数

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];  // スペースまで
        }
    }

    &s[..]  // スペースがなければ全体
}

fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);

    println!("{}", word);  // "hello"
}
```

---

## スライスと所有権の問題解決

### Before: インデックスを返す（危険）

```rust
fn first_word(s: &String) -> usize {
    // 最初の単語の終わり位置を返す
    5
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);  // word = 5

    s.clear();  // sを空にする

    // wordは5だが、sは空！
    // &s[0..word] なんて存在しない！
}
```

**問題:** `word` と `s` が独立、同期がずれる

### After: スライスを返す（安全）

```rust
fn first_word(s: &str) -> &str {
    &s[0..5]  // スライス（参照）を返す
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);  // wordはsへの参照

    s.clear();  // ❌ コンパイルエラー！
                // wordがsを借用中

    println!("{}", word);
}
```

**エラー:**
```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
```

**解決:** 借用ルールで自動的に防げる

---

## 配列スライス（&[T]）

### 文字列以外もスライスできる

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];  // 型: &[i32]

println!("{:?}", slice);  // [2, 3]
```

**メモリ:**
```
配列 a:
┌───┬───┬───┬───┬───┐
│ 1 │ 2 │ 3 │ 4 │ 5 │
└───┴───┴───┴───┴───┘
  0   1   2   3   4
      ↑_______↑
      sliceが指す範囲

スライス:
┌──────────┐
│ ptr: &a[1]│ ← 1番目を指す
│ len: 2    │ ← 長さ2
└──────────┘
```

### 型の形式

```rust
// 文字列スライス
let s: &str = &string[..];

// 配列スライス
let a: &[i32] = &array[..];
let b: &[f64] = &floats[..];
let c: &[bool] = &flags[..];
```

**パターン:** `&[要素の型]`

---

## Vec とスライス

### Vec のスライス

```rust
let v = vec![1, 2, 3, 4, 5];
let slice = &v[1..3];  // 型: &[i32]

println!("{:?}", slice);  // [2, 3]
```

**重要:** Vec のスライスも配列のスライスも同じ `&[T]` 型

---

## Python との比較

### Python の list（コピーを作る）

```python
a = [1, 2, 3, 4, 5]
slice = a[1:3]  # [2, 3] の新しいリスト

slice[0] = 999
print(a)      # [1, 2, 3, 4, 5]（変わらない）
print(slice)  # [999, 3]
```

### Rust のスライス（参照）

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];  // 参照（ポインタ）

// slice[0] = 999;  // ❌ エラー（不変参照）
println!("{:?}", a);      // [1, 2, 3, 4, 5]
println!("{:?}", slice);  // [2, 3]
```

**違い:**
- Python: コピーを作る（メモリ確保）
- Rust: 参照を作る（ポインタのみ）

---

## まとめ

### スライスの重要ポイント

| 概念 | 説明 |
|---|---|---|
| **スライス** | コレクションの一部への参照 |
| **&str** | 文字列スライス型 |
| **&[T]** | 配列スライス型 |
| **内部構造** | ポインタ + 長さ |
| **範囲** | `[start..end]`（endは含まない） |

### 文字列型の使い分け

```
String:  所有型、可変長、ヒープ
&str:    参照型、不変、どこでも
```

### スライスのメリット

1. **安全:** 借用ルールで保護
2. **高速:** コピー不要
3. **柔軟:** 部分参照が簡単

### Python との違い

```
Python: list[1:3] → コピー
Rust:   &vec[1..3] → 参照
```

---

## ベストプラクティス

### 関数の引数

```rust
// ❌ 良くない
fn process(s: &String) { }

// ✅ 良い
fn process(s: &str) { }
```

**理由:** `&str` の方が柔軟（String、リテラル、スライス全て受け入れる）

### スライスの活用

```rust
// インデックスではなくスライスを返す
fn first_word(s: &str) -> &str {
    // ...
    &s[0..i]  // スライス
}
```

**理由:** 借用チェッカーが安全性を保証
