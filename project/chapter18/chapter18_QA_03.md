# Chapter 18 Q&A Part 3: マッチガードと @ バインディング

## Q1: マッチガードでのシャドーイング

**質問:**
> 変数xのマッチ内でyがsome(5)で覆い隠され、その後のマッチガードで自分自身を比較しているため常にtrueだからバグの温床になるってこと？

**回答:**

**完璧に理解されています！** まさにその通りです。

### 問題の構造

```rust
let x = Some(5);
let y = 10;  // ← 外側のy

match x {
    Some(y) if y == y => {
    //   ^  シャドーイング！
    //      y = 5 になる
    //
    //         ^ ^
    //         | 新しいy (5)
    //         新しいy (5)
    //
    // 5 == 5 → 常に true ❌
        println!("Matched");
    }
}
```

**バグ:**
- プログラマは「外側の `y`（10）と比較したい」と思っている
- しかし実際は「新しい `y`（5）自身と比較」している
- `5 == 5` は常に `true`
- **意図と違う動作になる**

### 実際のバグ例

```rust
fn main() {
    let target = 100;  // 探したい値
    let numbers = vec![Some(1), Some(50), Some(100), Some(200)];

    for num in numbers {
        match num {
            // ❌ バグ！target をシャドーイングしている
            Some(target) if target == target => {
                println!("Found target: {}", target);
            }
            _ => {}
        }
    }
}
```

**出力:**
```
Found target: 1
Found target: 50
Found target: 100
Found target: 200
```

**全部マッチしてしまう！** `target == target` は常に `true` だから。

### 正しい書き方

```rust
fn main() {
    let target = 100;  // 探したい値
    let numbers = vec![Some(1), Some(50), Some(100), Some(200)];

    for num in numbers {
        match num {
            // ✅ 正しい！別の変数名を使う
            Some(n) if n == target => {
                println!("Found target: {}", n);
            }
            _ => {}
        }
    }
}
```

**出力:**
```
Found target: 100
```

**100だけマッチ！** `n == target` で正しく外側の `target` と比較できている。

### 比較

| コード | 比較内容 | 結果 | 問題 |
|--------|---------|------|------|
| `Some(y) if y == y` | `5 == 5` | 常に `true` | ❌ バグの温床 |
| `Some(n) if n == y` | `5 == 10` | 正しく評価 | ✅ 正しい |

### 図解

```rust
let y = 10;  // 外側のy
match x {
    Some(n) if n == y => {
    //   ^         ^
    //   |         外側のy（10）✅
    //   新しいn（5）
    }
}
```

vs

```rust
let y = 10;  // 外側のy（隠される）
match x {
    Some(y) if y == y => {
    //   ^         ^ ^
    //   |         | 新しいy（5）
    //   |         新しいy（5）
    //   新しいy（5）
    //   外側のy（10）は使えない❌
    }
}
```

---

## Q2: @ バインディングでなぜ id が存在しない？

**質問:**
> ```rust
> Message::Hello { id: 10..=12 } => {
>     println!("Found an id in another range {}", id)
> }
> ```
> なぜidが存在しない？
> Message::Hello内のidの値をmatch内で確かめているだけなのでprintで使用できない認識で大丈夫？

**回答:**

**100点満点です！** まさにその通りです。

### パターンの構造

```rust
Message::Hello { id: 10..=12 }
//               ^^  ^^^^^^^
//               |   範囲パターン（チェックするだけ）
//               フィールド名
//
// 変数を作っていない！
```

### パターンの文法

```rust
構造体 { フィールド名: パターン }
```

**重要:** `フィールド名: パターン` の部分で、**変数が作られるかどうか**が決まります。

### ケース1: 範囲パターン（変数を作らない）

```rust
Message::Hello { id: 10..=12 } => {
//               ^^  ^^^^^^^
//               |   これは「チェック」するだけ
//               |   変数は作られない
//               フィールド名

    // id という変数は存在しない！
    // ただ「id フィールドが 10〜12 か？」を確認しただけ

    // println!("{}", id);  // ❌ エラー！
}
```

### ケース2: 変数パターン（変数を作る）

```rust
Message::Hello { id } => {
//               ^^
//               これは省略記法
//               id: id と同じ意味
//               ↓
//               変数 id を作る

    // id という変数が存在する！
    println!("{}", id);  // ✅ OK
}
```

完全に書くと：
```rust
Message::Hello { id: id } => {
//               ^^  ^^
//               |   変数名（これを作る）
//               フィールド名
}
```

### ケース3: `@` パターン（チェック + 変数を作る）

```rust
Message::Hello { id: id_var @ 10..=12 } => {
//               ^^  ^^^^^^   ^^^^^^^
//               |   |        範囲チェック
//               |   変数名（これを作る）
//               フィールド名

    // id_var という変数が存在する！
    println!("{}", id_var);  // ✅ OK
}
```

### 比較表

| パターン | 何をしているか | 変数作成 | 使える変数 |
|---------|--------------|---------|----------|
| `id: 10..=12` | 範囲チェックだけ | ❌ なし | なし |
| `id: id` または `id` | 値を変数に束縛 | ✅ `id` | `id` |
| `id: x @ 10..=12` | 範囲チェック + 変数束縛 | ✅ `x` | `x` |

### 実際のエラー

```rust
Message::Hello { id: 10..=12 } => {
    println!("Found an id in another range {}", id);
    //                                          ^^ エラー！
}
```

**エラーメッセージ:**
```
error[E0425]: cannot find value `id` in this scope
  --> src/main.rs:19:57
   |
19 |             println!("Found an id in another range {}", id)
   |                                                         ^^ not found in this scope
```

### 修正方法

#### 方法1: `@` で変数を束縛する

```rust
Message::Hello { id: id_var @ 10..=12 } => {
//               ^^^^^^^^^^
//               変数 id_var を作る
    println!("Found an id in another range {}", id_var);
    //                                          ^^^^^^ 使える！
}
```

#### 方法2: 値が不要なら変数を使わない

```rust
Message::Hello { id: 10..=12 } => {
    println!("Found an id in another range");
    // 値を表示しない
}
```

### 図で理解

```rust
enum Message {
    Hello { id: i32 },  // ← 実際の値: 11
}

let msg = Message::Hello { id: 11 };

match msg {
    Message::Hello { id: 10..=12 } => {
    //               ^^  ^^^^^^^
    //               |   11 が 10〜12 の範囲？ → Yes
    //               |
    //               チェックしただけ
    //               変数は作られていない

        // println!("{}", id);  ← id は存在しない！
    }
}
```

vs

```rust
match msg {
    Message::Hello { id } => {
    //               ^^
    //               11 を変数 id に束縛

        println!("{}", id);  // ✅ 11
    }
}
```

### あなたの理解の確認

> Message::Hello内のidの値をmatch内で確かめているだけなのでprintで使用できない認識で大丈夫？

**→ 完璧です！**

```
id: 10..=12
    ^^^^^^^ ← これは「チェック（確認）」するだけ
              変数は作らない
              だから使えない
```

---

## まとめ

| 質問項目 | キーポイント |
|---------|------------|
| **シャドーイングの危険性** | `Some(y) if y == y` は常に true（バグ） |
| **正しい書き方** | 別の変数名を使う `Some(n) if n == y` |
| **範囲パターン** | チェックするだけで変数を作らない |
| **変数パターン** | 値を変数に束縛する |
| **@ バインディング** | チェック + 変数束縛の両方 |

### 重要なポイント

1. **マッチガード:**
   - シャドーイングに注意
   - 外側の変数と比較したいなら別の変数名を使う
   - `Some(y) if y == y` は常に true（バグ）

2. **@ バインディング:**
   - `id: 10..=12` は変数を作らない
   - `id: id_var @ 10..=12` で変数を作る
   - 範囲チェックと値の使用を同時に実現

3. **パターンの意味:**
   - `フィールド: パターン` でパターンが何をするか決まる
   - 範囲パターンはチェックだけ
   - 変数パターンは束縛
   - `@` は両方
