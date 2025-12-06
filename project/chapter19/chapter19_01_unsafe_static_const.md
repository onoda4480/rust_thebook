# 第19章 Part 1: unsafe Rust と static/const

## static vs const の違い

### const（定数）
- **コンパイル時にインライン展開される**
- 使用される場所ごとに値がコピーされる
- メモリアドレスは毎回異なる可能性がある
- 例:
  ```rust
  const HELLO_WORLD: &str = "Hello, world!";

  fn main() {
      println!("{:p}", &HELLO_WORLD);  // アドレスAが表示される
      println!("{:p}", &HELLO_WORLD);  // アドレスBが表示される（異なる可能性）
  }
  ```

### static（静的変数）
- **プログラム全体で固定のメモリアドレスを持つ**
- メモリ上に1つだけ存在する
- すべての参照が同じアドレスを指す
- 例:
  ```rust
  static HELLO_WORLD: &str = "Hello, world!";

  fn main() {
      println!("{:p}", &HELLO_WORLD);  // 常に同じアドレスが表示される
      println!("{:p}", &HELLO_WORLD);  // 同じアドレス
  }
  ```

### まとめ
| 特徴 | const | static |
|------|-------|--------|
| メモリ配置 | インライン展開 | 固定アドレス |
| アドレス | 毎回異なる可能性 | 常に同じ |
| 可変性 | 不可 | `static mut` で可能（unsafe） |

---

## static mut のエラー（Rust 2024エディション）

### 問題
The Bookに記載されているコードがRust 2024でエラーになる:

```rust
static mut COUNTER: u32 = 0;

fn main() {
    unsafe {
        println!("COUNTER: {}", COUNTER);  // ❌ エラー！
    }
}
```

**エラーメッセージ**:
```
error: creating a shared reference to mutable static is discouraged
```

### 原因
Rust 2024エディションでは、**可変静的変数への直接参照作成が禁止**された。
`println!` マクロは内部で `&COUNTER` という参照を作成するため、エラーになる。

### 解決方法
一時変数に値を保存してから使用する:

```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        let count = COUNTER;  // ✅ 値を読んで変数に保存
        println!("COUNTER: {}", count);  // ✅ 参照ではなく値を使う
    }
}
```

### ポイント
- **The Bookとの違い**: The Bookは古いエディションの例なので、Rust 2024では修正が必要
- **理由**: 可変静的変数への参照は安全性の問題があるため、Rust 2024でより厳格になった
- **対処**: 値を直接読み取る方式に変更する

---

## unsafe トレイト

unsafe キーワードは関数だけでなく、トレイトにも使用できる:

```rust
unsafe trait Foo {
    // メソッドがここに来る
}

unsafe impl Foo for i32 {
    // メソッドの実装がここに来る
}
```

### いつ使う？
- トレイトのメソッドが不変条件を持つ場合
- 実装者が守るべき安全性の契約がある場合
- 例: `Send` と `Sync` トレイト（次のセクションで詳細）
