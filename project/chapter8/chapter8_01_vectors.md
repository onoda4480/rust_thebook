# Chapter 8-1: ベクタ (Vec<T>)

## ベクタとは？

**同じ型の値を可変長で格納できるコレクション**

```rust
let v: Vec<i32> = Vec::new();
```

---

## ベクタの特徴

| 特徴 | 説明 |
|------|------|
| **可変長** | 要素数を実行時に変更可能 |
| **同じ型** | 全要素が同じ型 |
| **ヒープ** | データはヒープに格納 |
| **連続メモリ** | 要素は連続したメモリに配置 |

---

## Python との対応

| Rust | Python |
|------|--------|
| `Vec<T>` | `list` |
| `vec![1, 2, 3]` | `[1, 2, 3]` |
| `v.push(4)` | `v.append(4)` |
| `v.pop()` | `v.pop()` |

---

## ベクタの作成

### 方法1: `Vec::new()`

```rust
// 空のベクタを作成（型注釈が必要）
let v: Vec<i32> = Vec::new();
```

---

### 方法2: `vec!` マクロ

```rust
// 初期値付きで作成（型推論される）
let v = vec![1, 2, 3];
```

**推奨:** 初期値がある場合は `vec!` マクロが便利

---

## 要素の追加

### `push()` メソッド

```rust
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);
// v = [5, 6, 7]
```

**注意:** `mut` が必要

---

## 要素の読み取り

### 方法1: 添え字アクセス

```rust
let v = vec![1, 2, 3, 4, 5];
let third = &v[2];  // 3
println!("The third element is {}", third);
```

**特徴:**
- シンプル
- 範囲外アクセスは**パニック**

---

### 方法2: `get()` メソッド

```rust
let v = vec![1, 2, 3, 4, 5];
match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

**特徴:**
- `Option<&T>` を返す
- 範囲外アクセスは `None`
- **安全**

---

### 使い分け

```rust
let v = vec![1, 2, 3];

// ✅ 絶対に存在する場合
let x = &v[0];

// ✅ 存在しない可能性がある場合
if let Some(x) = v.get(10) {
    println!("{}", x);
}

// ❌ パニック
// let x = &v[100];
```

---

## 借用ルール

### 重要: 可変参照と不変参照は共存できない

```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];  // 不変借用

v.push(6);  // ❌ エラー！可変借用

println!("The first element is: {}", first);
```

**エラー:**
```
error: cannot borrow `v` as mutable because it is also borrowed as immutable
```

---

### なぜエラー？

**ベクタは要素を追加するとメモリ再配置する可能性がある**

```
1. first は v[0] への参照
2. v.push(6) でメモリが足りなくなる
3. ベクタが新しいメモリに移動
4. first が無効な参照になる
→ ダングリングポインタ！
```

---

### 正しいコード

```rust
let mut v = vec![1, 2, 3, 4, 5];

{
    let first = &v[0];  // 不変借用
    println!("The first element is: {}", first);
}  // first のスコープ終了

v.push(6);  // ✅ OK
```

---

## 要素の走査

### 不変参照で走査

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

---

### 可変参照で走査

```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;  // 参照外し演算子
}
// v = [150, 82, 107]
```

**ポイント:** `*i` で参照を外して値を変更

---

## Python との比較

### Python

```python
v = [100, 32, 57]
for i in v:
    print(i)

# 値を変更
for i in range(len(v)):
    v[i] += 50
```

---

### Rust

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}

// 値を変更
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

---

## 複数の型を格納する

### 問題: Vec は1つの型しか格納できない

```rust
// ❌ エラー
let v = vec![1, "hello"];  // i32 と &str は混在できない
```

---

### 解決策: Enum を使う

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

**ポイント:** Enum の各バリアントは異なる型を持てるが、Enum 自体は1つの型

---

### 値の取得

```rust
for cell in &row {
    match cell {
        SpreadsheetCell::Int(i) => println!("Int: {}", i),
        SpreadsheetCell::Float(f) => println!("Float: {}", f),
        SpreadsheetCell::Text(s) => println!("Text: {}", s),
    }
}
```

---

## ベクタの削除

### `pop()` メソッド

```rust
let mut v = vec![1, 2, 3];
let last = v.pop();  // Some(3)
// v = [1, 2]
```

**戻り値:** `Option<T>`

---

### スコープを抜けると自動削除

```rust
{
    let v = vec![1, 2, 3, 4];
    // v を使う
}  // v がスコープを抜け、メモリが解放される
```

---

## その他のメソッド

### `len()` - 要素数

```rust
let v = vec![1, 2, 3];
println!("Length: {}", v.len());  // 3
```

---

### `is_empty()` - 空チェック

```rust
let v: Vec<i32> = Vec::new();
if v.is_empty() {
    println!("Empty!");
}
```

---

### `clear()` - 全削除

```rust
let mut v = vec![1, 2, 3];
v.clear();
// v = []
```

---

### `contains()` - 要素の存在チェック

```rust
let v = vec![1, 2, 3];
if v.contains(&2) {
    println!("Found!");
}
```

---

### `remove()` - 指定位置の削除

```rust
let mut v = vec![1, 2, 3, 4];
v.remove(1);  // インデックス1を削除
// v = [1, 3, 4]
```

---

### `insert()` - 指定位置に挿入

```rust
let mut v = vec![1, 2, 3];
v.insert(1, 99);  // インデックス1に99を挿入
// v = [1, 99, 2, 3]
```

---

## まとめ

### ベクタの特徴

```
✅ 可変長の配列
✅ 同じ型の要素を格納
✅ ヒープに確保
✅ 所有権のルールが適用される
```

---

### 基本操作

```rust
// 作成
let mut v = vec![1, 2, 3];

// 追加
v.push(4);

// アクセス
let x = &v[0];           // パニックの可能性
let x = v.get(0);        // 安全

// 走査
for i in &v { }          // 不変
for i in &mut v { }      // 可変

// 削除
v.pop();
```

---

### 借用ルール

```
✅ 不変参照は複数持てる
✅ 可変参照は1つだけ
❌ 不変参照と可変参照は共存できない
❌ push() でメモリ再配置される可能性
```

---

### Enum で複数の型

```rust
enum Cell {
    Int(i32),
    Text(String),
}

let v = vec![
    Cell::Int(3),
    Cell::Text(String::from("blue")),
];
```

---

### Python との対応

| 操作 | Rust | Python |
|------|------|--------|
| **作成** | `vec![1, 2, 3]` | `[1, 2, 3]` |
| **追加** | `v.push(4)` | `v.append(4)` |
| **削除** | `v.pop()` | `v.pop()` |
| **長さ** | `v.len()` | `len(v)` |
| **走査** | `for i in &v` | `for i in v` |
| **アクセス** | `&v[0]` | `v[0]` |
