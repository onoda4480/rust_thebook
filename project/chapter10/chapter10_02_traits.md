# 第10章 まとめ (2/3): トレイト

## トレイトとは

**定義:** 型が持つべき振る舞いを定義する仕組み（インターフェース）

**目的:** 異なる型に共通の機能を持たせる

---

## 基本的な使い方

### トレイト定義

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

**意味:** 「Summary トレイトを実装する型は、summarize メソッドを持たなければならない」

---

### トレイト実装

```rust
pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub location: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

---

### 使用例

```rust
let article = NewsArticle {
    headline: String::from("Big News"),
    author: String::from("Taro"),
    location: String::from("Tokyo"),
};

let tweet = Tweet {
    username: String::from("user"),
    content: String::from("hello"),
};

println!("{}", article.summarize());  // "Big News, by Taro (Tokyo)"
println!("{}", tweet.summarize());    // "user: hello"
```

---

## デフォルト実装

### 基本的なデフォルト実装

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// デフォルトを使う
impl Summary for NewsArticle {}

let article = NewsArticle { ... };
println!("{}", article.summarize());  // "(Read more...)"
```

---

### デフォルト実装を上書き

```rust
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

let tweet = Tweet { ... };
println!("{}", tweet.summarize());  // "user: hello"
```

---

### デフォルト実装から他のメソッドを呼ぶ

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;  // 実装必須

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
        //                                 ^^^^^^^^^^^^^^^^^^^^^^
        //                                 デフォルトから呼ぶ
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

## 引数としてのトレイト

### impl Trait 構文

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 使用例
notify(&article);  // ✅ OK
notify(&tweet);    // ✅ OK
```

**意味:** 「Summary を実装している任意の型」

---

### トレイト境界構文

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

**同じ意味だが、より明示的**

---

### 複数の引数

```rust
// impl Trait 構文
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
    // item1 と item2 は異なる型でもいい
}

// トレイト境界構文
pub fn notify<T: Summary>(item1: &T, item2: &T) {
    // item1 と item2 は同じ型でなければならない
}
```

---

### 複数のトレイト境界

```rust
// + で複数のトレイトを指定
pub fn notify(item: &(impl Summary + Display)) {
    ...
}

// トレイト境界構文
pub fn notify<T: Summary + Display>(item: &T) {
    ...
}
```

---

### where 句

```rust
// 複雑な場合は where 句で読みやすく
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    ...
}
```

---

## 戻り値としてのトレイト

### impl Trait を返す

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
    }
}
```

**意味:** 「Summary を実装している何らかの型を返す」

---

### 制限: 1種類の型のみ

```rust
// ❌ エラー！異なる型を返せない
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle { ... }  // NewsArticle を返す
    } else {
        Tweet { ... }        // Tweet を返す
    }
}
```

**理由:** `impl Trait` は1種類の型しか返せない

---

## 孤児のルール (Orphan Rule)

### ルール

```
トレイトまたは型の少なくとも1つが
自分のクレートで定義されている必要がある
```

---

### ✅ OK な例

```rust
// 1. 自分の型 + 外部トレイト
impl Display for Tweet { ... }  // ✅ OK

// 2. 外部の型 + 自分のトレイト
impl Summary for Vec<T> { ... }  // ✅ OK

// 3. 自分の型 + 自分のトレイト
impl Summary for Tweet { ... }   // ✅ OK
```

---

### ❌ NG な例

```rust
// 外部の型 + 外部トレイト
impl Display for Vec<T> { ... }  // ❌ エラー！
```

**理由:** 競合を防ぐため

---

## トレイト境界を使った条件付き実装

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// すべての型で使える
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Display + PartialOrd を実装している型のみ
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

**使用例:**
```rust
let pair = Pair::new(5, 10);
pair.cmp_display();  // ✅ OK（i32 は Display + PartialOrd）

let pair = Pair::new(vec![1], vec![2]);
// pair.cmp_display();  // ❌ エラー（Vec は Display を実装していない）
```

---

## ブランケット実装

```rust
// ToString トレイトは、Display を実装している全ての型に対して実装される
impl<T: Display> ToString for T {
    fn to_string(&self) -> String {
        // ...
    }
}

// だから Display を実装していれば to_string が使える
let s = 3.to_string();  // i32 は Display を実装している
```

---

## Python との比較

### Python（抽象基底クラス）

```python
from abc import ABC, abstractmethod

class Summary(ABC):
    @abstractmethod
    def summarize(self):
        pass

class Tweet(Summary):
    def __init__(self, username, content):
        self.username = username
        self.content = content

    def summarize(self):
        return f"{self.username}: {self.content}"

tweet = Tweet("user", "hello")
print(tweet.summarize())
```

---

### Rust（トレイト）

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

let tweet = Tweet {
    username: String::from("user"),
    content: String::from("hello"),
};
println!("{}", tweet.summarize());
```

---

## まとめ

### トレイトの利点

```
✅ 統一されたインターフェース
✅ ジェネリック関数で使える
✅ 型に共通の振る舞いを保証
✅ デフォルト実装でコード重複を削減
✅ 条件付き実装で柔軟性を持たせる
```

---

### 基本構文

```rust
// トレイト定義
pub trait TraitName {
    fn method(&self) -> ReturnType;

    // デフォルト実装
    fn method_with_default(&self) -> ReturnType {
        // デフォルトの実装
    }
}

// トレイト実装
impl TraitName for TypeName {
    fn method(&self) -> ReturnType {
        // 実装
    }
}

// 引数としてのトレイト
fn function(param: &impl TraitName) { ... }
fn function<T: TraitName>(param: &T) { ... }

// 戻り値としてのトレイト
fn function() -> impl TraitName { ... }

// 複数のトレイト境界
fn function<T: Trait1 + Trait2>(param: &T) { ... }

// where 句
fn function<T>(param: &T)
where
    T: Trait1 + Trait2,
{
    ...
}
```

---

### 重要な概念

```
✅ デフォルト実装: 実装を省略できる
✅ トレイト境界: ジェネリック型に制約を付ける
✅ 孤児のルール: トレイトか型の少なくとも1つは自分のクレート
✅ impl Trait: 「このトレイトを実装している何か」
✅ 条件付き実装: 特定のトレイトを実装している型のみ
```
