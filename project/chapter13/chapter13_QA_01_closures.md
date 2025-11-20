# 第13章 Q&A (1/3): クロージャ編

## Q1: クロージャの記法が4つあるけど、どういうこと？

### A: すべて同じ動作、型推論のおかげで省略できる

---

### 4つのバージョン

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }  // 1. 関数
let add_one_v2 = |x: u32| -> u32 { x + 1 }; // 2. フル注釈のクロージャ
let add_one_v3 = |x|             { x + 1 }; // 3. 型省略
let add_one_v4 = |x|               x + 1  ; // 4. 波括弧も省略
```

**すべて同じ動作:**
```rust
println!("{}", add_one_v1(5));  // 6
println!("{}", add_one_v2(5));  // 6
println!("{}", add_one_v3(5));  // 6
println!("{}", add_one_v4(5));  // 6
```

---

### 違いは何？

#### v1: 関数

```rust
fn add_one_v1(x: u32) -> u32 { x + 1 }
// ^^ キーワードが必要
// 型注釈が必須
```

---

#### v2: フル注釈のクロージャ

```rust
let add_one_v2 = |x: u32| -> u32 { x + 1 };
//               ^^^^^^^^^^^^^^^^^^^^^^^^^
//               型注釈はあるが省略可能
```

---

#### v3: 型省略

```rust
let add_one_v3 = |x| { x + 1 };
//               ^^^^^^^^^^^^^^
//               型は推論される
```

---

#### v4: 波括弧も省略

```rust
let add_one_v4 = |x| x + 1;
//               ^^^^^^^^^
//               1つの式なら {} 不要
```

---

### 推奨される書き方

```rust
// ✅ 最もシンプル
let double = |x| x * 2;

// ✅ 型を明示したい場合
let double = |x: i32| -> i32 { x * 2 };
```

---

## Q2: Fn トレイトって何？FnMut や FnOnce もあるけど...

### A: クロージャが環境をどう使うかを表す3つのトレイト

---

### 3つのトレイト

```
FnOnce  ← 1回だけ呼べる（所有権を奪う）
  ↑
FnMut   ← 何度でも呼べる（可変借用）
  ↑
Fn      ← 何度でも呼べる（不変借用）
```

---

### 1. Fn - 不変借用

```rust
let x = 5;

let closure = |num| num + x;
//                        ^
//                        x を不変で借用

println!("{}", closure(10));  // 15
println!("{}", closure(20));  // 25 ← 何度でも呼べる
println!("x: {}", x);  // 5 ← x はまだ使える
```

**特徴:**
- 環境を不変で借用
- 何度でも呼べる
- 環境を変更しない

---

### 2. FnMut - 可変借用

```rust
let mut count = 0;

let mut closure = || {
    count += 1;
    //   ^^^^^ count を可変で借用
    count
};

println!("{}", closure());  // 1
println!("{}", closure());  // 2
println!("{}", closure());  // 3
```

**特徴:**
- 環境を可変で借用
- 何度でも呼べる
- 環境を変更できる
- `mut` キーワードが必要

---

### 3. FnOnce - 所有権を奪う

```rust
let s = String::from("hello");

let closure = || {
    println!("{}", s);
    s  // s の所有権を奪う
};

closure();  // ✅ 1回目は呼べる
// closure();  // ❌ 2回目は呼べない
```

**特徴:**
- 環境の所有権を奪う
- 1回だけ呼べる
- 環境を消費する

---

### どう使い分ける？

```rust
// Fn: 読むだけ
let add = |x| x + 10;

// FnMut: 変更する
let mut count = 0;
let mut increment = || count += 1;

// FnOnce: 消費する
let s = String::from("hello");
let consume = || s;
```

---

### 関数の引数として

```rust
// Fn を要求
fn call_twice<F>(f: F)
    where F: Fn(i32) -> i32
{
    f(1);
    f(2);  // 2回呼ぶので Fn
}

// FnMut を要求
fn call_and_mutate<F>(mut f: F)
    where F: FnMut()
{
    f();
    f();  // 変更するので FnMut
}

// FnOnce を要求
fn call_once<F>(f: F)
    where F: FnOnce()
{
    f();  // 1回だけなので FnOnce
}
```

---

## Q3: クロージャと関数、どっちを使えばいい？

### A: 環境をキャプチャするならクロージャ、それ以外は場合による

---

### クロージャを使うべき場合

#### 1. 環境の変数を使いたい

```rust
let threshold = 10;

let numbers = vec![5, 12, 8, 15, 3];

// ✅ クロージャ: threshold を使える
let large: Vec<&i32> = numbers.iter()
    .filter(|&&x| x > threshold)
    //              ^^^^^^^^^
    //              環境をキャプチャ
    .collect();
```

---

#### 2. 一時的な処理

```rust
// ✅ クロージャ: その場限り
let doubled: Vec<i32> = vec![1, 2, 3]
    .iter()
    .map(|x| x * 2)
    .collect();
```

---

#### 3. コールバック

```rust
// ✅ クロージャ: unwrap_or_else
Config::new(&args).unwrap_or_else(|err| {
    eprintln!("Error: {}", err);
    process::exit(1);
});
```

---

### 関数を使うべき場合

#### 1. 再利用したい

```rust
// ✅ 関数: 色々な場所で使う
fn calculate_tax(amount: f64) -> f64 {
    amount * 0.1
}

fn main() {
    let tax1 = calculate_tax(100.0);
    let tax2 = calculate_tax(200.0);
}
```

---

#### 2. 公開API

```rust
// ✅ 関数: 他のクレートから使える
pub fn process_data(data: &[i32]) -> Vec<i32> {
    data.iter().map(|x| x * 2).collect()
}
```

---

#### 3. 複雑なロジック

```rust
// ✅ 関数: 複雑で長い処理
pub fn validate_user(user: &User) -> Result<(), String> {
    if user.name.is_empty() {
        return Err("Name is empty".to_string());
    }
    if user.age < 18 {
        return Err("Too young".to_string());
    }
    // ... 他にも色々
    Ok(())
}
```

---

### 比較表

| 特徴 | クロージャ | 関数 |
|------|----------|------|
| **環境をキャプチャ** | ✅ できる | ❌ できない |
| **型推論** | ✅ 働く | ❌ 型注釈必須 |
| **名前** | 不要 | 必要 |
| **再利用** | △ その場限り | ✅ 色々な場所で |
| **公開API** | ❌ 難しい | ✅ 簡単 |

---

### 実例で比較

#### ケース1: イテレータと組み合わせ

```rust
// ✅ クロージャが便利
let doubled: Vec<i32> = vec![1, 2, 3]
    .iter()
    .map(|x| x * 2)
    .collect();

// △ 関数でも可能だが冗長
fn double(x: &i32) -> i32 { x * 2 }
let doubled: Vec<i32> = vec![1, 2, 3]
    .iter()
    .map(double)
    .collect();
```

---

#### ケース2: 環境をキャプチャ

```rust
let multiplier = 10;

// ✅ クロージャ: multiplier を使える
let result: Vec<i32> = vec![1, 2, 3]
    .iter()
    .map(|x| x * multiplier)
    .collect();

// ❌ 関数: multiplier を使えない
fn multiply(x: &i32) -> i32 {
    x * multiplier  // エラー！
}
```

---

#### ケース3: 複雑なビジネスロジック

```rust
// ✅ 関数: テストしやすい、再利用しやすい
pub fn calculate_price(item: &Item, discount: f64) -> f64 {
    let base_price = item.base_price;
    let tax = base_price * 0.1;
    let discounted = base_price * (1.0 - discount);
    discounted + tax
}

// △ クロージャ: テストしにくい
let calculate_price = |item: &Item, discount: f64| {
    let base_price = item.base_price;
    let tax = base_price * 0.1;
    let discounted = base_price * (1.0 - discount);
    discounted + tax
};
```

---

### まとめ

```
クロージャを使う:
✅ 環境の変数を使いたい
✅ 一時的な処理
✅ イテレータと組み合わせ
✅ コールバック
✅ 短い処理

関数を使う:
✅ 再利用したい
✅ 環境をキャプチャしない
✅ 公開API
✅ 複雑なロジック
✅ テストしやすくしたい

迷ったら:
→ まずクロージャで書いてみる
→ 再利用が必要になったら関数にする
```

---

## Python との比較

### Python

```python
# ラムダ式（クロージャ相当）
double = lambda x: x * 2

# 関数
def double(x):
    return x * 2

# 環境をキャプチャ
multiplier = 10
result = list(map(lambda x: x * multiplier, [1, 2, 3]))
```

---

### Rust

```rust
// クロージャ
let double = |x| x * 2;

// 関数
fn double(x: i32) -> i32 { x * 2 }

// 環境をキャプチャ
let multiplier = 10;
let result: Vec<i32> = vec![1, 2, 3]
    .iter()
    .map(|x| x * multiplier)
    .collect();
```
