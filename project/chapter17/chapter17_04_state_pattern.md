# Chapter 17-4: ステートパターン

## ステートパターンとは？

**状態によって振る舞いが変わるパターン**

```
Draft → PendingReview → Published
草稿     査読待ち         公開
```

---

## 実装

### Post 構造体

```rust
pub struct Post {
    state: Option<Box<dyn State>>,  // 状態
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}
```

---

### State トレイト

```rust
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""  // デフォルト実装
    }
}
```

---

### 各状態の実装

#### Draft（草稿）

```rust
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})  // → 査読待ち
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self  // 何もしない（Draft のまま）
    }
}
```

#### PendingReview（査読待ち）

```rust
struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self  // 何もしない
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})  // → 公開
    }
}
```

#### Published（公開）

```rust
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self  // 何もしない
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self  // 何もしない
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content  // 内容を返す
    }
}
```

---

## take() を使う理由

### 問題：直接書けない

```rust
// ❌ これはできない
self.state = self.state.request_review();
//           ^^^^^^^^^^
//           所有権を奪えない
```

### 解決策：take()

```rust
// ✅ take() を使う
if let Some(s) = self.state.take() {
    //              ^^^^
    //              1. self.state を None に
    //              2. 値を s に取り出す

    self.state = Some(s.request_review())
    //                3. 新しい状態を設定
}
```

---

## unwrap() を使っても安全な理由

```rust
pub fn content(&self) -> &str {
    self.state.as_ref().unwrap().content(self)
    //                 ^^^^^^^
    //                 絶対にパニックしない
}
```

**理由:**
1. `new()` で必ず `Some(Draft)` で初期化
2. `take()` の直後に必ず新しい `Some` を設定
3. `content()` は `&self`（変更できない）
4. → `state` は必ず `Some`

---

## Rust 的なアプローチ

### 型システムを使う

```rust
pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

pub struct Post {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {  // DraftPost を返す
        DraftPost {
            content: String::new(),
        }
    }
}

impl DraftPost {
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
```

**利点:** コンパイル時にエラーを検出

---

## 比較

| 項目 | ステートパターン | 型システム |
|------|---------------|-----------|
| **カプセル化** | ✅ 良い | ❌ 弱い |
| **コンパイル時安全性** | ❌ 弱い | ✅ 強い |
| **間違いの検出** | 実行時 | コンパイル時 |
| **使いやすさ** | ✅ 簡潔 | △ 再代入必要 |

---

## まとめ

**ステートパターン:**
- 状態を内部で管理
- カプセル化されている
- でも実行時エラーの可能性

**型システムを使う:**
- 状態を型で表現
- コンパイル時にエラーを検出
- Rust らしいアプローチ

**結論:** Rust では型システムを活かす方が良い場合が多い
