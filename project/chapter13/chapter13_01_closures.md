# 第13章 まとめ (1/3): クロージャ

## クロージャとは

**クロージャ（Closure）** = 匿名関数（名前のない関数）

```rust
// 関数
fn add_one(x: i32) -> i32 {
    x + 1
}

// クロージャ
let add_one = |x| x + 1;
```

---

## クロージャの記法

### 4つの書き方（すべて同じ）

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }  // 関数
let add_one_v2 = |x: u32| -> u32 { x + 1 }; // フル注釈
let add_one_v3 = |x|             { x + 1 }; // 型省略
let add_one_v4 = |x|               x + 1  ; // 波括弧も省略
```

**特徴:**
- `|` でパラメータを囲む
- 型推論が働く
- 波括弧は省略可能（式が1つの場合）

---

## 環境のキャプチャ

### クロージャは外側の変数を使える

```rust
fn main() {
    let x = 10;

    // ✅ クロージャは x を使える
    let add_x = |num| num + x;
    //                      ^
    //                      環境をキャプチャ

    println!("{}", add_x(5));  // 15
}
```

### 関数は外側の変数を使えない

```rust
fn main() {
    let x = 10;

    // ❌ 関数は x を使えない
    fn add_x(num: i32) -> i32 {
        num + x  // エラー！
    }
}
```

---

## Fn トレイト

Rust には **3つのクロージャトレイト** がある：

### 1. Fn - 不変借用

```rust
let x = 5;

let closure = |num| num + x;  // Fn
//                        ^
//                        x を不変で借用

println!("{}", closure(10));  // 15
println!("{}", closure(20));  // 25 ← 何度でも呼べる
println!("x: {}", x);  // x はまだ使える
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
    count += 1;  // FnMut
    //   ^^^^
    //   count を可変で借用
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

---

### 3. FnOnce - 所有権を奪う

```rust
let s = String::from("hello");

let closure = || {
    println!("{}", s);
    s  // FnOnce
    // ^
    // s の所有権を奪う
};

closure();  // ✅ 1回目は呼べる
// closure();  // ❌ 2回目は呼べない
```

**特徴:**
- 環境の所有権を奪う
- 1回だけ呼べる
- 環境を消費する

---

## move キーワード

```rust
let x = vec![1, 2, 3];

let closure = move |z| z == x;
//            ^^^^
//            x の所有権を奪う

// println!("{:?}", x);  // ❌ x はもう使えない

let y = vec![1, 2, 3];
assert!(closure(y));
```

**用途:**
- スレッドに渡す時
- 所有権を確実に移動したい時

---

## Cacher（メモ化）

### 基本的な Cacher

```rust
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
```

**用途:** 計算結果をキャッシュして、2回目以降は再計算しない

---

### 使用例

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
        //                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        //                             2回目は計算せず、キャッシュを使う
    }
}
```

---

## 改良版 Cacher（HashMap を使う）

### 複数の引数をキャッシュ

```rust
use std::collections::HashMap;
use std::hash::Hash;

struct Cacher<T, K, V>
    where
        T: Fn(K) -> V,
        K: Eq + Hash + Copy,
        V: Copy,
{
    calculation: T,
    values: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
    where
        T: Fn(K) -> V,
        K: Eq + Hash + Copy,
        V: Copy,
{
    fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> V {
        match self.values.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}
```

**改善点:**
- 複数の引数をキャッシュできる
- ジェネリックで任意の型に対応

---

## クロージャの型推論

### 最初の呼び出しで型が確定

```rust
let add = |x| x + 1;

let result1 = add(5);     // x の型が i32 に確定
// let result2 = add(5.0);  // ❌ エラー！もう i32 で確定
```

---

## Python との比較

### Python

```python
# ラムダ式
add_one = lambda x: x + 1

# クロージャ
def make_adder(n):
    def adder(x):
        return x + n  # n をキャプチャ
    return adder

add_10 = make_adder(10)
print(add_10(5))  # 15
```

---

### Rust

```rust
// クロージャ
let add_one = |x| x + 1;

// 環境をキャプチャ
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n  // n をキャプチャ
}

let add_10 = make_adder(10);
println!("{}", add_10(5));  // 15
```

---

## まとめ

```
クロージャ:
✅ 匿名関数（名前なし）
✅ 環境をキャプチャできる
✅ 型推論が働く
✅ 簡潔に書ける

記法:
|x| x + 1           // 最もシンプル
|x: i32| -> i32 { x + 1 }  // 型注釈付き

Fn トレイト:
Fn      → 不変借用（何度でも呼べる）
FnMut   → 可変借用（何度でも呼べる、変更できる）
FnOnce  → 所有権を奪う（1回だけ）

move キーワード:
move |x| ...  → 所有権を奪う

Cacher:
✅ 計算結果をキャッシュ
✅ 2回目以降は再計算しない
✅ HashMap で複数の引数に対応

使い分け:
関数     → 再利用、公開API
クロージャ → 一時的、環境をキャプチャ
```
