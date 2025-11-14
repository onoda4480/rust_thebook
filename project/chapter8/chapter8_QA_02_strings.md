# Chapter 8 Q&A Part 2: 文字列

## Q1: String と &str の違いは？

**質問:** `String` と `&str` は何が違うの？どう使い分ける？

**回答:** **所有権を持つか、借用かの違いです。**

---

### String

```rust
let mut s = String::from("hello");
s.push_str(" world");  // ✅ 変更可能
```

**特徴:**
- 所有権を持つ
- ヒープに格納
- 可変（`mut`）
- サイズ可変

**用途:** 文字列を動的に組み立てる、変更する

---

### &str

```rust
let s: &str = "hello world";
// s.push_str(" !");  // ❌ エラー！不変
```

**特徴:**
- 文字列スライス（借用）
- 不変
- 固定サイズ

**用途:** 文字列リテラル、読み取り専用

---

### 変換

```rust
// &str → String
let s1: &str = "hello";
let s2: String = s1.to_string();
let s3: String = String::from(s1);

// String → &str
let s4: String = String::from("hello");
let s5: &str = &s4;
let s6: &str = s4.as_str();
```

---

### 使い分け

```rust
// ✅ 関数の引数は &str を受け取る（柔軟）
fn print_string(s: &str) {
    println!("{}", s);
}

print_string("hello");  // &str リテラル
print_string(&String::from("hello"));  // String の参照

// ✅ 所有権が必要なら String を返す
fn create_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

---

## Q2: なぜ添え字アクセスできない？

**質問:** Python だと `s[0]` で1文字目を取れるのに、Rust ではできないのはなぜ？

**回答:** **3つの理由があります。**

---

### 理由1: バイトと文字の混同

```rust
let s = String::from("hello");
// s[0] は何を返すべき？
// - バイト値 104? ('h' の ASCII コード)
// - 文字 'h'?

// 曖昧！
```

---

### 理由2: UTF-8 エンコーディング

```rust
let s = String::from("こんにちは");
let len = s.len();  // 15バイト

// 文字数は5、バイト数は15
// s[0] は何を返すべき？
// - バイト位置0? → 意味のないデータ
// - 文字位置0? → O(n) の計算が必要
```

**1文字のバイト数が違う:**
```
'h' = 1バイト
'З' = 2バイト (キリル文字)
'こ' = 3バイト (日本語)
'🦀' = 4バイト (絵文字)
```

---

### 理由3: パフォーマンスの期待

**添え字アクセスは O(1) であるべき**

```rust
// Vec の場合
let v = vec![1, 2, 3, 4, 5];
let x = v[3];  // ✅ O(1) - 高速

// String の場合（もし許可されていたら）
let s = String::from("こんにちは");
// s[2] を取得するには...
// 1文字目のバイト数を数える
// 2文字目のバイト数を数える
// → O(n) になる！
```

**予期しないパフォーマンス問題を防ぐため禁止**

---

### Python との比較

#### Python

```python
s = "こんにちは"
print(s[0])    # 'こ' - O(1) ✅
print(len(s))  # 5 (文字数)
```

**Python は内部で Unicode 文字として管理**
- メモリを多く使う
- アクセスは高速

---

#### Rust

```rust
let s = String::from("こんにちは");
let c = s.chars().nth(0);     // Some('こ') - O(n)
println!("{}", s.len());      // 15 (バイト数)
println!("{}", s.chars().count());  // 5 (文字数)
```

**Rust は UTF-8 バイト列として管理**
- メモリ効率が良い
- アクセスには走査が必要

---

## Q3: 文字列にアクセスする方法は？

**質問:** 添え字が使えないなら、どうやって文字列にアクセスする？

**回答:** **3つの方法があります。**

---

### 方法1: `chars()` - 文字単位

```rust
let s = String::from("こんにちは");

// 1文字目を取得
if let Some(c) = s.chars().nth(0) {
    println!("{}", c);  // 'こ'
}

// 全文字を走査
for c in s.chars() {
    println!("{}", c);
}
// こ
// ん
// に
// ち
// は
```

**推奨:** 文字単位で処理したい場合

---

### 方法2: `bytes()` - バイト単位

```rust
let s = String::from("hello");

for b in s.bytes() {
    println!("{}", b);
}
// 104 ('h')
// 101 ('e')
// 108 ('l')
// 108 ('l')
// 111 ('o')
```

**用途:** バイトレベルの処理が必要な場合

---

### 方法3: スライス - 範囲指定

```rust
let s = String::from("hello");
let slice = &s[0..2];  // "he"

// 日本語の場合（注意が必要）
let s = String::from("こんにちは");
let slice = &s[0..3];  // "こ" (1文字 = 3バイト)

// ❌ 文字の途中で切るとパニック
// let slice = &s[0..1];  // パニック！
```

**⚠️ 注意:** 文字境界でないとパニック

---

## Q4: 文字列の連結方法は？

**質問:** 文字列を連結する方法は？

**回答:** **3つの方法があります。**

---

### 方法1: `+` 演算子

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;

// ❌ s1 は使えない（所有権が移動）
// println!("{}", s1);

// ✅ s2 は使える
println!("{}", s2);  // world!
println!("{}", s3);  // Hello, world!
```

**注意:**
- `s1` の所有権が移動
- `s2` は参照（`&`）

---

### 方法2: `push_str()`

```rust
let mut s1 = String::from("Hello, ");
let s2 = "world!";
s1.push_str(s2);

println!("{}", s1);  // Hello, world!
println!("{}", s2);  // world! ✅ s2 は使える
```

---

### 方法3: `format!` マクロ（推奨）

```rust
let s1 = String::from("Hello");
let s2 = String::from("world");
let s3 = format!("{}, {}!", s1, s2);

// ✅ s1, s2 も使える
println!("{}", s1);  // Hello
println!("{}", s2);  // world
println!("{}", s3);  // Hello, world!
```

**利点:**
- 所有権を取らない
- 読みやすい
- **推奨**

---

### 使い分け

```rust
// ✅ 単純な連結
let s = s1 + &s2;

// ✅ 既存の String に追加
s1.push_str(&s2);

// ✅ 複数の文字列を組み合わせる（推奨）
let s = format!("{}-{}-{}", s1, s2, s3);
```

---

## Q5: len() と chars().count() の違いは？

**質問:** `s.len()` と `s.chars().count()` は何が違う？

**回答:** **バイト数か文字数かの違いです。**

---

### `len()` - バイト数

```rust
let s1 = String::from("hello");
println!("{}", s1.len());  // 5バイト

let s2 = String::from("こんにちは");
println!("{}", s2.len());  // 15バイト (5文字 × 3バイト)
```

**戻り値:** UTF-8 でエンコードされたバイト数

---

### `chars().count()` - 文字数

```rust
let s1 = String::from("hello");
println!("{}", s1.chars().count());  // 5文字

let s2 = String::from("こんにちは");
println!("{}", s2.chars().count());  // 5文字
```

**戻り値:** Unicode スカラー値の数

---

### Python との比較

```python
s = "こんにちは"
print(len(s))  # 5 (文字数)

# バイト数を取得
print(len(s.encode('utf-8')))  # 15
```

---

### Rust

```rust
let s = String::from("こんにちは");
println!("{}", s.chars().count());  // 5 (文字数)
println!("{}", s.len());            // 15 (バイト数)
```

---

## Q6: 文字列リテラルの型は？

**質問:** `"hello"` の型は何？

**回答:** **`&str` です。**

---

### 文字列リテラル

```rust
let s: &str = "hello";
// s の型は &str
```

**特徴:**
- プログラムのバイナリに埋め込まれる
- 静的ライフタイム（`'static`）
- 不変

---

### String への変換

```rust
// &str から String
let s1: &str = "hello";
let s2: String = s1.to_string();
let s3: String = String::from(s1);

// String から &str
let s4: String = String::from("hello");
let s5: &str = &s4;
```

---

## まとめ

### String vs &str

```
String:
✅ 所有権を持つ
✅ ヒープに格納
✅ 可変・成長可能

&str:
✅ 文字列スライス（借用）
✅ 不変
✅ 固定サイズ
```

---

### 添え字アクセスが禁止の理由

```
1. バイトと文字の混同
2. UTF-8 で各文字のサイズが違う
3. O(1) のパフォーマンス保証ができない
```

---

### 文字列へのアクセス

```rust
// 文字単位
for c in s.chars() { }

// バイト単位
for b in s.bytes() { }

// スライス（危険）
let slice = &s[0..5];
```

---

### 文字列の連結

```rust
// + 演算子（所有権移動）
let s3 = s1 + &s2;

// push_str（追加）
s1.push_str(&s2);

// format! マクロ（推奨）
let s = format!("{}{}", s1, s2);
```

---

### ベストプラクティス

```
✅ 関数の引数は &str
✅ 所有権が必要なら String を返す
✅ 連結は format! を使う
✅ len() はバイト数、chars().count() は文字数
✅ 文字単位のアクセスは chars()
```
