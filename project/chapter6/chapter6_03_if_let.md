# Chapter 6-3: if let 構文

## if let とは？

**1つのパターンだけマッチさせたい時の簡潔な記法**

---

## match の冗長性

### match で1パターンだけ処理

```rust
let config_max = Some(3u8);

match config_max {
    Some(max) => println!("最大値: {}", max),
    None => (),  // ← 何もしないけど書く必要がある
}
```

**問題点:**
- `None` のケースは何もしない
- でも包括性チェックで書く必要がある
- 冗長

---

## if let で簡潔に

### 同じ処理を簡潔に書く

```rust
let config_max = Some(3u8);

if let Some(max) = config_max {
    println!("最大値: {}", max);
}
```

**利点:**
- 簡潔
- `None` のケースを書かなくていい
- 読みやすい

---

## if let の構文

### 基本形

```rust
if let パターン = 値 {
    // パターンにマッチした時の処理
}
```

---

### else を追加

```rust
if let パターン = 値 {
    // マッチした時
} else {
    // マッチしなかった時
}
```

---

## Python の if との違い

### Python の if: 条件分岐

```python
x = 5

if x > 3:
    print("大きい")
else:
    print("小さい")
```

**やること:** 真偽値で分岐

---

### Rust の if let: パターンマッチング

```rust
let x = Some(5);

if let Some(value) = x {
    println!("値: {}", value);
} else {
    println!("値なし");
}
```

**やること:**
1. パターンにマッチするか確認
2. マッチしたら値を取り出す（束縛）
3. 分岐

---

### Rust にも普通の if がある

```rust
let x = 5;

if x > 3 {
    println!("大きい");
} else {
    println!("小さい");
}
```

**これは Python の if と同じ！**

---

## match vs if let

### match: 全パターンを網羅

```rust
let coin = Coin::Penny;

match coin {
    Coin::Penny => println!("1セント"),
    Coin::Nickel => println!("5セント"),
    Coin::Dime => println!("10セント"),
    Coin::Quarter => println!("25セント"),
}
// 全てのケースが必要
```

**特徴:**
- 包括性チェックが**働く**
- 全パターンを処理する必要がある

---

### if let: 1パターンだけ処理

```rust
let coin = Coin::Penny;

if let Coin::Quarter = coin {
    println!("25セント硬貨発見！");
}
// 他のコインは無視
```

**特徴:**
- 包括性チェックが**働かない**
- 1つのパターンだけ処理できる

---

## 包括性チェックの違い

### match: 包括性チェック あり

```rust
let x = Some(5);

match x {
    Some(value) => println!("値: {}", value),
    // None がないのでエラー！
}
// ❌ コンパイルエラー: non-exhaustive patterns
```

---

### if let: 包括性チェック なし

```rust
let x = Some(5);

if let Some(value) = x {
    println!("値: {}", value);
}
// None のケースがなくても ✅ OK！
```

---

## if let の実践例

### 1. Option<T> を処理

```rust
let maybe_number = Some(42);

if let Some(n) = maybe_number {
    println!("数値: {}", n);
}
// None の時は何もしない
```

---

### 2. Result<T, E> から成功値だけ処理

```rust
let result: Result<i32, &str> = Ok(10);

if let Ok(value) = result {
    println!("成功: {}", value);
}
// Err の時は何もしない
```

---

### 3. Enumの特定バリアントだけ処理

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

let msg = Message::Write(String::from("hello"));

if let Message::Write(text) = msg {
    println!("メッセージ: {}", text);
}
// Quit と Move は無視
```

---

### 4. else を使った処理

```rust
let coin = Coin::Penny;
let mut count = 0;

if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

**これは以下と同じ:**

```rust
let coin = Coin::Penny;
let mut count = 0;

match coin {
    Coin::Quarter(state) => {
        println!("State quarter from {:?}!", state);
    }
    _ => {
        count += 1;
    }
}
```

---

## いつ使う？

### if let を使う場面

```
✅ 1つのパターンだけ処理したい
✅ 他のケースは無視してOK
✅ コードを簡潔にしたい
```

**例:**
- `Some` の時だけ処理（`None` は無視）
- `Ok` の時だけ処理（`Err` は無視）
- 特定のEnumバリアントだけ処理

---

### match を使う場面

```
✅ 全パターンを処理する必要がある
✅ 複数のパターンを処理したい
✅ 包括性チェックが欲しい
```

**例:**
- 全てのEnumバリアントを処理
- エラーハンドリング（`Ok` と `Err` 両方）
- 状態遷移の実装

---

## 比較表

| 項目 | match | if let |
|---|---|---|
| **包括性チェック** | ✅ あり | ❌ なし |
| **パターン数** | 全て | 1つ |
| **else** | `_` パターン | `else` ブロック |
| **用途** | 全パターン処理 | 1パターン処理 |
| **冗長性** | やや冗長 | 簡潔 |

---

## Python との比較

### Python の構造化パターンマッチング（3.10+）

```python
value = {"type": "some", "data": 5}

match value:
    case {"type": "some", "data": x}:
        print(f"値: {x}")
    case _:
        print("値なし")
```

**Rust の if let に近いが、簡潔さでは劣る**

---

### Rust の if let

```rust
let x = Some(5);

if let Some(value) = x {
    println!("値: {}", value);
}
```

**より簡潔で読みやすい**

---

## まとめ

### if let の重要ポイント

| 概念 | 説明 |
|---|---|
| **目的** | 1パターンだけ処理 |
| **簡潔性** | match より簡潔 |
| **包括性** | チェックなし（柔軟） |
| **else** | 他のケースを処理可能 |

---

### 使い分け

```
if let   = 1パターンだけ処理（簡潔）
match    = 全パターン処理（厳格）
普通の if = 条件分岐（bool）

選び方:
1. 全パターン処理が必要 → match
2. 1パターンだけ処理    → if let
3. 真偽値で分岐         → if
```

---

### if let vs Python if

```
Python if        = 真偽値で分岐
Rust if let      = パターンマッチで値を取り出しつつ分岐
Rust 普通の if   = Python if と同じ（真偽値で分岐）

if let は Python にはない Rust 特有の機能
```

---

### 包括性チェック

```
match    = 包括性チェック あり（全パターン必須）
if let   = 包括性チェック なし（1パターンでOK）

利点:
match    → 処理漏れを防ぐ（安全）
if let   → 柔軟に書ける（簡潔）
```
