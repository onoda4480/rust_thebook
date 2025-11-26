# Chapter 17-3: ディスパッチとオブジェクト安全性

## スタティックディスパッチ vs ダイナミックディスパッチ

### スタティックディスパッチ（静的）

**コンパイル時にどのメソッドを呼ぶか決まる**

```rust
fn draw<T: Draw>(item: &T) {
    item.draw();
    // ↑ コンパイル時に決定
    // ↑ 直接呼び出し（速い）
}

// コンパイル時に展開される
fn draw_button(item: &Button) {
    item.draw();  // Button::draw への直接呼び出し
}

fn draw_selectbox(item: &SelectBox) {
    item.draw();  // SelectBox::draw への直接呼び出し
}
```

**特徴:**
- ⚡⚡⚡ 速い
- 📦📦📦 コードサイズ大
- 同じ型のみ

---

### ダイナミックディスパッチ（動的）

**実行時にどのメソッドを呼ぶか決まる**

```rust
fn draw(item: &dyn Draw) {
    item.draw();
    // ↑ 実行時に決定
    // ↑ vtable 経由で呼び出し（少し遅い）
}
```

**特徴:**
- ⚡⚡ 少し遅い
- 📦 コードサイズ小
- 異なる型OK

---

### vtable（仮想関数テーブル）

```
dyn Draw
    ↓
┌──────────────┐
│ vtable       │ ← メソッドのアドレス表
├──────────────┤
│ draw: 0x1234 │ ← Button::draw のアドレス
└──────────────┘

実行時:
1. vtable を見る
2. draw のアドレスを取得
3. そのアドレスにジャンプ
```

---

## オブジェクト安全性

### オブジェクト安全なトレイト

```rust
pub trait Draw {
    fn draw(&self);
    //             ← 戻り値なし（() を返す）
}

// トレイトオブジェクトにできる
let obj: Box<dyn Draw> = Box::new(Button { });
```

---

### オブジェクト安全でないトレイト

```rust
pub trait Clone {
    fn clone(&self) -> Self;
    //                 ^^^^
    //                 Self を返す → NG
}

// トレイトオブジェクトにできない
let obj: Box<dyn Clone> = Box::new(String::from("hello"));
// ❌ エラー: Clone はオブジェクト安全でない
```

---

### なぜ Self を返せない？

```rust
// もしこれができたとしたら...
let obj: Box<dyn Clone> = /* 何か */;
let cloned = obj.clone();
//           ^^^^^^^^^^^^
//           何を返せばいい？
//           String? Vec<i32>? Button?
//           型がわからない！
```

---

### Self vs &Self

| 戻り値 | オブジェクト安全？ | 理由 |
|-------|----------------|------|
| **Self** | ❌ NG | 型によってサイズが違う |
| **&Self** | ✅ OK | 参照のサイズは一定 |
| **&str** | ✅ OK | サイズが一定 |
| **()** | ✅ OK | サイズゼロ |

---

## 普通に使う vs トレイトオブジェクト

### Clone の場合

```rust
// ✅ 普通に使う（OK）
let s = String::from("hello");
let s2 = s.clone();  // String::clone を呼ぶ

// ❌ トレイトオブジェクト（NG）
let obj: Box<dyn Clone> = Box::new(s);  // エラー
```

---

## まとめ

| 項目 | スタティック | ダイナミック |
|------|------------|------------|
| **決定** | コンパイル時 | 実行時 |
| **方法** | ジェネリクス | トレイトオブジェクト |
| **速度** | 速い | 少し遅い |
| **柔軟性** | 低い | 高い |

**オブジェクト安全:**
- `Self` を返す → NG
- `&Self` を返す → OK
- トレイトオブジェクトにできるかの基準
