# Chapter 15-2: Deref トレイト - 参照外しのカスタマイズ

## Deref トレイトとは？

**`*` 演算子の動作をカスタマイズするトレイト**

```rust
let x = 5;
let y = &x;
assert_eq!(5, *y);  // Deref で参照外し
```

---

## Deref トレイトの定義

```rust
pub trait Deref {
    type Target: ?Sized;
    fn deref(&self) -> &Self::Target;
}
```

**重要:** `&T` を返す（所有権は渡さない）

---

## Box<T> での Deref

### 通常の参照
```rust
let x = 5;
let y = &x;
assert_eq!(5, *y);
```

### Box での参照
```rust
let x = 5;
let y = Box::new(x);
assert_eq!(5, *y);  // Deref のおかげ
```

**動作:** `*y` → `*(y.deref())`

---

## 自作の MyBox<T>

### 実装例
```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0  // タプル構造体の最初の要素への参照
    }
}
```

### 使用例
```rust
let x = 5;
let y = MyBox::new(x);
assert_eq!(5, *y);  // *(y.deref()) と同じ
```

---

## 参照外し強制（Deref Coercion）

### 概要
**関数やメソッドの引数で自動的に型変換**

```rust
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

let m = MyBox::new(String::from("Rust"));
hello(&m);  // &MyBox<String> → &String → &str
```

### 変換の流れ
```
&MyBox<String>
   ↓ Deref
&String
   ↓ Deref
&str
```

---

## Deref 強制なしの場合

```rust
let m = MyBox::new(String::from("Rust"));
hello(&(*m)[..]);  // 😱 読みにくい！
//     ^^^  ^^
//      |    スライス
//      参照外し
```

**Deref 強制があれば:**
```rust
hello(&m);  // 😊 シンプル！
```

---

## Deref 強制の3パターン

| パターン | 変換 | 例 |
|---------|------|-----|
| **不変 → 不変** | `&T` → `&U` | `&MyBox<T>` → `&T` |
| **可変 → 可変** | `&mut T` → `&mut U` | `&mut MyBox<T>` → `&mut T` |
| **可変 → 不変** | `&mut T` → `&U` | `&mut MyBox<T>` → `&T` |

**注意:** 不変 → 可変 は不可能（借用規則）

---

## 実用例

### String と &str
```rust
fn print_str(s: &str) {
    println!("{}", s);
}

let my_string = String::from("hello");
print_str(&my_string);  // &String → &str (自動変換)
```

### Vec と スライス
```rust
fn print_slice(s: &[i32]) {
    println!("{:?}", s);
}

let my_vec = vec![1, 2, 3];
print_slice(&my_vec);  // &Vec<i32> → &[i32] (自動変換)
```

---

## メモリ図

### Deref の動作
```
y: MyBox<String>
   ┌─────────┐
   │ String  │───> "Rust" (ヒープ)
   └─────────┘

*y → y.deref() → &String → &str
```

---

## まとめ

| 項目 | 説明 |
|------|------|
| **Deref トレイト** | `*` 演算子をカスタマイズ |
| **deref メソッド** | `&T` を返す |
| **参照外し強制** | 関数の引数で自動型変換 |
| **利点** | コードがシンプルになる |
| **制限** | 不変 → 可変 は不可 |

**重要:** スマートポインタを普通の参照のように扱える！
