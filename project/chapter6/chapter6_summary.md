# Chapter 6: Enum とパターンマッチング - まとめ

## この章で学んだこと

1. **Enum の定義と使用**
2. **match 式によるパターンマッチング**
3. **if let 構文による簡潔な記法**
4. **Option<T> による null 安全性**

---

## 1. Enum（列挙型）

### Enum とは？

**複数の選択肢の中から1つを選ぶ型**

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

---

### Struct vs Enum

| 概念 | 関係 | 例 |
|---|---|---|
| **Struct** | AND関係 | 全てのフィールドを持つ |
| **Enum** | OR関係 | 1つのバリアントを持つ |

```rust
// Struct = AND
struct User {
    username: String,  // AND
    email: String,     // AND
}

// Enum = OR
enum IpAddr {
    V4(...),  // OR
    V6(...),  // OR
}
```

---

### バリアントの種類

```rust
enum Message {
    Quit,                       // データなし
    Move { x: i32, y: i32 },    // 名前付きフィールド
    Write(String),              // タプル型
    ChangeColor(i32, i32, i32), // 複数の値
}
```

**ポイント:** 各バリアントは異なる型とデータ量を持てる

---

### Enum にメソッドを実装

```rust
impl Message {
    fn call(&self) {
        // メソッドの実装
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

---

## 2. Option<T>

### Option<T> とは？

**値があるかもしれない状態を表す Enum**

```rust
enum Option<T> {
    Some(T),
    None,
}
```

---

### なぜ Option<T> が必要？

**null の問題を型システムで解決**

| 項目 | null | Option<T> |
|---|---|---|
| **型安全性** | ❌ なし | ✅ あり |
| **コンパイラチェック** | ❌ なし | ✅ あり |
| **クラッシュ** | ✅ する | ❌ しない |

---

### None vs null

```
null = ほんとに何もない（型システムの外）
None = 無いことを表す値（型システムの内）

Rust:
- None は Option<T> の一部
- 使用前に必ず存在チェックが必要
- nullポインタエラーが起きない
```

---

### Prelude

**自動的にインポートされる型や関数のセット**

```rust
// ❌ 不要
// use std::option::Option;

// ✅ そのまま使える
let x = Some(5);
let y: Option<i32> = None;
```

**Prelude に含まれるもの:**
- `Option<T>` (`Some`, `None`)
- `Result<T, E>` (`Ok`, `Err`)
- `Vec<T>`, `String`
- `println!`

---

### Option<T> と T は別の型

```rust
let x: i32 = 5;
let y: Option<i32> = Some(5);

// let sum = x + y;  // ❌ エラー！型が違う
```

**重要:** 明示的に変換が必要 → 安全性が保証される

---

## 3. match 式

### match とは？

**パターンに基づいて処理を分岐させる制御フロー構文**

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

---

### 包括性チェック（Exhaustive Checking）

**全てのパターンを網羅する必要がある**

```rust
// ❌ エラー！
match x {
    Some(i) => Some(i + 1),
    // None のケースがない
}

// ✅ OK
match x {
    Some(i) => Some(i + 1),
    None => None,  // 全パターン網羅
}
```

**利点:** 処理漏れを防ぐ

---

### パターンに値を束縛

```rust
enum Coin {
    Penny,
    Quarter(UsState),
}

match coin {
    Coin::Penny => 1,
    Coin::Quarter(state) => {
        println!("State: {:?}", state);  // state を使える
        25
    }
}
```

**処理の流れ:**
1. `Coin::Quarter` にマッチ
2. 内部の `UsState` を `state` 変数に束縛
3. 処理を実行
4. 値を返す

---

### 包括パターン `_`

```rust
let dice_roll = 9;

match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),  // 他の全て
}
```

**`_` の意味:**
- 「他の全て」にマッチ
- 値を束縛しない

---

### 何もしないパターン

```rust
match dice_roll {
    3 => add_fancy_hat(),
    _ => (),  // 何もしない
}
```

**`()` = ユニット値**（何も返さない）

---

### match は式

```rust
let result = match x {
    Some(value) => value + 1,
    None => 0,
};
```

**ポイント:**
- `match` 全体が1つの式
- 値を返す
- 全アームの型が一致する必要がある

---

## 4. if let 構文

### if let とは？

**1つのパターンだけマッチさせたい時の簡潔な記法**

---

### match との比較

#### match（冗長）

```rust
match config_max {
    Some(max) => println!("最大値: {}", max),
    None => (),  // 何もしないけど書く必要がある
}
```

---

#### if let（簡潔）

```rust
if let Some(max) = config_max {
    println!("最大値: {}", max);
}
```

---

### 包括性チェックの違い

| 構文 | 包括性チェック | 特徴 |
|---|---|---|
| `match` | ✅ あり | 全パターン必須 |
| `if let` | ❌ なし | 1パターンでOK |

---

### if let の実践例

#### 1. Option<T> を処理

```rust
if let Some(value) = maybe_number {
    println!("値: {}", value);
}
```

---

#### 2. else を使った処理

```rust
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

**これは以下と同じ:**

```rust
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

### if let vs Python if

```
Python の if        = 真偽値で分岐
Rust の if let      = パターンマッチで値を取り出しつつ分岐
Rust の普通の if    = Python の if と同じ（真偽値で分岐）
```

```rust
// Rust の普通の if（Python と同じ）
if x > 3 {
    println!("大きい");
}

// Rust の if let（パターンマッチング）
if let Some(value) = x {
    println!("値: {}", value);
}
```

---

## 使い分けガイド

### Struct vs Enum

```
複数のデータを同時に持つ → Struct
複数の選択肢から1つ     → Enum

例:
- ユーザー情報（名前AND年齢） → Struct
- IPアドレス（V4 OR V6）     → Enum
```

---

### match vs if let

```
全パターンを処理する必要がある → match
1パターンだけ処理したい       → if let

例:
- 全てのEnumバリアントを処理 → match
- Someの時だけ処理（Noneは無視）→ if let
```

---

### match vs if

```
パターンマッチング → match / if let
条件分岐（bool）   → if

例:
- Enumを処理      → match
- 値を取り出す    → match / if let
- 真偽値で分岐    → if
```

---

## 重要な概念

### 1. Enum = OR関係

```
いずれか1つのバリアントを持つ
型システムで「どちらか1つ」を保証
```

---

### 2. Option<T> = null 安全性

```
値があるかもしれない状態を型で表現
コンパイラが存在チェックを強制
nullポインタエラーが起きない
```

---

### 3. match = 包括性チェック

```
全パターンを網羅する必要がある
処理漏れを防ぐ
コンパイル時にチェック
```

---

### 4. if let = 簡潔な記法

```
1パターンだけ処理
包括性チェックなし
簡潔に書ける
```

---

## コード例集

### Enum の定義

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

---

### Option<T> の使用

```rust
let some_number = Some(5);
let some_string = Some("a string");
let absent_number: Option<i32> = None;
```

---

### match でパターンマッチング

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```

---

### if let で簡潔に

```rust
if let Some(max) = config_max {
    println!("最大値: {}", max);
}
```

---

## まとめ

### Enum

```
✅ OR関係を表現
✅ バリアントごとに異なるデータ
✅ メソッドを実装可能
✅ 型安全性
```

---

### Option<T>

```
✅ null安全性を提供
✅ Preludeに含まれる
✅ 存在チェックを強制
✅ クラッシュを防ぐ
```

---

### match

```
✅ パターンマッチング
✅ 包括性チェック
✅ 値の束縛
✅ 式として値を返す
```

---

### if let

```
✅ 簡潔な記法
✅ 1パターンだけ処理
✅ 包括性チェックなし
✅ elseで他のケースも処理可能
```

---

## 次のステップ

第六章で学んだ内容：
- ✅ Enum の定義と使用
- ✅ Option<T> による null 安全性
- ✅ match によるパターンマッチング
- ✅ if let による簡潔な記法

**これらの知識は Rust プログラミングの基礎となります！**

次の章では、さらに高度なトピックに進みます。
