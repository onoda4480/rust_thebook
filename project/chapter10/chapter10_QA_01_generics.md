# 第10章 Q&A (1/3): ジェネリック型

## Q1: ジェネリック型を使う目的は？

### A: コードの重複を避けて、複数の型で動作する汎用的なコードを書くため

**例:**

ジェネリックなし（重複）:
```rust
fn largest_i32(list: &[i32]) -> &i32 { ... }
fn largest_char(list: &[char]) -> &char { ... }
fn largest_f64(list: &[f64]) -> &f64 { ... }
// 型ごとに関数を書く必要がある
```

ジェネリックあり:
```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T { ... }
// 1つの関数で全ての型に対応
```

---

## Q2: ジェネリック型 T を使うとパフォーマンスが低下する？

### A: いいえ、パフォーマンスの低下はありません

**理由:** Rust は**単相化 (monomorphization)** を行うため

**単相化とは:**
- コンパイル時にジェネリック型を具体的な型に展開する
- 実行時には型ごとの専用コードになる

**例:**

コード:
```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T { ... }

let numbers = vec![1, 2, 3];
let chars = vec!['a', 'b', 'c'];

largest(&numbers);  // T = i32
largest(&chars);    // T = char
```

コンパイル後（イメージ）:
```rust
fn largest_i32(list: &[i32]) -> &i32 { ... }
fn largest_char(list: &[char]) -> &char { ... }
```

**結果:** 実行時のコストはゼロ

---

## Q3: `impl<T>` と `impl` の違いは？

### A: `impl<T>` はジェネリック型用、`impl` は具体的な型用

**パターン1: すべての型で使える**

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
//  ^^^
//  ジェネリックな実装
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

// どの型でも使える
let p1 = Point::new(5, 10);      // Point<i32>
let p2 = Point::new(1.0, 4.0);   // Point<f64>
```

---

**パターン2: 特定の型のみで使える**

```rust
impl Point<f32> {
//  ^^^^^^^^^^^^
//  f32 専用の実装
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

let p = Point { x: 3.0, y: 4.0 };
p.distance_from_origin();  // ✅ OK（f32）

let p = Point { x: 3, y: 4 };
// p.distance_from_origin();  // ❌ エラー（i32 では使えない）
```

---

## Q4: トレイト境界とは何？なぜ必要？

### A: ジェネリック型に制約を付けて、特定のトレイトを実装している型のみ受け入れる仕組み

**問題:**

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

**エラー:**
```
error: binary operation `>` cannot be applied to type `&T`
```

**理由:** すべての型が `>` 演算子を使えるわけではない

---

**解決: トレイト境界**

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

**意味:** 「T は PartialOrd トレイトを実装している型でなければならない」

---

**複数のトレイト境界:**

```rust
fn print_and_compare<T: Display + PartialOrd>(a: T, b: T) {
//                   ^^^^^^^^^^^^^^^^^^^^^^^^
//                   Display と PartialOrd の両方
    println!("a = {}, b = {}", a, b);  // Display が必要
    if a > b {                          // PartialOrd が必要
        println!("a is larger");
    }
}
```

---

## Q5: 構造体のジェネリック型パラメータとメソッドの型パラメータの違いは？

### A: 構造体の型パラメータは構造体全体で使われ、メソッドの型パラメータはそのメソッド内でのみ使われる

**例:**

```rust
struct Point<T, U> {
//          ^^^^^^
//          構造体の型パラメータ
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
//  ^^^^^^
//  構造体と同じ型パラメータ
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         ^^^^^^
//         メソッド独自の型パラメータ
        Point {
            x: self.x,   // T を使用
            y: other.y,  // W を使用
        }
    }
}
```

**使用例:**

```rust
let p1 = Point { x: 5, y: 10.4 };       // Point<i32, f64>
//                                         T=i32, U=f64

let p2 = Point { x: "Hello", y: 'c' };  // Point<&str, char>
//                                         V=&str, W=char

let p3 = p1.mixup(p2);                  // Point<i32, char>
//                                         T=i32, W=char
// p3.x = 5 (i32), p3.y = 'c' (char)
```

**ポイント:**
- `T, U`: 構造体全体で使われる
- `V, W`: `mixup` メソッド内でのみ使われる
- 戻り値の型 `Point<T, W>` は、構造体の `T` とメソッドの `W` を組み合わせている

---

## まとめ

### ジェネリック型の重要ポイント

```
✅ 目的: コードの重複を避ける
✅ パフォーマンス: 実行時コストなし（単相化）
✅ impl<T>: すべての型用
✅ impl 具体型: 特定の型専用
✅ トレイト境界: 型に制約を付ける
✅ 複数の型パラメータ: 構造体とメソッドで独立
```
