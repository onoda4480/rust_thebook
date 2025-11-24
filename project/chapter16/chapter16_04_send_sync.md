# Chapter 16-4: Send と Sync トレイト

## マーカートレイトとは？

**メソッドを持たないトレイト**
- データを持たない
- メソッドを実装しない
- 型に「印」を付けるだけ

---

## 普通のトレイト vs マーカートレイト

### 普通のトレイト
```rust
trait Display {
    fn fmt(&self, f: &mut Formatter) -> Result;
    //  ^^^
    //  メソッドがある
}
```

### マーカートレイト
```rust
trait Send {
    // メソッドがない！
    // 「この型はスレッド間で送れますよ」という印だけ
}
```

---

## Send トレイト

### 定義

> **所有権をスレッド間で転送できる型**

```rust
pub unsafe auto trait Send { }
//     ^^^^^^ ^^^^
//       |     自動実装される
//       unsafe（手動実装は危険）
```

---

### ✅ Send な型（スレッド間で送れる）

```rust
// i32
let x = 5;
thread::spawn(move || {
    println!("{}", x);  // ✅ OK
});

// String
let s = String::from("hello");
thread::spawn(move || {
    println!("{}", s);  // ✅ OK
});

// Vec<T>
let v = vec![1, 2, 3];
thread::spawn(move || {
    println!("{:?}", v);  // ✅ OK
});
```

---

### ❌ Send でない型（スレッド間で送れない）

```rust
use std::rc::Rc;

let x = Rc::new(5);
thread::spawn(move || {
    println!("{}", x);  // ❌ エラー！
});
// エラー: `Rc<i32>` cannot be sent between threads safely
```

**理由:** Rc の参照カウントがスレッド安全でない

---

### 図解：Send

```
スレッド1                スレッド2
┌─────────┐            ┌─────────┐
│ data    │──Send────> │ data    │
└─────────┘            └─────────┘
    ↓                      ↑
所有権が移動           所有権を受け取る

✅ Send な型：移動できる
❌ Send でない型：移動できない
```

---

## Sync トレイト

### 定義

> **複数のスレッドから参照を共有できる型**

```rust
pub unsafe auto trait Sync { }
```

**言い換えると:**
- `&T` が `Send` なら、`T` は `Sync`
- 複数のスレッドから `&T` で安全にアクセスできる

---

### ✅ Sync な型（参照を共有できる）

```rust
// i32
let x = 5;
// 複数のスレッドから &i32 でアクセスできる
```

---

### ❌ Sync でない型（参照を共有できない）

```rust
use std::cell::RefCell;

let x = RefCell::new(5);
let y = &x;

thread::spawn(move || {
    println!("{:?}", y);  // ❌ エラー！
});
// エラー: `RefCell<i32>` cannot be shared between threads safely
```

**理由:** RefCell の借用チェックがスレッド安全でない

---

### 図解：Sync

```
        data
         ↑
    ┌────┴────┐
    │    │    │
スレッド1 │ スレッド2
    &T   │   &T
         │
    スレッド3
         &T

✅ Sync な型：複数のスレッドから &T でアクセスできる
❌ Sync でない型：複数のスレッドから &T でアクセスできない
```

---

## Send vs Sync の違い

| トレイト | 意味 | 例 |
|---------|------|-----|
| **Send** | **所有権**をスレッド間で転送できる | `String`, `Vec<T>` |
| **Sync** | **参照**をスレッド間で共有できる | `i32`, `&str` |

---

## 具体例で比較

### Send: 所有権の移動
```rust
let s = String::from("hello");
thread::spawn(move || {
    // s の所有権を受け取る
    println!("{}", s);
});
// s はもう使えない
```

### Sync: 参照の共有
```rust
let s = String::from("hello");
// &String を複数のスレッドで共有するには Arc が必要
```

---

## よくある型の Send/Sync

| 型 | Send | Sync | 理由 |
|----|------|------|------|
| `i32` | ✅ | ✅ | 単純な値 |
| `String` | ✅ | ✅ | 所有権がある |
| `Vec<T>` | ✅* | ✅* | T に依存 |
| **Rc<T>** | ❌ | ❌ | 非スレッド安全 |
| **Arc<T>** | ✅* | ✅* | T に依存 |
| `RefCell<T>` | ✅* | ❌ | 借用チェックが非スレッド安全 |
| `Mutex<T>` | ✅* | ✅* | T に依存 |

`*` = T が Send/Sync なら実装される

---

## 自動実装

### ほとんどの型は自動で実装される

```rust
struct MyStruct {
    x: i32,
    y: String,
}
// i32 と String は Send/Sync
// → MyStruct も自動的に Send/Sync
```

---

### 例外

```rust
struct MyStruct {
    x: Rc<i32>,  // Rc は Send でない
}
// → MyStruct も Send でない
```

---

## 実用例

### Rc<T>: Send も Sync も実装していない

```rust
let x = Rc::new(5);

// ❌ Send でない
thread::spawn(move || {
    println!("{}", x);  // エラー
});

// ❌ Sync でない
let y = &x;
thread::spawn(move || {
    println!("{}", y);  // エラー
});
```

---

### Arc<T>: Send も Sync も実装している

```rust
let x = Arc::new(5);

// ✅ Send
let x1 = Arc::clone(&x);
thread::spawn(move || {
    println!("{}", x1);  // OK
});

// ✅ Sync（Arc 経由）
let x2 = Arc::clone(&x);
thread::spawn(move || {
    println!("{}", x2);  // OK
});
```

---

## 通常は書かない

### 自動実装される

```rust
// こんなことは書かない
impl Send for MyStruct { }  // 自動実装される
```

### unsafe で手動実装（稀）

```rust
// 危険！特別な理由がない限りしない
unsafe impl Send for MyUnsafeType { }
```

---

## 実用上の知識

### エラーメッセージを理解する

```rust
let x = Rc::new(5);
thread::spawn(move || {
    println!("{}", x);
});
```

**エラー:**
```
`Rc<i32>` cannot be sent between threads safely
         ^^^^^^^^^^^^^^^^^^^^^^^^
         Send トレイトを実装していない
```

**対処:**
```rust
let x = Arc::new(5);  // Arc に変える
```

---

## 型を選ぶ基準

| 状況 | 使う型 | 理由 |
|------|--------|------|
| シングルスレッド | `Rc<T>` | 軽い |
| マルチスレッド | `Arc<T>` | Send/Sync |
| 内部可変性（シングル） | `RefCell<T>` | 実行時チェック |
| 内部可変性（マルチ） | `Mutex<T>` | スレッド安全 |

---

## まとめ

### マーカートレイト

| 項目 | 説明 |
|------|------|
| **定義** | メソッドを持たないトレイト |
| **用途** | 型に「印」を付ける |
| **例** | Send, Sync, Copy |

### Send

| 項目 | 説明 |
|------|------|
| **意味** | 所有権をスレッド間で転送できる |
| **例** | String, Vec<T>, Arc<T> |
| **非例** | Rc<T> |

### Sync

| 項目 | 説明 |
|------|------|
| **意味** | 参照をスレッド間で共有できる |
| **条件** | &T が Send なら T は Sync |
| **例** | i32, String, Mutex<T> |
| **非例** | RefCell<T>, Rc<T> |

---

## 覚え方

```
Send = 送れる（所有権を転送）
Sync = 同期（参照を共有）

Rc  = どちらもダメ（シングルスレッド専用）
Arc = どちらもOK（マルチスレッド対応）
```

**重要:** 記述する必要はないが、知っておくべきこと！
- エラーメッセージを理解できる
- 適切な型を選べる
- 並行性の安全性を理解できる
