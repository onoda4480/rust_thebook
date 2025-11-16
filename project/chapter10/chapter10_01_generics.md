# 第10章 まとめ (1/3): ジェネリック型

## ジェネリック型とは

**定義:** 複数の型で動作する汎用的なコードを書く仕組み

**目的:** コードの重複を避ける

---

## 基本的な使い方

### 関数でのジェネリック

```rust
// i32 専用
fn largest_i32(list: &[i32]) -> &i32 { ... }

// char 専用
fn largest_char(list: &[char]) -> &char { ... }

// ✅ ジェネリック版（任意の型 T）
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

**使用例:**
```rust
let numbers = vec![34, 50, 25, 100, 65];
let result = largest(&numbers);  // T = i32

let chars = vec!['y', 'm', 'a', 'q'];
let result = largest(&chars);    // T = char
```

---

## 構造体でのジェネリック

### 1つの型パラメータ

```rust
struct Point<T> {
    x: T,
    y: T,
}

let integer = Point { x: 5, y: 10 };      // Point<i32>
let float = Point { x: 1.0, y: 4.0 };     // Point<f64>
```

---

### 複数の型パラメータ

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

let both_integer = Point { x: 5, y: 10 };       // Point<i32, i32>
let both_float = Point { x: 1.0, y: 4.0 };      // Point<f64, f64>
let integer_and_float = Point { x: 5, y: 4.0 }; // Point<i32, f64>
```

---

## enum でのジェネリック

### Option<T>

```rust
enum Option<T> {
    Some(T),
    None,
}

let some_number = Some(5);          // Option<i32>
let some_string = Some("hello");    // Option<&str>
let absent_number: Option<i32> = None;
```

---

### Result<T, E>

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

let success: Result<i32, String> = Ok(10);
let failure: Result<i32, String> = Err(String::from("error"));
```

---

## メソッド定義でのジェネリック

### すべての型で使えるメソッド

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }
}
```

**使用例:**
```rust
let p = Point::new(5, 10);      // Point<i32>
let p = Point::new(1.0, 4.0);   // Point<f64>
```

---

### 特定の型専用のメソッド

```rust
// f32 の場合のみ使えるメソッド
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

let p = Point { x: 3.0, y: 4.0 };
println!("{}", p.distance_from_origin());  // 5.0

let p = Point { x: 3, y: 4 };
// p.distance_from_origin();  // ❌ エラー！i32 では使えない
```

---

## 異なる型パラメータを持つメソッド

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

let p1 = Point { x: 5, y: 10.4 };       // Point<i32, f64>
let p2 = Point { x: "Hello", y: 'c' };  // Point<&str, char>
let p3 = p1.mixup(p2);                  // Point<i32, char>
// p3.x = 5, p3.y = 'c'
```

---

## トレイト境界

### 問題

```rust
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {  // ❌ エラー！> が使えるとは限らない
            largest = item;
        }
    }
    largest
}
```

---

### 解決: トレイト境界

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
//         ^^^^^^^^^^^^^^
//         T は PartialOrd を実装している必要がある
    let mut largest = &list[0];
    for item in list {
        if item > largest {  // ✅ OK
            largest = item;
        }
    }
    largest
}
```

---

### 複数のトレイト境界

```rust
fn print_and_compare<T: Display + PartialOrd>(a: T, b: T) {
//                   ^^^^^^^^^^^^^^^^^^^^^^^^
//                   Display と PartialOrd の両方
    println!("a = {}, b = {}", a, b);
    if a > b {
        println!("a is larger");
    }
}
```

---

## パフォーマンス

**重要:** ジェネリックを使ってもパフォーマンスの低下はない

**理由:** Rust は**単相化 (monomorphization)** を行う

```rust
// ジェネリック版
fn largest<T: PartialOrd>(list: &[T]) -> &T { ... }

// コンパイル時に展開される
fn largest_i32(list: &[i32]) -> &i32 { ... }
fn largest_char(list: &[char]) -> &char { ... }
```

**結果:** 実行時のコストはゼロ

---

## Python との比較

### Python（動的型付け）

```python
def largest(list):
    largest = list[0]
    for item in list:
        if item > largest:
            largest = item
    return largest

# どんな型でも使える（型チェックは実行時）
print(largest([1, 2, 3]))
print(largest(['a', 'b', 'c']))
```

---

### Rust（静的型付け + ジェネリック）

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 型安全性はコンパイル時に保証
println!("{}", largest(&vec![1, 2, 3]));
println!("{}", largest(&vec!['a', 'b', 'c']));
```

---

## まとめ

### ジェネリック型の利点

```
✅ コードの重複を避ける
✅ 型安全性を保つ
✅ 実行時のコストゼロ
✅ 複数の型で動作する汎用的なコード
```

---

### 基本構文

```rust
// 関数
fn name<T>(param: T) -> T { ... }

// 構造体
struct Name<T> { field: T }

// enum
enum Name<T> { Variant(T) }

// メソッド
impl<T> Name<T> {
    fn method(&self) -> &T { ... }
}

// 特定の型専用
impl Name<f32> {
    fn special_method(&self) { ... }
}
```

---

### トレイト境界

```rust
// 1つのトレイト
fn name<T: Trait>(param: T) { ... }

// 複数のトレイト
fn name<T: Trait1 + Trait2>(param: T) { ... }

// where 句
fn name<T>(param: T)
where
    T: Trait1 + Trait2,
{
    ...
}
```
