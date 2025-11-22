# Chapter 15-4: Rc<T> - 参照カウント型スマートポインタ

## Rc<T> とは？

**複数の所有者を可能にするスマートポインタ**

```rust
use std::rc::Rc;

let a = Rc::new(5);
let b = Rc::clone(&a);  // 所有者が2人
```

---

## なぜ Rc<T> が必要か？

### 問題：単一所有権の制限
```rust
let a = Box::new(5);
let b = a;  // a から所有権が移動
// a はもう使えない ❌
```

### 解決：Rc<T> で複数所有
```rust
let a = Rc::new(5);
let b = Rc::clone(&a);  // 参照カウント++
let c = Rc::clone(&a);  // 参照カウント++
// a, b, c すべて使える ✅
```

---

## Rc<T> の特徴

| 特徴 | 説明 |
|------|------|
| **複数所有者** | 複数の変数が同じデータを所有 |
| **参照カウント** | 所有者の数を追跡 |
| **自動解放** | カウントが0になると解放 |
| **不変のみ** | 共有された読み取り専用アクセス |
| **シングルスレッド** | マルチスレッド不可 |

---

## 参照カウントの仕組み

```rust
let a = Rc::new(5);
println!("count = {}", Rc::strong_count(&a));  // 1

{
    let b = Rc::clone(&a);
    println!("count = {}", Rc::strong_count(&a));  // 2
}
// b がドロップ

println!("count = {}", Rc::strong_count(&a));  // 1
```

---

## Rc::clone vs Clone::clone

### Rc::clone（推奨）
```rust
let b = Rc::clone(&a);
// - 参照カウントを増やすだけ
// - データのディープコピーはしない
// - 高速！
```

### データの Clone
```rust
let b = (*a).clone();
// - データをディープコピー
// - 重い処理
```

**重要:** `Rc::clone` はポインタのコピーのみ

---

## Cons List での使用例

### 問題：Box では共有不可
```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

let a = Cons(5, Box::new(Nil));
let b = Cons(3, Box::new(a));
let c = Cons(4, Box::new(a));  // ❌ a は移動済み
```

### 解決：Rc で共有
```rust
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

let a = Rc::new(Cons(5, Rc::new(Nil)));
let b = Cons(3, Rc::clone(&a));
let c = Cons(4, Rc::clone(&a));  // ✅ OK!
```

---

## メモリレイアウト

### Rc<T> の構造
```
┌──────────────┐
│ strong_count │  ← 参照カウント
│ weak_count   │  ← 弱参照カウント
│ data: T      │  ← 実際のデータ
└──────────────┘
     ↑  ↑  ↑
     │  │  │
     a  b  c  ← 各変数がポインタを持つ
```

---

## 参照カウントの追跡例

```rust
let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
println!("count after creating a = {}", Rc::strong_count(&a));
// 出力: count after creating a = 1

let b = Cons(3, Rc::clone(&a));
println!("count after creating b = {}", Rc::strong_count(&a));
// 出力: count after creating b = 2

{
    let c = Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));
    // 出力: count after creating c = 3
}

println!("count after c goes out of scope = {}", Rc::strong_count(&a));
// 出力: count after c goes out of scope = 2
```

---

## Rc<T> の制限

### 1. 不変のみ
```rust
let a = Rc::new(5);
*a = 10;  // ❌ コンパイルエラー！
// Rc は不変の共有参照のみ
```

### 2. シングルスレッド専用
```rust
// マルチスレッドでは Arc<T> を使う
use std::sync::Arc;
let a = Arc::new(5);
```

---

## 用途

### グラフ構造
```rust
struct Node {
    value: i32,
    neighbors: Vec<Rc<Node>>,
}

// 複数のノードから同じノードを参照
```

### ツリー構造
```rust
struct TreeNode {
    value: i32,
    children: Vec<Rc<TreeNode>>,
}

// 複数の親から同じ子ノードを参照
```

---

## Python との対応

| 概念 | Rust | Python |
|------|------|--------|
| **参照カウント** | `Rc<T>` | すべてのオブジェクト |
| **カウント確認** | `Rc::strong_count` | `sys.getrefcount` |
| **クローン** | `Rc::clone` | 自動 |
| **スレッド安全** | `Arc<T>` | GIL |

**重要:** Pythonは常に参照カウント、Rustは明示的

---

## まとめ

| 項目 | 説明 |
|------|------|
| **用途** | 複数の所有者が必要な時 |
| **参照カウント** | strong_count で管理 |
| **解放** | count = 0 で自動解放 |
| **制限** | 不変のみ、シングルスレッド |
| **クローン** | `Rc::clone` は軽量 |
| **代替** | マルチスレッドは `Arc<T>` |

**重要:** データを共有したいが、可変性は不要な場合に使う！
