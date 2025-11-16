# 第10章 Q&A (2/3): トレイト

## Q1: トレイトとは何？なぜ必要？

### A: 型が持つべき振る舞いを定義する仕組み（インターフェース）。異なる型に共通の機能を持たせるために必要。

**トレイトなしの問題:**

```rust
struct Tweet { ... }
impl Tweet {
    fn tweet_summary(&self) -> String { ... }
}

struct Article { ... }
impl Article {
    fn article_summary(&self) -> String { ... }
}

// ❌ メソッド名がバラバラ
tweet.tweet_summary();
article.article_summary();
```

---

**トレイトありの解決:**

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for Tweet {
    fn summarize(&self) -> String { ... }
}

impl Summary for Article {
    fn summarize(&self) -> String { ... }
}

// ✅ 統一されたインターフェース
tweet.summarize();
article.summarize();
```

---

**さらに重要: ジェネリック関数で使える**

```rust
// Summary を実装している任意の型を受け入れる
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

notify(&tweet);    // ✅ OK
notify(&article);  // ✅ OK
```

---

## Q2: デフォルト実装とは何？どういう時に使う？

### A: トレイトのメソッドにあらかじめ実装を用意すること。実装者が省略できるようにするため。

**デフォルト実装なし:**

```rust
pub trait Summary {
    fn summarize(&self) -> String;  // 実装必須
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}
```

---

**デフォルト実装あり:**

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")  // デフォルト実装
    }
}

// デフォルトを使う
impl Summary for NewsArticle {}

let article = NewsArticle { ... };
println!("{}", article.summarize());  // "(Read more...)"

// デフォルトを上書き
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

let tweet = Tweet { ... };
println!("{}", tweet.summarize());  // "user: hello"
```

---

**デフォルト実装から他のメソッドを呼ぶ（テンプレートメソッドパターン）:**

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;  // 実装必須

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
        // デフォルト実装が他のメソッドを呼ぶ
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // summarize は実装不要（デフォルトを使う）
}

let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("hello"),
};
println!("{}", tweet.summarize());
// "(Read more from @horse_ebooks...)"
```

**メリット:** 実装者は最小限のコードを書けばいい

---

## Q3: `impl Trait` 構文とトレイト境界構文の違いは？

### A: 意味は同じだが、書き方と使いどころが異なる

**impl Trait 構文（簡潔）:**

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

**トレイト境界構文（明示的）:**

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

---

**違いが出るケース:**

**複数の引数（異なる型でもいい）:**

```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
    // item1 と item2 は異なる型でもいい
}

notify(&tweet, &article);  // ✅ OK
```

**複数の引数（同じ型でなければならない）:**

```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {
    // item1 と item2 は同じ型でなければならない
}

notify(&tweet, &article);  // ❌ エラー（異なる型）
notify(&tweet1, &tweet2);  // ✅ OK（同じ型）
```

---

**複雑な場合は where 句:**

```rust
// 読みにくい
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { ... }

// 読みやすい
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    ...
}
```

---

## Q4: 孤児のルール（Orphan Rule）とは何？なぜ必要？

### A: トレイトまたは型の少なくとも1つが自分のクレートで定義されている必要があるというルール。競合を防ぐために必要。

**ルール:**

```
トレイトまたは型の少なくとも1つが
自分のクレートで定義されている必要がある
```

---

**✅ OK な例:**

```rust
// 1. 自分の型 + 外部トレイト
impl Display for Tweet { ... }  // ✅ OK

// 2. 外部の型 + 自分のトレイト
impl Summary for Vec<T> { ... }  // ✅ OK

// 3. 自分の型 + 自分のトレイト
impl Summary for Tweet { ... }   // ✅ OK
```

---

**❌ NG な例:**

```rust
// 外部の型 + 外部トレイト
impl Display for Vec<T> { ... }  // ❌ エラー
```

**エラー:**
```
error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
```

---

**なぜ必要？**

ルールがない場合の問題:

```rust
// あなたのクレート
impl Display for Vec<i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "My custom display")
    }
}

// 他の人のクレート
impl Display for Vec<i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Their custom display")
    }
}

// ❌ 競合！どちらを使う？
let v = vec![1, 2, 3];
println!("{}", v);  // どっちの実装？
```

**ルールがあれば:** 競合が起きない

---

## Q5: `impl Trait` を戻り値として使う場合の制限は？

### A: 1種類の型しか返せない。異なる型を返すことはできない。

**✅ OK な例（1種類の型）:**

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
    }
}
```

---

**❌ NG な例（異なる型を返そうとする）:**

```rust
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {  // NewsArticle を返す
            headline: String::from("Penguins win!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
        }
    } else {
        Tweet {  // Tweet を返す
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
        }
    }
}
```

**エラー:**
```
error: `if` and `else` have incompatible types
```

---

**理由:** `impl Trait` はコンパイル時に具体的な型に置き換えられる

コンパイラの視点:
```rust
fn returns_summarizable() -> impl Summary
// ↓ コンパイル時に
fn returns_summarizable() -> Tweet  // または NewsArticle
```

異なる型を返す場合は、**トレイトオブジェクト**を使う必要がある（第17章で学ぶ）

---

## まとめ

### トレイトの重要ポイント

```
✅ 目的: 異なる型に共通の機能を持たせる
✅ デフォルト実装: 実装を省略できる
✅ impl Trait: 簡潔、トレイト境界: 明示的
✅ 孤児のルール: 競合を防ぐ
✅ impl Trait 戻り値: 1種類の型のみ
```

---

### トレイトの利点

```
✅ 統一されたインターフェース
✅ ジェネリック関数で使える
✅ デフォルト実装でコード重複を削減
✅ 条件付き実装で柔軟性を持たせる
```
