# Chapter 17-2: ポリモーフィズムとダックタイピング

## ポリモーフィズム

### 異なる型を同じように扱う

```rust
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

---

### 使用例

```rust
let screen = Screen {
    components: vec![
        Box::new(Button { /* ... */ }),
        Box::new(SelectBox { /* ... */ }),
        Box::new(Image { /* ... */ }),
    ],
};

screen.run();
// ✅ 全部 draw() を呼べる
```

---

## ダックタイピング

### 「アヒルのように歩き、鳴くならアヒル」

**Python（動的型付け）:**
```python
def make_it_draw(obj):
    obj.draw()  # draw メソッドがあればOK
```

**Rust（静的型付け）:**
```rust
fn make_it_draw(obj: &dyn Draw) {
    obj.draw();  // Draw トレイトがあればOK
}
```

---

### 重要なポイント

**型ではなく振る舞いを見る**

```rust
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            // Button? SelectBox? Image?
            // → 知らない、気にしない

            component.draw();
            // draw メソッドさえあればOK
        }
    }
}
```

---

## ジェネリクス vs トレイトオブジェクト

### ジェネリクス（スタティック）

```rust
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
    //                  ^
    //                  1つの型のみ
}

// Button だけ
let screen1: Screen<Button> = Screen {
    components: vec![
        Button { },
        Button { },
    ],
};

// 混在は不可能
let screen2 = Screen {
    components: vec![
        Button { },
        SelectBox { },  // ❌ エラー
    ],
};
```

---

### トレイトオブジェクト（ダイナミック）

```rust
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
    //                      ^^^^^^^^
    //                      異なる型を混在できる
}

// 混在できる
let screen = Screen {
    components: vec![
        Box::new(Button { }),
        Box::new(SelectBox { }),
    ],
};
```

---

## まとめ

| 項目 | ジェネリクス | トレイトオブジェクト |
|------|------------|-------------------|
| **型宣言** | `Vec<T>` | `Vec<Box<dyn Draw>>` |
| **混在** | ❌ できない | ✅ できる |
| **決定** | コンパイル時 | 実行時 |
| **速度** | 速い | 少し遅い |

**ダックタイピング:** 型ではなく、メソッドがあるかだけが重要
