# Chapter 17 Q&A Part 2: ポリモーフィズムとディスパッチ

## Q1: screen.run() でそれぞれの draw 関数を呼んでいる

**質問:**
> Drawトレイトを持っている奴らをvecで格納していてそれをforloopで回して、それぞれのdraw関数を読んでいるってことね

**回答:**

**まさにその通りです！**

### コードの動き

```rust
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
    //                      ^^^^^^^^
    //                      Draw トレイトを持つ何か
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            //   ^^^^^^^^^ Button, SelectBox, Image...
            component.draw();
            //        ^^^^ それぞれの draw() が呼ばれる
        }
    }
}
```

### 実行イメージ

```rust
let screen = Screen {
    components: vec![
        Box::new(Button { label: "OK" }),     // 1番目
        Box::new(SelectBox { /* ... */ }),     // 2番目
        Box::new(Image { /* ... */ }),         // 3番目
    ],
};

screen.run();
// ↓ 実際の動き
// 1. Button の draw() が呼ばれる
// 2. SelectBox の draw() が呼ばれる
// 3. Image の draw() が呼ばれる
```

### 重要なポイント

```rust
for component in self.components.iter() {
    component.draw();
    // ↑ Screen は component が何型か知らない
    // ↑ Button? SelectBox? Image? → 気にしない
    // ↑ draw() メソッドがあればそれでOK
}
```

---

## Q2: ダックタイピングとの関係

**質問:**
> ダックタイピングに似た概念とありましたが、今回だとButtonやSelectBoxのインスタンスであるかを確認しているのではなく、ただDrawトレイトを持っていてdraw関数があればいいだけってこと？

**回答:**

**完全に正しい理解です！**

### ダックタイピング

**「アヒルのように歩き、アヒルのように鳴くなら、それはアヒルだ」**

```
型を見ない
↓
メソッドがあるかだけを見る
```

### Rust の場合

```rust
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            // Button? SelectBox? Image?
            // → 知らない、確認しない

            component.draw();
            // ↑ draw() があればそれでOK
        }
    }
}
```

### Python との比較

```python
# Python（動的型付け）
def make_it_draw(obj):
    obj.draw()  # draw があればOK、型チェックなし

# どんな型でも受け入れる
make_it_draw(Button())
make_it_draw(SelectBox())
make_it_draw(Image())
```

```rust
// Rust（静的型付け + トレイト）
fn make_it_draw(obj: &dyn Draw) {
    obj.draw();  // Draw トレイトがあればOK
}

// コンパイル時にチェック
make_it_draw(&Button { });     // ✅ OK
make_it_draw(&SelectBox { });  // ✅ OK
make_it_draw(&String::new());  // ❌ エラー: Draw 未実装
```

**違い:**
- Python: 実行時に draw() があるか確認（動的）
- Rust: コンパイル時に Draw トレイトがあるか確認（静的）

---

## Q3: スタティックディスパッチとダイナミックディスパッチ

**質問:**
> スタティックディスパッチとダイナミックディスパッチって何？噛み砕いて教えて

**回答:**

**「いつ、どのメソッドを呼ぶか決まるか」の違いです。**

### スタティックディスパッチ（静的）

**コンパイル時に決まる**

```rust
fn draw<T: Draw>(item: &T) {
    item.draw();
    // ↑ コンパイル時に決定
}

// コンパイル後
fn draw_button(item: &Button) {
    item.draw();  // Button::draw への直接呼び出し
}

fn draw_selectbox(item: &SelectBox) {
    item.draw();  // SelectBox::draw への直接呼び出し
}
```

**特徴:**
- ⚡⚡⚡ 速い（直接呼び出し）
- 📦📦📦 コードサイズ大（型ごとに関数が生成される）
- 同じ型のみ扱える

### ダイナミックディスパッチ（動的）

**実行時に決まる**

```rust
fn draw(item: &dyn Draw) {
    item.draw();
    // ↑ 実行時に決定（vtable 経由）
}
```

**vtable の仕組み:**

```
実行時:
1. vtable を見る
   ┌──────────────┐
   │ draw: 0x1234 │ ← Button::draw のアドレス
   └──────────────┘

2. アドレスにジャンプ
   → Button::draw() が実行される
```

**特徴:**
- ⚡⚡ 少し遅い（vtable 経由）
- 📦 コードサイズ小
- 異なる型を混在できる

### 比較

```rust
// スタティック（ジェネリクス）
fn process<T: Draw>(items: Vec<T>) {
    // Button だけ、または SelectBox だけ
}

// ダイナミック（トレイトオブジェクト）
fn process(items: Vec<Box<dyn Draw>>) {
    // Button と SelectBox を混在できる
}
```

---

## Q4: Vec<T> vs Vec<Box<dyn Draw>>

**質問:**
> ジェネリクスは一つの方しかダメだしVecも<T>の場合だと異なる型は使えませんよね Vec<Box<dyn Draw>>だったらDrawトレイトを持っているなら異なる型でも大丈夫ってことで合っている？

**回答:**

**完璧に正しい理解です！**

### Vec<T> の場合（ジェネリクス）

```rust
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
    //                  ^ 1つの型のみ
}

// Button だけ
let screen: Screen<Button> = Screen {
    components: vec![
        Button { label: "OK" },
        Button { label: "Cancel" },
    ],
};

// 混在は不可能
let screen = Screen {
    components: vec![
        Button { },
        SelectBox { },  // ❌ エラー: 型が違う
    ],
};
```

### Vec<Box<dyn Draw>> の場合（トレイトオブジェクト）

```rust
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
    //                      ^^^^^^^^
    //                      異なる型を混在できる
}

// 混在できる
let screen = Screen {
    components: vec![
        Box::new(Button { label: "OK" }),
        Box::new(SelectBox { options: vec![...] }),
        Box::new(Image { path: "..." }),
    ],
};
```

### 比較表

| 項目 | `Vec<T>` | `Vec<Box<dyn Draw>>` |
|------|---------|---------------------|
| **型** | 1つの型のみ | 異なる型を混在できる |
| **決定** | コンパイル時 | 実行時 |
| **速度** | 速い | 少し遅い |
| **柔軟性** | 低い | 高い |

### 実際の使い分け

```rust
// 全て同じ型 → Vec<T> を使う
let buttons: Vec<Button> = vec![
    Button { label: "OK" },
    Button { label: "Cancel" },
];

// 異なる型を混ぜたい → Vec<Box<dyn Trait>> を使う
let components: Vec<Box<dyn Draw>> = vec![
    Box::new(Button { label: "OK" }),
    Box::new(SelectBox { /* ... */ }),
];
```

---

## まとめ

| 質問項目 | キーポイント |
|---------|------------|
| **screen.run()** | for ループで各コンポーネントの draw() を呼ぶ |
| **ダックタイピング** | 型ではなく、メソッドの有無だけを見る |
| **スタティック** | コンパイル時決定、速い、同じ型のみ |
| **ダイナミック** | 実行時決定、vtable 経由、異なる型OK |
| **Vec<T>** | 1つの型のみ（ジェネリクス） |
| **Vec<Box<dyn Draw>>** | 異なる型を混在可能（トレイトオブジェクト） |
