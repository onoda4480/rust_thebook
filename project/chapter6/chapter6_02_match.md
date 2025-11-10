# Chapter 6-2: matchとパターンマッチング

## matchとは？

**パターンに基づいて処理を分岐させる制御フロー構文**

---

## 基本構文

```rust
match 値 {
    パターン1 => 式1,
    パターン2 => 式2,
    パターン3 => 式3,
}
```

**特徴:**
- 全てのパターンを網羅する必要がある（**包括性チェック**）
- 最初にマッチしたパターンの式を実行
- 式なので値を返す

---

## 基本的な例

### コインの価値を返す

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

**ポイント:**
- 各パターンは `=>` で式と結ばれる
- 式の値が関数の戻り値になる
- 全てのバリアントを網羅している

---

## 複数行の処理

### ブロック `{}` を使う

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

**ポイント:**
- `{}` で複数行を書ける
- ブロックの最後の式が戻り値

---

## パターンに値を束縛

### データを持つEnumを処理

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

---

### 値を取り出す

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

**処理の流れ:**
1. `coin` が `Coin::Quarter` にマッチ
2. 内部の `UsState` を `state` 変数に束縛
3. `println!` で `state` を出力
4. `25` を返す

---

### 実行例

```rust
let coin = Coin::Quarter(UsState::Alaska);
let value = value_in_cents(coin);

// 出力: State quarter from Alaska!
// value = 25
```

---

## Option<T> を match で処理

### None の可能性がある値を安全に扱う

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);    // Some(6)
let none = plus_one(None);   // None
```

---

### 処理の流れ

#### ケース1: `Some(5)` の場合

```rust
let five = Some(5);

match five {
    None => None,           // マッチしない
    Some(i) => Some(i + 1), // ✅ マッチ！i = 5
}
// 結果: Some(6)
```

---

#### ケース2: `None` の場合

```rust
let none = None;

match none {
    None => None,           // ✅ マッチ！
    Some(i) => Some(i + 1), // マッチしない
}
// 結果: None
```

---

## 包括性チェック（Exhaustive Checking）

### ❌ パターンが足りないとエラー

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        // None のケースがない！
    }
}
```

**コンパイルエラー:**
```
error[E0004]: non-exhaustive patterns: `None` not covered
```

---

### ✅ 全パターンを網羅

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,           // ✅ 追加
        Some(i) => Some(i + 1),
    }
}
```

**利点:**
- コンパイラが処理漏れを防ぐ
- バグを事前に検出
- 安全性が保証される

---

## 包括パターン `_`

### 他の全てのケースをまとめて処理

```rust
let dice_roll = 9;

match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),  // 3と7以外の全て
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
```

**`_` の意味:**
- 「他の全て」にマッチ
- 値を束縛しない（変数名をつけない）

---

### 何もしないパターン

```rust
let dice_roll = 9;

match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),  // 何もしない
}
```

**`()` = ユニット値**
- 「何も返さない」を表す
- 他のパターンで何もしたくない時に使う

---

## 数値リテラルの型サフィックス

### `0u8` とは？

```rust
fn get_default() -> u8 {
    0u8  // 0 を u8 型として明示
}
```

**構造:**
```
0u8
│└─ 型（u8）
└── 値（0）
```

---

### 他の例

```rust
let a = 42i32;   // i32型の42
let b = 100u64;  // u64型の100
let c = 3.14f32; // f32型の3.14
let d = 2.0f64;  // f64型の2.0
```

---

### いつ使う？

#### ケース1: 型推論が難しい場合

```rust
let x = 0u8;  // u8 型を明示
```

#### ケース2: コードの意図を明確にしたい

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,    // 型推論でOK
        // ...
    }
}
```

**ほとんどの場合、型推論で十分**

---

## matchの実践パターン

### 1. 値の変換

```rust
enum Color {
    Red,
    Green,
    Blue,
}

fn color_to_rgb(color: Color) -> (u8, u8, u8) {
    match color {
        Color::Red => (255, 0, 0),
        Color::Green => (0, 255, 0),
        Color::Blue => (0, 0, 255),
    }
}
```

---

### 2. エラーハンドリング

```rust
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

let result = divide(10, 2);

match result {
    Some(value) => println!("結果: {}", value),
    None => println!("エラー: ゼロ除算"),
}
```

---

### 3. 状態遷移

```rust
enum State {
    Start,
    Processing,
    Done,
}

fn next_state(current: State) -> State {
    match current {
        State::Start => State::Processing,
        State::Processing => State::Done,
        State::Done => State::Start,
    }
}
```

---

## match vs if

### if: 真偽値で分岐

```rust
let x = 5;

if x > 3 {
    println!("大きい");
} else {
    println!("小さい");
}
```

**用途:** 条件（bool）で分岐

---

### match: パターンで分岐

```rust
let x = Some(5);

match x {
    Some(value) => println!("値: {}", value),
    None => println!("値なし"),
}
```

**用途:** パターンマッチング + 値の取り出し

---

## match は式

### 値を返す

```rust
let x = Some(5);

let result = match x {
    Some(value) => value + 1,
    None => 0,
};

println!("{}", result);  // 6
```

**ポイント:**
- `match` 全体が1つの式
- 各アームの値が戻り値になる
- 全アームの型が一致する必要がある

---

### 型の一致が必要

```rust
let x = Some(5);

// ❌ エラー！型が一致しない
let result = match x {
    Some(value) => value,      // i32
    None => "nothing",          // &str
};
```

```rust
// ✅ OK！型が一致
let result = match x {
    Some(value) => value.to_string(),  // String
    None => String::from("nothing"),   // String
};
```

---

## ネストしたmatch

### Option<Option<T>> を処理

```rust
fn unwrap_twice(x: Option<Option<i32>>) -> i32 {
    match x {
        Some(inner) => match inner {
            Some(value) => value,
            None => 0,
        },
        None => 0,
    }
}

let nested = Some(Some(5));
println!("{}", unwrap_twice(nested));  // 5
```

---

## まとめ

### matchの重要ポイント

| 概念 | 説明 |
|---|---|
| **パターンマッチング** | パターンに基づいて分岐 |
| **包括性チェック** | 全パターンを網羅する必要あり |
| **値の束縛** | パターン内で値を取り出せる |
| **式** | 値を返す（代入可能） |

---

### 包括性チェック

```
match = 全パターンを網羅する必要がある

利点:
- 処理漏れを防ぐ
- コンパイル時にチェック
- バグを事前に検出

ツール:
- _ パターンで「他の全て」をカバー
- () で「何もしない」を表現
```

---

### パターンの種類

```
1. リテラル     : Coin::Penny => ...
2. 変数束縛     : Some(x) => ...
3. 構造体分解   : Point { x, y } => ...
4. 包括パターン : _ => ...
5. 複数パターン : 1 | 2 | 3 => ...
```

---

### match vs if

```
if    = bool で分岐（条件）
match = パターンで分岐（構造）

使い分け:
- 真偽値で判定 → if
- Enumを処理  → match
- 値を取り出す → match
```
