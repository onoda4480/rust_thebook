# Chapter 8-2: 文字列 (String)

## Rustの文字列

Rust には2種類の文字列型があります。

| 型 | 説明 | 格納場所 |
|---|---|---|
| **`String`** | 所有権を持つ、可変、成長可能 | ヒープ |
| **`&str`** | 文字列スライス、不変 | スタックまたはバイナリ |

---

## Python との対応

| Rust | Python |
|------|--------|
| `String` | `str`（可変的な使い方） |
| `&str` | `str`（リテラル） |

---

## String の作成

### 方法1: `String::new()`

```rust
let mut s = String::new();
s.push_str("hello");
```

---

### 方法2: `to_string()`

```rust
let s = "initial contents".to_string();
```

---

### 方法3: `String::from()`

```rust
let s = String::from("initial contents");
```

**推奨:** `String::from()` が最も一般的

---

## String の更新

### `push_str()` - 文字列を追加

```rust
let mut s = String::from("foo");
s.push_str("bar");
// s = "foobar"
```

**所有権を取らない:**

```rust
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("{}", s2);  // ✅ s2 はまだ使える
```

---

### `push()` - 1文字を追加

```rust
let mut s = String::from("lo");
s.push('l');
// s = "lol"
```

**注意:** シングルクォート `'l'` を使う（char型）

---

### `+` 演算子

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;  // s1 はムーブされる
// println!("{}", s1);  // ❌ エラー！s1 は使えない
println!("{}", s3);  // "Hello, world!"
```

**シグネチャ:**
```rust
fn add(self, s: &str) -> String
```

**ポイント:**
- `s1` の所有権を取る（`self`）
- `s2` は参照（`&str`）

---

### `format!` マクロ（推奨）

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
// s = "tic-tac-toe"

// ✅ s1, s2, s3 はまだ使える
println!("{}", s1);
```

**利点:**
- 所有権を取らない
- 読みやすい
- **推奨**

---

## 文字列の添え字アクセス

### ❌ Rust では添え字アクセスできない

```rust
let s = String::from("hello");
// let h = s[0];  // ❌ コンパイルエラー！
```

---

### なぜ？3つの理由

#### 理由1: UTF-8 エンコーディング

```rust
let hello = String::from("Здравствуйте");
let len = hello.len();  // 24バイト（12文字 × 2バイト）
```

**問題:**
```
s[0] は何を返すべき？
- バイト値 208?（'З' の最初のバイト）
- 文字 'З'?

曖昧すぎる！
```

---

#### 理由2: 1文字のバイト数が違う

```rust
// ASCII: 1バイト
let s1 = String::from("hello");  // 5文字 = 5バイト

// キリル文字: 2バイト
let s2 = String::from("Здравствуйте");  // 12文字 = 24バイト

// 日本語: 3バイト
let s3 = String::from("こんにちは");  // 5文字 = 15バイト
```

---

#### 理由3: パフォーマンスの期待

**添え字アクセスは O(1) であるべき**

```rust
let v = vec![1, 2, 3, 4, 5];
let x = v[3];  // ✅ O(1) - 高速
```

**でも String では O(n) になる:**

```rust
let s = String::from("こんにちは");
// s[2] を取得するには...
// 1. 最初から走査
// 2. 1文字目のバイト数を数える
// 3. 2文字目のバイト数を数える
// 4. やっと3文字目の位置が分かる
// → O(n)
```

**予期しないパフォーマンス問題を防ぐため禁止**

---

## 文字列へのアクセス方法

### 方法1: `chars()` - 文字単位

```rust
let s = String::from("こんにちは");
for c in s.chars() {
    println!("{}", c);
}
// こ
// ん
// に
// ち
// は
```

---

### 方法2: `bytes()` - バイト単位

```rust
let s = String::from("こんにちは");
for b in s.bytes() {
    println!("{}", b);
}
// 227, 129, 147, ... (15個のバイト)
```

---

### 方法3: スライス（危険）

```rust
let hello = "Здравствуйте";

// ✅ OK: 文字境界で切る
let s = &hello[0..4];  // "Зд" (2文字 × 2バイト)

// ❌ パニック: 文字の途中で切る
// let s = &hello[0..1];  // パニック！
```

**注意:** 文字境界でない位置で切るとパニック

---

## String と &str の違い

### String

```rust
let mut s = String::from("hello");
s.push_str(" world");  // ✅ 変更可能
// s = "hello world"
```

**特徴:**
- 所有権を持つ
- ヒープに格納
- 可変
- サイズ可変

---

### &str

```rust
let s: &str = "hello world";
// s.push_str(" !");  // ❌ エラー！不変
```

**特徴:**
- 文字列スライス
- 借用
- 不変
- 固定サイズ

---

## 内部表現

### String = Vec<u8> のラッパー

```rust
let s = String::from("hello");
// 内部的には Vec<u8> として保持
// [104, 101, 108, 108, 111]
```

---

### UTF-8 エンコーディング

```rust
let hello = String::from("Hola");
// 各文字1バイト: [72, 111, 108, 97]
// len() = 4

let hello = String::from("Здравствуйте");
// 各文字2バイト
// len() = 24

let hello = String::from("こんにちは");
// 各文字3バイト
// len() = 15
```

---

## Python との比較

### Python

```python
s = "hello"
print(s[0])      # 'h' ✅ 添え字アクセス可能
print(len(s))    # 5 (文字数)

s = "こんにちは"
print(s[0])      # 'こ' ✅ 簡単
print(len(s))    # 5 (文字数)
```

**Python は内部で Unicode 文字として管理**

---

### Rust

```rust
let s = String::from("hello");
// let c = s[0];  // ❌ エラー
let c = s.chars().nth(0);  // Some('h')
println!("{}", s.len());   // 5 (バイト数)

let s = String::from("こんにちは");
let c = s.chars().nth(0);     // Some('こ')
println!("{}", s.len());      // 15 (バイト数)
println!("{}", s.chars().count());  // 5 (文字数)
```

**Rust は UTF-8 バイト列として管理**

---

## その他のメソッド

### `len()` - バイト数

```rust
let s = String::from("hello");
println!("{}", s.len());  // 5

let s = String::from("こんにちは");
println!("{}", s.len());  // 15（バイト数）
```

---

### `is_empty()` - 空チェック

```rust
let s = String::new();
if s.is_empty() {
    println!("Empty!");
}
```

---

### `contains()` - 部分文字列の検索

```rust
let s = String::from("hello world");
if s.contains("world") {
    println!("Found!");
}
```

---

### `replace()` - 置換

```rust
let s = String::from("I like apples");
let new_s = s.replace("apples", "oranges");
// new_s = "I like oranges"
```

---

### `trim()` - 空白削除

```rust
let s = String::from("  hello  ");
let trimmed = s.trim();
// trimmed = "hello"
```

---

### `split()` - 分割

```rust
let s = String::from("hello world wonderful");
for word in s.split_whitespace() {
    println!("{}", word);
}
// hello
// world
// wonderful
```

---

## まとめ

### String の特徴

```
✅ 所有権を持つ
✅ ヒープに格納
✅ 可変・成長可能
✅ UTF-8 エンコーディング
✅ Vec<u8> のラッパー
```

---

### &str の特徴

```
✅ 文字列スライス
✅ 借用
✅ 不変
✅ 固定サイズ
```

---

### 添え字アクセスが禁止の理由

```
1. バイトと文字の混同を防ぐ
2. UTF-8 で各文字のサイズが違う
3. O(1) のパフォーマンス保証ができない
```

---

### 文字列の操作

```rust
// 作成
let s = String::from("hello");

// 追加
s.push_str(" world");
s.push('!');

// 連結
let s3 = s1 + &s2;           // s1 はムーブ
let s = format!("{}{}", s1, s2);  // ムーブなし（推奨）

// アクセス
for c in s.chars() { }        // 文字単位
for b in s.bytes() { }        // バイト単位
let slice = &s[0..5];         // スライス（危険）
```

---

### ベストプラクティス

```
✅ 連結は format! を使う
✅ 文字単位のアクセスは chars()
✅ スライスは文字境界に注意
✅ len() はバイト数、chars().count() は文字数
```
