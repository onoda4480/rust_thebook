# Chapter 5-2: メソッドと関連関数

## メソッドとは？

**構造体に紐付いた関数**

---

## 関数 vs メソッド

### 関数（従来）

```rust
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// 呼び出し
let rect = Rectangle { width: 30, height: 50 };
let a = area(&rect);
```

### メソッド（改善）

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// 呼び出し
let rect = Rectangle { width: 30, height: 50 };
let a = rect.area();  // シンプル！
```

---

## メソッドの定義

### impl ブロック

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}
```

**構成要素:**
- `impl` キーワード
- 構造体名
- メソッド定義

---

## `self` とは？

### self = メソッドを呼んだインスタンス自身

```rust
let rect = Rectangle { width: 30, height: 50 };
rect.area();
// ↓ メソッド内では
// self = rect
// self.width = 30
// self.height = 50
```

### Python の self と同じ

```python
# Python
class Rectangle:
    def area(self):
        return self.width * self.height

rect = Rectangle()
rect.area()  # self = rect
```

```rust
// Rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

let rect = Rectangle { width: 30, height: 50 };
rect.area();  // self = rect
```

---

## self の3つの形

### 1. `&self` - 不変借用（最も一般的）

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        //    ^^^^^ 読み取り専用
        self.width * self.height
    }
}

let rect = Rectangle { width: 30, height: 50 };
let a = rect.area();

println!("{}", rect.width);  // ✅ rectはまだ使える
```

**用途:** 読み取るだけ

---

### 2. `&mut self` - 可変借用

```rust
impl Rectangle {
    fn set_width(&mut self, width: u32) {
        //       ^^^^^^^^ 変更可能
        self.width = width;
    }

    fn double(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
}

let mut rect = Rectangle { width: 30, height: 50 };
//  ^^^ mut が必要

rect.set_width(40);
println!("{}", rect.width);  // 40
```

**用途:** インスタンスを変更する

---

### 3. `self` - 所有権を奪う（稀）

```rust
impl Rectangle {
    fn into_square(self) -> Rectangle {
        //         ^^^^ 所有権を取る
        let size = self.width.max(self.height);
        Rectangle {
            width: size,
            height: size,
        }
    }
}

let rect = Rectangle { width: 30, height: 50 };
let square = rect.into_square();  // rectの所有権がメソッドに移動

// println!("{}", rect.width);  // ❌ エラー！rectは使えない
```

**用途:** 変換・消費

---

## 自動参照・参照外し

### C/C++ の問題

```c
// C/C++
Rectangle* rect = new Rectangle();

rect->area();     // ポインタ
(*rect).area();   // 参照外し

Rectangle rect2;
rect2.area();     // 実体
```

**問題:** `.` と `->` を使い分けないといけない

---

### Rust の解決策

```rust
let rect = Rectangle { width: 30, height: 50 };

// 全部同じ意味！Rustが自動で判断
rect.area();        // ← これが一番自然
(&rect).area();     // Rustが自動でやってくれる
```

**メリット:** 常に `.` を使えばいい

---

## メソッドのメリット

### 1. 整理整頓

```rust
// ❌ 関数だと散らばる
fn area(rect: &Rectangle) -> u32 { }
fn perimeter(rect: &Rectangle) -> u32 { }
fn is_square(rect: &Rectangle) -> bool { }

// ✅ メソッドだとまとまる
impl Rectangle {
    fn area(&self) -> u32 { }
    fn perimeter(&self) -> u32 { }
    fn is_square(&self) -> bool { }
}
```

### 2. 発見しやすい

```rust
let rect = Rectangle { width: 30, height: 50 };

// ドット補完で全メソッドが見える
rect.  // ← IDE が area(), perimeter() などを提案
```

### 3. 自然な読み方

```rust
// 関数
let area = area(&rect);  // 「area関数にrectを渡す」

// メソッド
let area = rect.area();  // 「rectのareaを取得」（自然！）
```

---

## 複数のパラメータ

### メソッドは他の引数も取れる

```rust
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

let rect1 = Rectangle { width: 30, height: 50 };
let rect2 = Rectangle { width: 10, height: 40 };
let rect3 = Rectangle { width: 60, height: 45 };

println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));  // true
println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));  // false
```

---

## 関連関数（Associated Functions）

### self を取らない関数

```rust
impl Rectangle {
    // 関連関数（コンストラクタ）
    fn new(width: u32, height: u32) -> Rectangle {
        //  ^^^^ selfがない
        Rectangle { width, height }
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// 呼び出しは :: を使う
let rect = Rectangle::new(30, 50);
let sq = Rectangle::square(20);
```

**用途:**
- コンストラクタ
- ファクトリメソッド

---

## 複数の impl ブロック

### 分けることも可能

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

// 計算メソッド
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

// 比較メソッド
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// コンストラクタ
impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
}
```

**理由:**
- コードの整理
- 機能ごとにグループ化

---

## struct と impl の関係（道具箱の例え）

```
┌─────────────────────────────┐
│ struct Rectangle            │  ← 道具箱の設計図
│ ┌─────────────────────────┐ │
│ │ width: u32              │ │  ← データ（仕切り）
│ │ height: u32             │ │
│ └─────────────────────────┘ │
└─────────────────────────────┘

        ↓ impl で機能を追加

┌─────────────────────────────┐
│ impl Rectangle              │  ← 道具箱に入れる道具
│ ┌─────────────────────────┐ │
│ │ fn area(&self)          │ │  ← 道具1
│ │ fn perimeter(&self)     │ │  ← 道具2
│ │ fn can_hold(&self)      │ │  ← 道具3
│ │ fn new(...)             │ │  ← 道具箱を作る道具
│ └─────────────────────────┘ │
└─────────────────────────────┘

        ↓ インスタンス化

┌─────────────────────────────┐
│ let rect = Rectangle {      │  ← 実際の道具箱
│   width: 30,                │
│   height: 50,               │
│ }                           │
└─────────────────────────────┘

rect.area() を使える！
```

---

## 実践例

### 完全な Rectangle 実装

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // コンストラクタ（関連関数）
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }

    // メソッド（計算）
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    // メソッド（判定）
    fn is_square(&self) -> bool {
        self.width == self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // メソッド（変更）
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn double(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }

    // メソッド（変換・消費）
    fn into_square(self) -> Rectangle {
        let size = self.width.max(self.height);
        Rectangle::square(size)
    }
}

fn main() {
    // 作成
    let rect = Rectangle::new(30, 50);
    let sq = Rectangle::square(20);

    // 計算
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());

    // 判定
    println!("Is square: {}", rect.is_square());
    println!("Can hold: {}", rect.can_hold(&sq));

    // 変更
    let mut mut_rect = Rectangle::new(10, 20);
    mut_rect.set_width(15);
    mut_rect.double();

    // 変換
    let square = rect.into_square();
    println!("{:?}", square);
}
```

---

## まとめ

### メソッドの重要ポイント

| 概念 | 説明 |
|---|---|
| **定義** | `impl Type { fn method(&self) }` |
| **self** | メソッドを呼んだインスタンス自身 |
| **&self** | 不変借用（読み取り専用） |
| **&mut self** | 可変借用（変更可能） |
| **self** | 所有権を奪う（変換・消費） |
| **関連関数** | `self` がない、`::`で呼ぶ |

### struct と impl の関係

```
struct = 道具箱の設計図（データ構造）
impl   = 道具箱に入れる道具（機能）
```

### 自動参照

```
C/C++: . と -> を使い分け
Rust:  常に . だけ（自動で判断）
```

### メソッドのメリット

```
1. コードの整理
2. 発見しやすさ
3. 自然な読み方
4. 自動参照の恩恵
```
