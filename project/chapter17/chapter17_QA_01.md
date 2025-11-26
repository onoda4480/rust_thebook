# Chapter 17 Q&A Part 1: カプセル化とトレイトの基礎

## Q1: カプセル化でVec<i32>からHashSet<i32>に変更できるとは？

**質問:**
> 構造体AveragedCollectionの実装詳細をカプセル化したので、データ構造などの側面を将来容易に変更することができます。例を挙げれば、listフィールドにVec<i32>ではなくHashSet<i32>を使うこともできます。とありますがどゆこと？

**回答:**

カプセル化のメリットは「内部実装を変更しても、外部のコードに影響しない」ということです。

### Before（変更前）

```rust
pub struct AveragedCollection {
    list: Vec<i32>,        // ← Vec を使用
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
}
```

### After（変更後）

```rust
pub struct AveragedCollection {
    list: HashSet<i32>,    // ← HashSet に変更（重複自動除去）
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.insert(value);  // push → insert
        self.update_average();
    }
}
```

### 外部コードは変わらない

```rust
// 使う側のコード
let mut collection = AveragedCollection::new();
collection.add(10);
collection.add(20);
let avg = collection.average();
// ↑ 全く同じコード！変更不要！
```

**重要なポイント:**
- `list` フィールドは private（外から見えない）
- 外部から触れるのは `add()` と `average()` だけ
- 内部で Vec を使おうが HashSet を使おうが、外部コードには影響ない

---

## Q2: trait定義とimpl実装のdrawは同じもの？

**質問:**
> pub trait Draw { fn draw(&self); }とimpl Draw for Button { fn draw(&self) { ... }の fn draw(&self)は同じもの？

**回答:**

**「契約書」と「実際の履行」の関係**です。

### トレイト定義（契約書）

```rust
pub trait Draw {
    fn draw(&self);
    //     ^^^^^^ ← シグネチャだけ（中身なし）
    //              「drawメソッドを持つべき」という約束
}
```

- セミコロン `;` で終わる
- 本体（`{ }` の中身）がない
- 「こういうメソッドがあるべき」という約束だけ

### トレイト実装（実際の履行）

```rust
impl Draw for Button {
    fn draw(&self) {
        // ← 中括弧 { } の中に実際のコード
        println!("ボタンを描画");
    }
}
```

- 中括弧 `{ }` の中に実際のコードがある
- Button は「Draw の約束を守りました」と言っている
- シグネチャ（`fn draw(&self)`）は同じでなければならない

---

## Q3: トレイトでロジック（処理）を書く場合はある？

**質問:**
> traitでは今回の場合だとdraw関数があると言っているだけで実際の実装はimplにあるという認識で大丈夫？またトレイトではロジック(処理)を書く場合はある？

**回答:**

はい、**デフォルト実装**としてトレイト内にロジックを書くことができます。

### デフォルト実装の例

```rust
pub trait Summary {
    fn author(&self) -> String;  // ← 必須（実装必要）

    // デフォルト実装（本体あり）
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.author())
        //      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        //      ← トレイト内に処理がある！
    }
}
```

### 使い方

```rust
struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn author(&self) -> String {
        self.username.clone()
    }
    // summarize() は実装しなくてもOK
    // デフォルト実装が使われる
}

let tweet = Tweet { /* ... */ };
println!("{}", tweet.summarize());
// → "(Read more from @username...)"
```

### デフォルト実装の上書き

```rust
impl Summary for Article {
    fn author(&self) -> String {
        self.author.clone()
    }

    // デフォルトを上書き
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author())
    }
}
```

**まとめ:**
- トレイト定義：シグネチャだけ書く（必須メソッド）
- デフォルト実装：トレイト内に処理を書くこともできる
- 実装側：必須メソッドだけ実装すればOK、デフォルトは上書き可能

---

## Q4: デフォルト実装は簡素なもの？

**質問:**
> デフォルト実装は impl の処理と比べ簡素なものになりますか？

**回答:**

**必ずしもそうではありません。** デフォルト実装は単純なこともあれば、複雑な処理を含むこともあります。

### 簡単な例

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")  // ← シンプル
    }
}
```

### 複雑な例

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;  // ← 必須

    // デフォルト実装（複雑な処理）
    fn count(self) -> usize
    where
        Self: Sized,
    {
        let mut count = 0;
        while let Some(_) = self.next() {
            count += 1;
        }
        count
    }

    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
    {
        Map::new(self, f)  // ← かなり複雑
    }
}
```

**デフォルト実装の目的:**
- コードの再利用（同じ処理を各実装で書かなくて済む）
- 一部のメソッドだけ実装すれば、他のメソッドは自動で使える

---

## Q5: わざわざトレイトを実装しなくても impl や fn で良くない？

**質問:**
> わざわざトレイトを実装しなくてもimplやfnなどで実装すれば良くね？

**回答:**

**トレイトの真の価値は「異なる型を統一的に扱える」こと**です。

### トレイトなしだと...

```rust
struct Button;
impl Button {
    fn draw_button(&self) {
        println!("ボタン描画");
    }
}

struct SelectBox;
impl SelectBox {
    fn draw_select(&self) {
        println!("セレクトボックス描画");
    }
}

// メソッド名がバラバラ → まとめて扱えない
// button.draw_button();
// selectbox.draw_select();
```

### トレイトありだと...

```rust
trait Draw {
    fn draw(&self);
}

impl Draw for Button {
    fn draw(&self) { println!("ボタン描画"); }
}

impl Draw for SelectBox {
    fn draw(&self) { println!("セレクトボックス描画"); }
}

// まとめて扱える！
let components: Vec<Box<dyn Draw>> = vec![
    Box::new(Button {}),
    Box::new(SelectBox {}),
];

for component in &components {
    component.draw();  // ← 統一されたメソッド名
}
```

### Pythonの継承との比較

```python
# Python
class Shape:
    def draw(self):
        pass

class Button(Shape):
    def draw(self):
        print("ボタン描画")

class SelectBox(Shape):
    def draw(self):
        print("セレクトボックス描画")

shapes = [Button(), SelectBox()]
for shape in shapes:
    shape.draw()
```

**トレイトの目的:**
- 型を超えた共通インターフェース
- 異なる型をまとめて扱う
- ポリモーフィズムを実現

---

## まとめ

| 質問項目 | キーポイント |
|---------|------------|
| **カプセル化** | 内部実装の変更が外部に影響しない |
| **トレイト定義** | 契約書（シグネチャだけ） |
| **トレイト実装** | 実際の履行（中身を書く） |
| **デフォルト実装** | トレイト内に処理を書ける、簡単でも複雑でもOK |
| **トレイトの目的** | 異なる型を統一的に扱う |
