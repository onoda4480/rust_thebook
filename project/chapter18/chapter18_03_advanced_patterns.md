# Chapter 18-03: 高度なパターン

## マッチガード

**マッチガード**は、`match` アームに追加の `if` 条件を指定する機能です。

### 基本的な使い方

```rust
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("5未満: {}", x),
    //      ^^^^^^^^ マッチガード
    Some(x) => println!("{}", x),
    None => (),
}
```

### マッチガードでシャドーイングを防ぐ

```rust
let x = Some(5);
let y = 10;

match x {
    Some(50) => println!("Got 50"),
    Some(n) if n == y => println!("Matched, n = {:?}", n),
    //   ^         ^
    //   |         外側の y を使える
    //   新しい変数 n
    _ => println!("Default case, x = {:?}", x),
}

println!("at the end: x = {:?}, y = {:?}", x, y);
```

**問題のある例:**
```rust
match x {
    Some(y) if y == y => {
    //   ^         ^ ^
    //   |         | 新しい y
    //   |         新しい y
    //   新しい y（外側の y を隠す）

        // 常に true（バグ！）
        println!("Matched");
    }
}
```

### OR パターンとマッチガード

```rust
let x = 4;
let y = false;

match x {
    4 | 5 | 6 if y => println!("yes"),
    //        ^^^^^ (4 || 5 || 6) && y
    _ => println!("no"),
}
// 出力: no（y が false なので）
```

**注意:** マッチガードは OR 全体に適用される

```rust
// これは
4 | 5 | 6 if y
// ↓ このように解釈される
(4 | 5 | 6) if y

// ↓ こうではない
4 | 5 | (6 if y)
```

## `@` バインディング

**`@` 演算子**は、値が特定のパターンにマッチするかテストしつつ、その値を変数に保存します。

### 基本的な使い方

```rust
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello {
        id: id_variable @ 3..=7,
        //  ^^^^^^^^^^^   ^^^^^
        //  変数名        範囲チェック
    } => {
        println!("Found an id in range: {}", id_variable);
        //                                    ^^^^^^^^^^^ 使える！
    }
    Message::Hello { id: 10..=12 } => {
        //               ^^^^^^^ 範囲チェックだけ
        println!("Found an id in another range");
        // id の値は使えない（変数がない）
    }
    Message::Hello { id } => {
        //               ^^ 変数に束縛だけ
        println!("Found some other id: {}", id);
        // 範囲チェックなし
    }
}
```

### `@` の利点

**パターン1: マッチガードを使う（冗長）**
```rust
Message::Hello { id } if id >= 3 && id <= 7 => {
    println!("Found: {}", id);
}
```

**パターン2: `@` を使う（簡潔）**
```rust
Message::Hello { id: id_variable @ 3..=7 } => {
    println!("Found: {}", id_variable);
}
```

### 範囲だけだと値が使えない

```rust
Message::Hello { id: 10..=12 } => {
    // id という変数は存在しない
    // println!("id: {}", id);  // ❌ エラー！

    // 範囲をチェックしただけで
    // 値を変数に束縛していない
}
```

**なぜ？**
```rust
id: 10..=12
^^  ^^^^^^^
|   範囲パターン（チェックするだけ）
フィールド名

// 変数を作っていない！
```

**修正:**
```rust
Message::Hello { id: id_var @ 10..=12 } => {
    println!("id: {}", id_var);  // ✅ OK
}
```

### `@` の比較

| パターン | 範囲チェック | 値を使える |
|---------|------------|----------|
| `id: id_var @ 3..=7` | ✅ 3〜7 | ✅ `id_var` |
| `id: 10..=12` | ✅ 10〜12 | ❌ なし |
| `id` | ❌ なし | ✅ `id` |

## パターンのまとめ

### パターンの組み合わせ例

```rust
struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("Quit");
        }
        Message::Move { x: x_val @ 0..=10, y } => {
            //             ^^^^^^^^^^^^^^^^ @ バインディング + 範囲
            println!("Move to x: {} (small), y: {}", x_val, y);
        }
        Message::Move { x, y } => {
            println!("Move to x: {}, y: {}", x, y);
        }
        Message::Write(ref text) if text.len() > 10 => {
            //         ^^^^^^^^ ref  ^^^^^^^^^^^^^^^ マッチガード
            println!("Long message: {}", text);
        }
        Message::Write(text) => {
            println!("Text: {}", text);
        }
        Message::ChangeColor(r, g, b) if r > 200 || g > 200 || b > 200 => {
            //                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ マッチガード
            println!("Bright color: ({}, {}, {})", r, g, b);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Color: ({}, {}, {})", r, g, b);
        }
    }
}
```

### よく使うパターンのチートシート

```rust
// 1. 基本的なマッチ
match x {
    1 => {},
    2 | 3 => {},        // OR
    4..=10 => {},       // 範囲
    _ => {},            // その他
}

// 2. Option/Result
match option {
    Some(x) => {},
    None => {},
}

match result {
    Ok(value) => {},
    Err(e) => {},
}

// 3. 構造体分解
let Point { x, y } = point;
match point {
    Point { x: 0, y } => {},
    Point { x, y: 0 } => {},
    Point { x, y } => {},
}

// 4. タプル分解
let (a, b, c) = tuple;
match tuple {
    (0, y, z) => {},
    (x, 0, z) => {},
    (x, y, z) => {},
}

// 5. 参照
match &value {
    &x => {},           // 参照を外す
}

match value {
    ref x => {},        // 参照を作る
}

// 6. マッチガード
match num {
    Some(x) if x > 10 => {},
    Some(x) => {},
    None => {},
}

// 7. @ バインディング
match msg {
    Message::Hello { id: id_var @ 3..=7 } => {},
    Message::Hello { id } => {},
}
```

## まとめ

| 機能 | 構文 | 用途 |
|------|------|------|
| **マッチガード** | `Some(x) if x > 10` | 追加条件でフィルタ |
| **`@` バインディング** | `id @ 3..=7` | 範囲チェック + 変数束縛 |
| **`ref`** | `ref x` | 参照を作る（所有権を奪わない） |
| **`&`** | `&x` | 参照を外す |
| **OR** | `1 \| 2 \| 3` | 複数パターンマッチ |
| **範囲** | `1..=5` | 範囲にマッチ |
| **`_`** | `_`, `_x` | 値を無視 |
| **`..`** | `Point { x, .. }` | 残りを無視 |
