# RustのCopyとClone

## 基本ルール

**スタックだけで完結 → 自動Copy**
**ヒープを所有する → .clone() が必要**

---

## Copy トレイトの判定ルール

### ✅ Copy になる条件
- **単純なスカラー値の集合** = スタック上に全データが収まる型
- コンパイル時にサイズが確定している

### ❌ Copy にならない条件
1. **メモリ確保が必要** → ヒープを使う
2. **何らかのリソース** → ファイルハンドル、ネットワーク接続など

---

## 自動コピーされる型（Copy トレイト）

### 1. あらゆる整数型
```rust
let a: i8 = 1;      // Copy ✅
let b: i16 = 2;     // Copy ✅
let c: i32 = 3;     // Copy ✅
let d: i64 = 4;     // Copy ✅
let e: i128 = 5;    // Copy ✅
let f: u8 = 6;      // Copy ✅
let g: usize = 7;   // Copy ✅

let x = 5;
let y = x;  // 自動的にコピーされる
println!("{} {}", x, y);  // 両方使える！
```

### 2. 論理値型 bool
```rust
let flag: bool = true;
let another = flag;  // 自動コピー
println!("{} {}", flag, another);  // 両方使える
```

### 3. 浮動小数点型
```rust
let x: f32 = 3.14;   // Copy ✅
let y: f64 = 2.71;   // Copy ✅
```

### 4. 文字型 char（固定4バイト）
```rust
let c1: char = 'a';      // 4バイト
let c2: char = 'あ';     // 4バイト
let c3: char = '🦀';     // 4バイト（絵文字も！）

// 全て同じサイズ = 固定長 = スタックに収まる = Copy ✅
let d = c1;
println!("{} {}", c1, d);  // 両方使える
```

### 5. タプル（条件付き）

#### ✅ 中身が全て Copy なら Copy
```rust
let t1: (i32, i32) = (1, 2);              // Copy ✅
let t2: (i32, f64, char) = (1, 3.14, 'a'); // Copy ✅
let t3: (bool, u8) = (true, 255);         // Copy ✅

let copied = t1;
println!("{:?} {:?}", t1, copied);  // 両方使える
```

#### ❌ 中身に Copy でない型があると Copy ではない
```rust
let t4: (i32, String) = (1, String::from("hello"));  // Copy ❌
let moved = t4;
// println!("{:?}", t4);  // エラー！ムーブされた
```

### 6. 配列（中身がCopyなら）
```rust
// 配列は128バイトでもCopy可能！（スタックだけだから）
let big_array: [i32; 32] = [0; 32];  // 128バイト
let copy = big_array;  // コピーされる！
```

---

## 明示的にクローンが必要な型

### ヒープを使う型
```rust
// String はヒープにメモリ確保する
let s1 = String::from("hello");
let s2 = s1;  // ムーブ（コピーではない）
// println!("{}", s1);  // エラー！

// 明示的にクローン
let s3 = String::from("hello");
let s4 = s3.clone();  // .clone() で明示的にコピー
println!("{} {}", s3, s4);  // 両方使える！

// Vec（可変長配列）
let v1: Vec<i32> = vec![1, 2, 3];
let v2 = v1.clone();  // クローンが必要

// Box（ヒープ上のデータ）
let b1: Box<i32> = Box::new(5);
let b2 = b1.clone();  // クローンが必要
```

---

## 特殊ケース：参照型

参照自体はポインタなので**スタック**に置かれる → Copy ✅

```rust
// &str: 参照自体（ポインタ+長さ）はスタック
let s: &str = "hello";  // データは静的領域など
let t = s;              // 参照がコピーされる（Copy ✅）
println!("{} {}", s, t);  // 両方使える

// &T: 参照
let x = 5;
let r1 = &x;
let r2 = r1;  // 参照のコピー（Copy ✅）
println!("{} {}", r1, r2);  // 両方使える
```

---

## まとめ表

| 型 | サイズ | 保存場所 | Copy? | 理由 |
|---|---|---|---|---|
| `i32`, `u64` など | 固定 | スタック | ✅ | スカラー値 |
| `bool` | 固定1バイト | スタック | ✅ | スカラー値 |
| `f64` など | 固定 | スタック | ✅ | スカラー値 |
| `char` | 固定4バイト | スタック | ✅ | 固定長Unicode |
| `(i32, bool)` | 固定 | スタック | ✅ | 中身が全てCopy |
| `[i32; 100]` | 固定 | スタック | ✅ | スタックに収まる |
| `(i32, String)` | - | - | ❌ | StringがCopy不可 |
| `String` | 可変長 | ヒープ | ❌ | ヒープメモリ確保 |
| `Vec<T>` | 可変長 | ヒープ | ❌ | ヒープメモリ確保 |
| `Box<T>` | - | ヒープ | ❌ | ヒープメモリ所有 |
| `&str` | 固定16バイト | スタック（参照） | ✅ | 参照自体はスタック |
| `&T` | 固定8バイト | スタック（参照） | ✅ | ポインタのコピー |
| `File` | - | - | ❌ | リソース |

---

## なぜこのルールなのか？

### Rustの設計思想
**「勝手に重い処理が走らない」**

```rust
// もしStringがCopyだったら...
let s1 = String::from("非常に長い文字列...");
let s2 = s1;  // 勝手にヒープメモリが大量にコピーされて遅くなる！

// だからRustでは明示的にclone()が必要
let s3 = String::from("非常に長い文字列...");
let s4 = s3.clone();  // 「重い処理だ」と認識できる
```

### リソース型がCopyできない理由
```rust
use std::fs::File;

let file1 = File::open("test.txt").unwrap();
let file2 = file1;  // ムーブ

// もしFileがCopyだったら...
// ファイルハンドルが複製されて、どちらがファイルを閉じるか曖昧になる！
```

---

## 実践例：整数 vs String

```rust
// 整数: 自動コピー
let x = 5;
let y = x;  // 自動的にコピー（たった数バイト）
println!("x = {}, y = {}", x, y);  // OK!

// String: ムーブ
let s1 = String::from("hello");
let s2 = s1;  // ムーブ（コピーではない）
// println!("{}", s1);  // エラー！s1は無効化された

// String: 明示的クローン
let s3 = String::from("hello");
let s4 = s3.clone();  // 明示的にクローン
println!("{} {}", s3, s4);  // OK!
```

---

## キーポイント

1. **固定長 = コンパイル時にサイズが分かる = スタック = Copy**
2. **可変長 = 実行時にサイズが決まる = ヒープ = Clone必要**
3. **線引き = サイズではなく「ヒープを所有するか」**
4. **参照は特殊: 参照自体（ポインタ）はCopy可能**
