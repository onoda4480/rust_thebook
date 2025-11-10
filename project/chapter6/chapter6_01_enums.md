# Chapter 6-1: Enumの定義と使用

## Enumとは？

**複数の選択肢の中から1つを選ぶ型**

---

## Enum vs Struct

### 関係性の違い

| 概念 | 関係 | 例 |
|---|---|---|
| **Struct** | AND関係 | 全てのフィールドを持つ |
| **Enum** | OR関係 | いずれか1つのバリアントを持つ |

---

### Structの例（AND関係）

```rust
struct User {
    username: String,  // AND
    email: String,     // AND
    active: bool,      // AND
}

let user = User {
    username: String::from("user1"),
    email: String::from("user@example.com"),
    active: true,
};
// 全てのフィールドが必要
```

**特徴:** 全てのフィールドを同時に持つ

---

### Enumの例（OR関係）

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),  // OR
    V6(String),          // OR
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

**特徴:** いずれか1つのバリアントだけを持つ

---

## なぜEnumを使うのか？

### ❌ Structで表現すると...

```rust
struct IpAddr {
    v4: (u8, u8, u8, u8),
    v6: String,
}

let addr = IpAddr {
    v4: (127, 0, 0, 1),
    v6: String::from(""),  // 使わないのに必要！
};
```

**問題点:**
- 両方のフィールドが必要（無駄）
- どちらを使うか判定が必要（めんどくさい）
- 意味的に間違っている（両方持つことはない）

---

### ✅ Enumで表現すると...

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
// V6は存在しない（無駄がない）
```

**利点:**
- 必要なデータだけ持つ
- 型システムで「どちらか1つ」を保証
- 意味が明確

---

## Enumの定義

### 基本構文

```rust
enum Message {
    Quit,                       // データなし
    Move { x: i32, y: i32 },    // 名前付きフィールド
    Write(String),              // タプル型
    ChangeColor(i32, i32, i32), // 複数の値
}
```

**ポイント:**
- 各バリアントは異なる型とデータ量を持てる
- バリアントごとに適切なデータ構造を選べる

---

## バリアントの種類

### 1. データなし

```rust
enum Message {
    Quit,
}

let msg = Message::Quit;
```

**用途:** シンプルな状態や命令

---

### 2. タプル型（名前なしフィールド）

```rust
enum Message {
    Write(String),
}

let msg = Message::Write(String::from("hello"));
```

**用途:** 単一または複数の値を持つ

---

### 3. 構造体型（名前付きフィールド）

```rust
enum Message {
    Move { x: i32, y: i32 },
}

let msg = Message::Move { x: 10, y: 20 };
```

**用途:** 複数の関連データを持つ

---

## Enumにメソッドを実装

### impl ブロック

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // メソッドの実装
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

**ポイント:** Structと同じように `impl` でメソッドを追加できる

---

## Option<T> Enum

### 定義（標準ライブラリ）

```rust
enum Option<T> {
    Some(T),
    None,
}
```

**特徴:**
- **Prelude** に含まれる（明示的なインポート不要）
- ジェネリック型 `<T>` を使用
- Rustの「null安全性」の核心

---

### Preludeとは？

**自動的にインポートされる型や関数のセット**

| 含まれるもの | 例 |
|---|---|
| `Option<T>` | `Some`, `None` |
| `Result<T, E>` | `Ok`, `Err` |
| `Vec<T>` | ベクタ型 |
| `String` | 文字列型 |

**使わなくていいもの:**
```rust
// ❌ 不要
use std::option::Option;

// ✅ そのまま使える
let x = Some(5);
let y: Option<i32> = None;
```

---

### Option<T> の使用例

```rust
let some_number = Some(5);          // Option<i32>
let some_string = Some("a string"); // Option<&str>

let absent_number: Option<i32> = None;  // 型注釈が必要
```

**なぜ `None` に型注釈が必要？**
- `Some(5)` → 値 `5` から `i32` を推論できる
- `None` → 値がないので型を推論できない

---

## None vs null

### null（C/Java/Python等）

```
null = 「何もない」を表す特別な値
問題点:
- 型システムの外側
- nullチェック忘れでクラッシュ
- Billion Dollar Mistake
```

### None（Rust）

```
None = 「無いことを表す値」
利点:
- 型システムの内側（Option<T>型）
- コンパイラが強制チェック
- 安全性が保証される
```

---

### 比較表

| 項目 | null | None |
|---|---|---|
| **型安全性** | ❌ なし | ✅ あり |
| **コンパイラチェック** | ❌ なし | ✅ あり |
| **値の種類** | 特別な値 | Enum の1バリアント |
| **使用前チェック** | 任意（忘れる） | 強制（matchで処理） |

---

### 実例で比較

#### Python（null的な None）

```python
def divide(a, b):
    if b == 0:
        return None
    return a / b

result = divide(10, 0)
print(result + 1)  # ❌ 実行時エラー！（Noneに+はできない）
```

---

#### Rust（Option<T>）

```rust
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

let result = divide(10, 0);
// println!("{}", result + 1);  // ❌ コンパイルエラー！
// Option<i32> と i32 は違う型
```

---

## Option<T> と T は別の型

### 重要な概念

```rust
let x: i32 = 5;
let y: Option<i32> = Some(5);

// let sum = x + y;  // ❌ エラー！型が違う
```

**理由:**
- `i32` は常に有効な値
- `Option<i32>` は「値があるかもしれない」
- **明示的に変換が必要** → 安全性が保証される

---

### Option<T> から T を取り出す方法

#### 1. `match` で処理（推奨）

```rust
let x: Option<i32> = Some(5);

match x {
    Some(value) => println!("値: {}", value),
    None => println!("値なし"),
}
```

---

#### 2. `if let` で簡潔に

```rust
let x: Option<i32> = Some(5);

if let Some(value) = x {
    println!("値: {}", value);
}
```

---

#### 3. `unwrap()` で強制取得（危険）

```rust
let x: Option<i32> = Some(5);
let value = x.unwrap();  // Some なら値を返す、None ならパニック
```

**⚠️ 注意:** `None` の時にパニックする

---

#### 4. `unwrap_or()` でデフォルト値

```rust
let x: Option<i32> = None;
let value = x.unwrap_or(0);  // None なら 0 を返す
```

---

#### 5. `map()` で変換

```rust
let x: Option<i32> = Some(5);
let y = x.map(|n| n + 1);  // Some(6)
```

---

## Enumの実践例

### コイン列挙型

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),  // データを持つバリアント
}
```

---

### 使用例

```rust
let coin = Coin::Quarter(UsState::Alaska);

// matchで処理（次のセクションで詳しく）
match coin {
    Coin::Quarter(state) => {
        println!("State quarter from {:?}!", state);
    }
    _ => {}
}
```

---

## まとめ

### Enumの重要ポイント

| 概念 | 説明 |
|---|---|
| **OR関係** | いずれか1つのバリアントを持つ |
| **バリアント** | それぞれ異なる型・データ量を持てる |
| **メソッド** | `impl` でメソッドを追加可能 |
| **型安全性** | コンパイラが正しい使用を強制 |

---

### Struct vs Enum

```
Struct = AND関係（全てのフィールドを持つ）
Enum   = OR関係（1つのバリアントを持つ）

使い分け:
- 複数のデータを同時に持つ → Struct
- 複数の選択肢から1つ → Enum
```

---

### Option<T> の重要性

```
null の問題を型システムで解決

Option<T> = 値があるかもしれない
T         = 値が必ず存在する

違いを型で表現 → コンパイラがチェック → 安全
```

---

### None vs null

```
null = ほんとに何もない（型システムの外）
None = 無いことを表す値（型システムの内）

Rust は None を Option<T> の一部として扱う
→ 使用前に必ず存在チェックが必要
→ nullポインタエラーが起きない
```
