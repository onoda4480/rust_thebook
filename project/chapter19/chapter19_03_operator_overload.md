# 第19章 Part 3: 演算子のオーバーロードとデフォルト型パラメータ

## Add トレイトによる演算子のオーバーロード

### 演算子オーバーロードとは？

Rustでは `+`, `-`, `*` などの演算子をカスタム型で使えるようにできる。
これは対応するトレイトを実装することで実現する。

### Add トレイトの定義

```rust
pub trait Add<Rhs = Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

### 実装例: Point 構造体

```rust
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };

    let p3 = p1 + p2;  // ✅ + 演算子が使える！
    // これは実際には p1.add(p2) と同じ

    println!("{:?}", p3);  // Point { x: 4, y: 6 }
}
```

### シンタックスシュガー（構文糖衣）

```rust
p1 + p2          // ← これは実は...
p1.add(p2)       // ← これと同じ！
```

`+` 演算子は `add()` メソッドを呼び出すための**シンタックスシュガー**（見た目を良くするための構文）

---

## デフォルト型パラメータ

### Rhs = Self とは？

```rust
pub trait Add<Rhs = Self> {  // ← Rhs = Self がデフォルト型パラメータ
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}
```

- `Rhs`: "Right Hand Side"（右辺）の略
- `= Self`: デフォルト値。型を指定しなければ `Self` が使われる

### デフォルト型パラメータの2つの目的

---

### 目的1: 既存のコードを破壊せずに型を拡張

**シナリオ**: トレイトにジェネリクス型パラメータを追加したい

**問題点**:
```rust
// 元のトレイト
trait Add {
    fn add(self, rhs: Self) -> Self;
}

// もしデフォルトなしでジェネリクスを追加すると...
trait Add<Rhs> {  // ❌ 既存のコードが壊れる！
    fn add(self, rhs: Rhs) -> Self;
}

// 既存のコード
impl Add for Point { ... }  // ❌ エラー！Rhs を指定していない
```

**解決策**: デフォルト型パラメータ
```rust
trait Add<Rhs = Self> {  // ✅ デフォルトがあるので既存コードも動く
    fn add(self, rhs: Rhs) -> Self;
}

// 既存のコード
impl Add for Point { ... }  // ✅ Rhs = Self が自動的に使われる
```

---

### 目的2: カスタマイズを可能にする

デフォルトを使わずに、異なる型との演算を定義できる:

```rust
use std::ops::Add;

#[derive(Debug)]
struct Millimeters(u32);

#[derive(Debug)]
struct Meters(u32);

// Millimeters + Meters の演算を定義
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn main() {
    let mm = Millimeters(100);
    let m = Meters(2);

    let total = mm + m;  // ✅ Millimeters + Meters ができる
    println!("{:?}", total);  // Millimeters(2100)
}
```

### さらなる例: Point + i32

```rust
impl Add<i32> for Point {
    type Output = Point;

    fn add(self, offset: i32) -> Point {
        Point {
            x: self.x + offset,
            y: self.y + offset,
        }
    }
}

fn main() {
    let p = Point { x: 1, y: 2 };
    let p2 = p + 5;  // Point に i32 を足す
    println!("{:?}", p2);  // Point { x: 6, y: 7 }
}
```

---

## まとめ

### 演算子のオーバーロード
- 対応するトレイト（`Add`, `Sub`, `Mul` など）を実装する
- 演算子はメソッド呼び出しのシンタックスシュガー
- 例: `a + b` は `a.add(b)` と同じ

### デフォルト型パラメータ
- **目的1**: 後方互換性を保つ（既存コードを壊さない）
- **目的2**: カスタマイズを可能にする（異なる型との演算を定義）
- 例: `Add<Rhs = Self>` は `impl Add` だけで `Self` 同士の加算を、`impl Add<i32>` で異なる型との加算を定義できる
