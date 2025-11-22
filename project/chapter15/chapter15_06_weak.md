# Chapter 15-6: Weak<T> - 循環参照を防ぐ弱参照

## 循環参照の問題

**強参照（Rc）同士が循環すると、メモリリークが発生**

```rust
// a → b → a の循環
let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
*a.tail().unwrap().borrow_mut() = Rc::clone(&b);
// strong_count が 0 にならない！
```

---

## メモリリークとは？

**使い終わったメモリが解放されず、使える場所が減る現象**

```
普通の場合:
  使う → 使い終わる → 片付ける（メモリ解放）✅

循環参照の場合:
  使う → 使い終わる → 片付けられない（お互いが掴んでる）❌
  → メモリがどんどん減る
```

---

## Weak<T> とは？

**所有権を持たない弱参照**

```rust
let strong = Rc::new(5);
let weak = Rc::downgrade(&strong);  // Weak<i32>
```

---

## Rc と Weak の違い

| 項目 | Rc（強参照） | Weak（弱参照） |
|------|------------|--------------|
| **所有権** | ✅ ある | ❌ ない |
| **参照カウント** | strong_count | weak_count |
| **値の保持** | ✅ 保持する | ❌ 保持しない |
| **メモリ解放** | strong_count = 0 で解放 | weak_count は無関係 |
| **アクセス方法** | 直接 | `upgrade()` → `Option<Rc<T>>` |

---

## Weak の作成とアクセス

### 作成
```rust
let strong = Rc::new(5);
let weak = Rc::downgrade(&strong);  // Rc → Weak
```

### アクセス
```rust
match weak.upgrade() {
    Some(rc) => println!("値: {}", rc),  // 値が存在
    None => println!("既に解放済み"),      // 値は解放された
}
```

---

## 例え話

### 強参照 = 図書館の本を借りている人
- 誰か1人でも借りていたら、本は図書館に残る
- 全員が返却したら（strong_count = 0）、本を廃棄できる

### 弱参照 = 本の予約リスト
- 予約している人がいても、本を廃棄することはある
- 廃棄されてたら「もうないです」と言われる（None）
- 残ってたら借りられる（Some）

---

## 循環参照の解決策

### 問題：強参照の循環
```
a (Rc) ──強──→ b (Rc)
 ↑              ↓
 └─────強──────┘

両方とも strong_count ≥ 1
→ 永遠に解放されない！
```

### 解決：弱参照を使う
```
a (Rc) ──強──→ b (Rc)
 ↑              ↓
 └─────弱──────┘

a の strong_count = 1
→ a がスコープを出ると解放される！
```

---

## 親子関係の実装

### Node 構造体
```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,      // 親への弱参照
    children: RefCell<Vec<Rc<Node>>>, // 子への強参照
}
```

### なぜこの設計？
- **親 → 子**: 強参照（親が子を所有）
- **子 → 親**: 弱参照（子は親を参照するだけ）

---

## 具体例：ツリー構造

```rust
let leaf = Rc::new(Node {
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
});

let branch = Rc::new(Node {
    value: 5,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![Rc::clone(&leaf)]),
});

*leaf.parent.borrow_mut() = Rc::downgrade(&branch);
```

---

## 参照カウントの変化

```rust
// 1. leaf 作成
println!("leaf strong = {}, weak = {}",
    Rc::strong_count(&leaf),  // 1
    Rc::weak_count(&leaf));   // 0

{
    // 2. branch 作成 + 親子関係設定
    let branch = Rc::new(Node { ... });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("branch strong = {}, weak = {}",
        Rc::strong_count(&branch),  // 1
        Rc::weak_count(&branch));   // 1 ← leaf.parent が弱参照

    println!("leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),    // 2 ← branch.children が強参照
        Rc::weak_count(&leaf));     // 0
}
// 3. branch がスコープを出る

println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
// None （親は解放された）

println!("leaf strong = {}, weak = {}",
    Rc::strong_count(&leaf),  // 1 （元に戻る）
    Rc::weak_count(&leaf));   // 0
```

---

## メモリ解放のタイミング

```
branch (strong=1, weak=1)
  ↓ 強参照
leaf (strong=1)
  ↓ 弱参照
branch
```

**branch がスコープを出ると:**
1. branch の strong_count が 0 になる
2. **weak_count は無視される**
3. branch が解放される
4. leaf の strong_count が 0 になる
5. leaf も解放される

---

## 循環はあるが安全

### 双方向参照 ≠ メモリリーク
```
✅ 双方向だが安全:
   parent ──強──→ child
      ↑            ↓
      └────弱──────┘

❌ 双方向でメモリリーク:
   a ──強──→ b
   ↑         ↓
   └───強────┘
```

**重要:** 強参照の循環が問題、弱参照を含む循環は安全

---

## デバッグ表示

### Weak は辿らない
```rust
println!("{:?}", leaf);
// Node { value: 3, parent: (Weak), children: [] }
//                         ^^^^^^
//                         循環を辿らない
```

**利点:** スタックオーバーフローを防ぐ

---

## 使い分け

| 関係 | 参照の種類 | 理由 |
|------|----------|------|
| 親 → 子 | 強参照（Rc） | 親が子を所有 |
| 子 → 親 | 弱参照（Weak） | 子は親を参照するだけ |
| 兄弟 → 兄弟 | 弱参照（Weak） | お互いを所有しない |

---

## Python との対応

| 概念 | Rust | Python |
|------|------|--------|
| **循環参照** | 手動で防ぐ（Weak） | GCが検出 |
| **弱参照** | `Weak<T>` | `weakref` |
| **メモリリーク** | 循環参照で発生 | 通常は発生しない |

**重要:** Pythonは循環参照を自動検出、Rustは手動で対処

---

## まとめ

| 項目 | 説明 |
|------|------|
| **Weak<T>** | 所有権を持たない弱参照 |
| **循環参照** | 強参照同士の循環でメモリリーク |
| **解決策** | 片方を Weak にする |
| **upgrade()** | `Option<Rc<T>>` を返す |
| **weak_count** | メモリ解放に影響しない |
| **用途** | 親子関係、双方向参照 |

**重要:** 循環参照を作りたいが、メモリリークは避けたい時に Weak を使う！
