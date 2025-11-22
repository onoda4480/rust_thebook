# Chapter 15-5: RefCell<T> - 内部可変性パターン

## RefCell<T> とは？

**実行時に借用規則をチェックするスマートポインタ**

```rust
let x = RefCell::new(5);
*x.borrow_mut() = 10;  // 実行時に可変借用をチェック
```

---

## 内部可変性とは？

**外側は不変でも、内側を可変化できるパターン**

```rust
let x = RefCell::new(5);  // x は不変
*x.borrow_mut() = 10;     // でも中身は変更できる
```

---

## RefCell<T> の特徴

| 特徴 | 説明 |
|------|------|
| **実行時チェック** | 借用規則を実行時に検証 |
| **パニック** | 規則違反でパニック |
| **内部可変性** | 不変参照経由で変更可能 |
| **シングルスレッド** | マルチスレッド不可 |

---

## 借用規則の比較

### 通常の借用（コンパイル時）
```rust
let mut x = 5;
let y = &x;
let z = &mut x;  // ❌ コンパイルエラー！
```

### RefCell（実行時）
```rust
let x = RefCell::new(5);
let y = x.borrow();
let z = x.borrow_mut();  // ⚠️  実行時パニック！
```

---

## RefCell<T> のメソッド

### borrow() - 不変借用
```rust
let x = RefCell::new(5);
let y = x.borrow();  // Ref<i32>
println!("{}", *y);
```

### borrow_mut() - 可変借用
```rust
let x = RefCell::new(5);
let mut y = x.borrow_mut();  // RefMut<i32>
*y = 10;
```

---

## 借用規則の実行時チェック

### OK: 複数の不変借用
```rust
let x = RefCell::new(5);
let a = x.borrow();
let b = x.borrow();  // ✅ OK
println!("{} {}", a, b);
```

### NG: 不変と可変の同時借用
```rust
let x = RefCell::new(5);
let a = x.borrow();
let b = x.borrow_mut();  // ⚠️ パニック！
```

### NG: 複数の可変借用
```rust
let x = RefCell::new(5);
let a = x.borrow_mut();
let b = x.borrow_mut();  // ⚠️ パニック！
```

---

## 実用例：モックオブジェクト

### Messenger トレイト
```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}
```

### 問題：send は &self なのに記録したい
```rust
struct MockMessenger {
    sent_messages: Vec<String>,  // これを変更したい
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        self.sent_messages.push(String::from(message));
        // ❌ &self では push できない
    }
}
```

### 解決：RefCell を使う
```rust
struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,  // RefCell で包む
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        self.sent_messages.borrow_mut().push(String::from(message));
        // ✅ OK!
    }
}
```

---

## Rc<RefCell<T>> vs RefCell<Rc<T>>

### Rc<RefCell<T>>: 共有された値の中身を変更
```rust
let value = Rc::new(RefCell::new(5));
let a = Rc::clone(&value);
let b = Rc::clone(&value);

*a.borrow_mut() = 10;
println!("{}", b.borrow());  // 10（変更が見える）
```

**用途:** 複数の所有者で同じ値を変更

---

### RefCell<Rc<T>>: 参照先を差し替え
```rust
let a = Rc::new(String::from("A"));
let b = Rc::new(String::from("B"));
let pointer = RefCell::new(Rc::clone(&a));

*pointer.borrow_mut() = Rc::clone(&b);  // ポインタを差し替え
```

**用途:** リンクリストのノードを繋ぎ変える

---

## 比較表

| パターン | 何を変更するか | 用途 |
|---------|--------------|------|
| `Rc<RefCell<T>>` | 共有された値の中身 | 共有カウンター、状態管理 |
| `RefCell<Rc<T>>` | 参照先（ポインタ） | グラフ構造、リンクの変更 |

---

## Box/Rc/RefCell の使い分け

| 型 | 所有者 | 可変性 | チェック |
|----|--------|--------|---------|
| `Box<T>` | 単一 | 可変 | コンパイル時 |
| `Rc<T>` | 複数 | 不変のみ | コンパイル時 |
| `RefCell<T>` | 単一 | 可変 | 実行時 |
| `Rc<RefCell<T>>` | 複数 | 可変 | 実行時 |

---

## パフォーマンス

### コンパイル時チェック（通常の借用）
- オーバーヘッドなし
- 安全性が保証される

### 実行時チェック（RefCell）
- わずかなランタイムオーバーヘッド
- パニックのリスク
- 柔軟性が高い

---

## Python との対応

| 概念 | Rust | Python |
|------|------|--------|
| **可変性** | 明示的（`mut`、`RefCell`） | すべて可変 |
| **チェック** | コンパイル時 or 実行時 | 実行時のみ |
| **パニック** | 借用規則違反 | AttributeError |

**重要:** Pythonはすべて実行時、Rustは選択可能

---

## まとめ

| 項目 | 説明 |
|------|------|
| **内部可変性** | 不変参照経由で変更可能 |
| **実行時チェック** | 借用規則を実行時に検証 |
| **borrow()** | 不変借用を取得 |
| **borrow_mut()** | 可変借用を取得 |
| **パニック** | 規則違反で実行時エラー |
| **用途** | コンパイル時に安全性を証明できない場合 |

**重要:** 安全性をランタイムに委ねることで、柔軟性を得る！
