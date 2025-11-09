# Chapter 4-1: 所有権（Ownership）

## 所有権とは？

Rustの最も重要な機能。メモリ安全性をガベージコレクションなしで実現する。

---

## 所有権のルール

1. **各値は「所有者」という変数を持つ**
2. **所有者は同時に1つだけ**
3. **所有者がスコープを抜けると、値は破棄される（drop）**

---

## スタックとヒープ

### スタック（Stack）

| 特徴 | 説明 |
|---|---|
| 速度 | 超高速 |
| サイズ | コンパイル時に確定 |
| 順序 | LIFO（後入れ先出し） |
| 例 | i32, bool, char, 配列 |

```rust
let x = 5;        // スタックに保存
let y = true;     // スタックに保存
let z = 'a';      // スタックに保存
```

### ヒープ（Heap）

| 特徴 | 説明 |
|---|---|
| 速度 | スタックより遅い |
| サイズ | 実行時に決定可能 |
| 管理 | メモリ確保・解放が必要 |
| 例 | String, Vec, Box |

```rust
let s = String::from("hello");  // ヒープに保存
```

**メモリ構造:**
```
スタック（s）       ヒープ
┌────────┐       ┌─┬─┬─┬─┬─┐
│ ptr    │──────→│h│e│l│l│o│
│ len: 5 │       └─┴─┴─┴─┴─┘
│ cap: 5 │
└────────┘
```

---

## Copy型 vs ムーブ型

### Copy型（スタック）

**自動的にコピーされる**

```rust
let x = 5;
let y = x;  // コピー

println!("{} {}", x, y);  // 両方使える！
```

**Copy型の一覧:**
- 全ての整数型（i32, u64など）
- bool
- 全ての浮動小数点型（f32, f64）
- char
- タプル（中身が全てCopy型なら）

### ムーブ型（ヒープ）

**所有権が移動する**

```rust
let s1 = String::from("hello");
let s2 = s1;  // ムーブ

// println!("{}", s1);  // ❌ エラー！s1は無効
println!("{}", s2);  // ✅ OK
```

---

## ムーブの仕組み

### Stringのムーブ

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1からs2へムーブ
```

**メモリの状態:**
```
ムーブ前:
s1 → ヒープ["hello"]

ムーブ後:
s1: 無効
s2 → ヒープ["hello"]
```

**重要:** ヒープのデータはコピーされない。ポインタだけが移動する。

---

## クローン（Clone）

**明示的にヒープをコピーする**

```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // ヒープのデータをコピー

println!("{} {}", s1, s2);  // 両方使える！
```

**メモリコスト:** 高い（ヒープのデータを完全コピー）

---

## 関数と所有権

### 所有権のムーブ

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);  // sがムーブ

    // println!("{}", s);  // ❌ エラー！sは無効
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}  // some_stringがdrop
```

### Copy型は影響なし

```rust
fn main() {
    let x = 5;
    makes_copy(x);  // xがコピー

    println!("{}", x);  // ✅ OK！xは有効
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
```

---

## 戻り値と所有権

### 所有権を返す

```rust
fn gives_ownership() -> String {
    let s = String::from("hello");
    s  // 所有権を呼び出し元に返す
}

fn main() {
    let s1 = gives_ownership();  // 所有権を受け取る
    println!("{}", s1);  // ✅ OK
}
```

### 所有権を受け取って返す

```rust
fn takes_and_gives_back(s: String) -> String {
    s  // そのまま返す
}

fn main() {
    let s1 = String::from("hello");
    let s2 = takes_and_gives_back(s1);  // s1はムーブ

    // println!("{}", s1);  // ❌ エラー
    println!("{}", s2);  // ✅ OK
}
```

---

## スコープとdrop

### スコープの基本

```rust
fn main() {  // ← スコープ開始
    let s = String::from("hello");  // sが有効になる

    // sを使う

}  // ← スコープ終了、sが自動的にdrop
```

### ムーブされた変数はdropされない

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1からs2へムーブ

}  // s1: dropされない（既にムーブ済み）
  // s2: dropされる（所有者だから）
```

**Rustコンパイラが自動判断:**
- ムーブ済み変数 → dropしない
- 有効な変数 → dropする

---

## まとめ

### 重要ポイント

| 概念 | 説明 |
|---|---|
| **所有権** | 各値は1つの所有者を持つ |
| **ムーブ** | ヒープ型は所有権が移動 |
| **コピー** | スタック型は自動コピー |
| **クローン** | 明示的なヒープコピー |
| **drop** | スコープ終了時に自動解放 |

### スタック vs ヒープ

```
スタック型 → 自動Copy → 両方使える
ヒープ型   → ムーブ   → 元は無効
```

### 安全性の保証

Rustの所有権システムにより：
- ✅ メモリリークなし
- ✅ 二重解放なし
- ✅ ダングリングポインタなし
- ✅ データ競合なし

全てコンパイル時にチェック！
