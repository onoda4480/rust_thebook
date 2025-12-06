# 第19章 Part 2: トレイトの高度な機能

## Send と Sync マーカートレイト（復習）

### Send トレイト
- **意味**: スレッド間で**所有権を転送**できる
- **重要**: "共有"ではなく"転送"！
- ほとんどの型が `Send` を実装している
- 例外: `Rc<T>` は `Send` ではない（参照カウントがスレッドセーフでない）

```rust
use std::thread;

let data = vec![1, 2, 3];
thread::spawn(move || {
    println!("{:?}", data);  // ✅ Vec<T> は Send なので所有権を転送できる
});
```

### Sync トレイト
- **意味**: スレッド間で**参照を共有**できる
- `&T` が `Send` なら、`T` は `Sync`
- 例: `i32` は `Sync`（`&i32` を複数スレッドで安全に共有できる）
- 例外: `RefCell<T>` は `Sync` ではない（内部可変性がスレッドセーフでない）

```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3]);
let data_clone = Arc::clone(&data);

thread::spawn(move || {
    println!("{:?}", data_clone);  // ✅ Arc は参照を安全に共有できる
});
```

### まとめ
| トレイト | 意味 | 例 |
|---------|------|-----|
| `Send` | 所有権を転送できる | `Vec<T>`, `String` |
| `Sync` | 参照を共有できる | `i32`, `Arc<T>` |

---

## 関連型 vs ジェネリクス

### 関連型（Associated Types）

**特徴**: 1つの型につき**1つの実装のみ**可能

```rust
pub trait Iterator {
    type Item;  // 関連型

    fn next(&mut self) -> Option<Self::Item>;
}
```

**実装例**:
```rust
struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;  // Counter は u32 を返す Iterator として実装

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        Some(self.count)
    }
}
```

**ポイント**:
- `Counter` に対して `Iterator` は**1つだけ**実装できる
- `type Item = u32` と決めたら、それ以外の型を返す `Iterator` は実装不可
- 呼び出し時に型注釈不要: `counter.next()` だけでOK

---

### ジェネリクス（Generic Type Parameters）

**特徴**: 1つの型に**複数の実装**が可能

```rust
pub trait From<T> {
    fn from(value: T) -> Self;
}
```

**実装例**:
```rust
struct MyType;

impl From<String> for MyType {
    fn from(s: String) -> Self {
        // String からの変換
        MyType
    }
}

impl From<i32> for MyType {
    fn from(n: i32) -> Self {
        // i32 からの変換
        MyType
    }
}

impl From<&str> for MyType {
    fn from(s: &str) -> Self {
        // &str からの変換
        MyType
    }
}
```

**ポイント**:
- `MyType` に対して `From<String>`, `From<i32>`, `From<&str>` など**複数実装可能**
- 呼び出し時に型を指定することも: `MyType::from::<String>(value)`

---

### 使い分け

| 観点 | 関連型 | ジェネリクス |
|------|--------|------------|
| 実装数 | 1つの型に1つだけ | 1つの型に複数可能 |
| 使用例 | `Iterator::Item` | `From<T>`, `Add<Rhs>` |
| 型推論 | 自動的に決まる | 明示が必要な場合も |
| 目的 | 型に固有の関連情報 | 複数の型との関係 |

### なぜこの違いが重要？

**関連型の場合**:
```rust
let mut counter = Counter::new();
let item = counter.next();  // Item の型は自動的に u32 と決まる
```

**もしジェネリクスだったら**:
```rust
let mut counter = Counter::new();
let item = counter.next::<u32>();  // 毎回型を指定する必要がある
```

関連型を使うことで、コードがシンプルになり、型の曖昧性がなくなる！
