# Chapter 8 Q&A Part 1: ベクタ

## Q1: Vec と配列の違いは？

**質問:** `Vec<T>` と配列 `[T; N]` は何が違うの？

**回答:** **サイズが固定か可変か**が大きな違いです。

---

### 配列 `[T; N]`

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];
// サイズ固定（コンパイル時に決定）
```

**特徴:**
- サイズがコンパイル時に決まる
- スタックに格納
- サイズ変更不可

---

### Vec `Vec<T>`

```rust
let mut v = vec![1, 2, 3];
v.push(4);  // サイズ変更可能
```

**特徴:**
- サイズが実行時に決まる
- ヒープに格納
- サイズ変更可能

---

### 対応表

| 項目 | 配列 `[T; N]` | Vec `Vec<T>` |
|------|--------------|--------------|
| **サイズ** | 固定 | 可変 |
| **格納場所** | スタック | ヒープ |
| **型** | `[i32; 5]` | `Vec<i32>` |
| **変更** | 不可 | 可能（`mut`） |

---

## Q2: Vec に異なる型を入れられる？

**質問:** Python の `list` みたいに、Vec に異なる型を混在できる？

**回答:** **直接はできません。Enum を使います。**

---

### ❌ 直接は不可

```rust
// ❌ エラー
let v = vec![1, "hello", 3.14];
```

**理由:** Vec は同じ型しか格納できない

---

### ✅ Enum を使う

```rust
#[derive(Debug)]
enum Value {
    Int(i32),
    Text(String),
    Float(f64),
}

let v = vec![
    Value::Int(1),
    Value::Text(String::from("hello")),
    Value::Float(3.14),
];

for val in &v {
    println!("{:?}", val);
}
```

---

### Python との比較

#### Python

```python
# ✅ 異なる型を混在できる
v = [1, "hello", 3.14]
```

#### Rust

```rust
// ❌ 直接は不可
// let v = vec![1, "hello", 3.14];

// ✅ Enum で実現
enum Value {
    Int(i32),
    Text(String),
    Float(f64),
}
let v = vec![Value::Int(1), Value::Text(String::from("hello"))];
```

---

## Q3: Vec[0] と v.get(0) の違いは？

**質問:** `v[0]` と `v.get(0)` はどう違う？

**回答:** **エラー処理の方法が違います。**

---

### `v[index]` - 添え字アクセス

```rust
let v = vec![1, 2, 3];
let x = &v[1];  // &2

// ❌ 範囲外アクセスはパニック
// let x = &v[100];  // パニック！
```

**特徴:**
- シンプル
- 範囲外はパニック
- **確実に存在する場合に使う**

---

### `v.get(index)` - 安全なアクセス

```rust
let v = vec![1, 2, 3];

match v.get(1) {
    Some(x) => println!("{}", x),  // 2
    None => println!("Not found"),
}

// 範囲外は None を返す
match v.get(100) {
    Some(x) => println!("{}", x),
    None => println!("Not found"),  // こちらが実行される
}
```

**特徴:**
- `Option<&T>` を返す
- 範囲外は `None`
- **存在しない可能性がある場合に使う**

---

### 使い分け

```rust
let v = vec![1, 2, 3, 4, 5];

// ✅ 確実に存在する
let first = &v[0];

// ✅ 存在しない可能性がある
let user_input = 100;
if let Some(x) = v.get(user_input) {
    println!("{}", x);
}
```

---

## Q4: Vec を走査中に push できない？

**質問:** Vec を走査しながら要素を追加できない？

**回答:** **できません。借用ルールに違反するからです。**

---

### ❌ エラーになるコード

```rust
let mut v = vec![1, 2, 3];

for i in &v {
    v.push(4);  // ❌ エラー！
}
```

**エラー:**
```
error: cannot borrow `v` as mutable because it is also borrowed as immutable
```

---

### 理由

```
1. for i in &v で不変借用
2. v.push() で可変借用を試みる
3. 不変と可変の借用は同時にできない
→ エラー
```

---

### 解決策1: インデックスで走査

```rust
let mut v = vec![1, 2, 3];
let len = v.len();

for i in 0..len {
    v.push(v[i] * 2);
}
// v = [1, 2, 3, 2, 4, 6]
```

---

### 解決策2: clone してから走査

```rust
let mut v = vec![1, 2, 3];
let v_clone = v.clone();

for i in &v_clone {
    v.push(i * 2);
}
// v = [1, 2, 3, 2, 4, 6]
```

---

### 解決策3: collect を使う

```rust
let v = vec![1, 2, 3];
let mut v2: Vec<i32> = v.iter()
    .flat_map(|&x| vec![x, x * 2])
    .collect();
// v2 = [1, 2, 2, 4, 3, 6]
```

---

## Q5: なぜ Display が実装されていない？

**質問:** `println!("{}", row[1]);` でエラーが出るのはなぜ？

**回答:** **独自の Enum は Display トレイトが実装されていないからです。**

---

### 問題のコード

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
];

// ❌ エラー
println!("{}", row[1]);
```

**エラー:**
```
error: `SpreadsheetCell` doesn't implement `std::fmt::Display`
```

---

### 理由

**`{}` を使うには Display トレイトが必要**

- Display = ユーザー向けの表示形式
- Rust は「どう表示すべきか」を自動で決められない
- プログラマが明示的に定義する必要がある

---

### 解決策1: Debug を使う

```rust
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Text(String::from("blue")),
];

// ✅ {:?} を使う
println!("{:?}", row[0]);  // Text("blue")
```

---

### 解決策2: match で値を取り出す

```rust
match &row[0] {
    SpreadsheetCell::Int(i) => println!("Int: {}", i),
    SpreadsheetCell::Float(f) => println!("Float: {}", f),
    SpreadsheetCell::Text(s) => println!("Text: {}", s),
}
```

---

### 解決策3: Display を実装

```rust
use std::fmt;

impl fmt::Display for SpreadsheetCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SpreadsheetCell::Int(i) => write!(f, "{}", i),
            SpreadsheetCell::Float(fl) => write!(f, "{}", fl),
            SpreadsheetCell::Text(s) => write!(f, "{}", s),
        }
    }
}

// ✅ OK
println!("{}", row[0]);  // blue
```

---

## Q6: Vec のメモリ再配置とは？

**質問:** Vec を push するとメモリが再配置されるって？

**回答:** **容量が足りなくなると、より大きなメモリ領域に移動します。**

---

### 仕組み

```rust
let mut v = Vec::new();

// 容量: 0
v.push(1);  // メモリ確保（容量: 4）
v.push(2);  // 容量内
v.push(3);  // 容量内
v.push(4);  // 容量内
v.push(5);  // 容量不足 → 再配置（容量: 8）
```

---

### 問題: 参照が無効になる

```rust
let mut v = vec![1, 2, 3, 4];

let first = &v[0];  // v[0] への参照

v.push(5);  // メモリ再配置される可能性
// → first が無効な参照になる！

// ❌ エラー
// println!("{}", first);
```

---

### なぜエラー？

```
1. first は v[0] のメモリ位置を指す
2. v.push(5) でメモリが再配置
3. v が新しいメモリに移動
4. first は古いメモリ位置を指したまま
5. ダングリングポインタ！
```

**Rust はコンパイル時にこれを検出してエラーにする**

---

### 正しいコード

```rust
let mut v = vec![1, 2, 3, 4];

{
    let first = &v[0];
    println!("{}", first);
}  // first のスコープ終了

v.push(5);  // ✅ OK
```

---

## まとめ

### Vec の重要なポイント

```
✅ 可変長の配列
✅ ヒープに格納
✅ 同じ型のみ（Enum で異なる型を扱える）
✅ v[i] vs v.get(i) の使い分け
✅ 借用ルール（不変と可変は共存できない）
✅ メモリ再配置に注意
```

---

### ベストプラクティス

```
✅ 確実に存在するなら v[i]
✅ 存在しない可能性があるなら v.get(i)
✅ 走査中の変更は避ける
✅ Debug トレイトを derive
✅ メモリ再配置を考慮して参照の扱いに注意
```
