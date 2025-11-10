# Chapter 6: Enum とパターンマッチング - Q&A

## Q1: Enum と Struct の違い

**質問:** Enum と Struct はどう使い分けるの？

**回答:** **関係性の違い**で使い分けます

### Struct = AND関係

```rust
struct User {
    username: String,  // AND
    email: String,     // AND
    active: bool,      // AND
}
```

**全てのフィールドを同時に持つ**

---

### Enum = OR関係

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),  // OR
    V6(String),          // OR
}
```

**いずれか1つのバリアントを持つ**

---

### なぜ Enum が必要？

**質問:** `struct IpAddr {v4:(u8, u8, u8, u8), v6:String}` ではいけないの？

**回答:** 使えないことはないけど無駄とめんどくささがある

#### ❌ Struct で表現

```rust
struct IpAddr {
    v4: (u8, u8, u8, u8),
    v6: String,
}

let addr = IpAddr {
    v4: (127, 0, 0, 1),
    v6: String::from(""),  // 使わないのに必要
};

// どっちを使うか判定が必要
if addr.v6.is_empty() {
    // v4 を使う
} else {
    // v6 を使う
}
```

**問題点:**
- 両方のフィールドが必要（無駄）
- どちらを使うか判定が必要（めんどくさい）
- 意味的に間違っている（両方持つことはない）

---

#### ✅ Enum で表現

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
// V6 は存在しない（無駄がない）

// match で自然に処理
match home {
    IpAddr::V4(a, b, c, d) => println!("IPv4: {}.{}.{}.{}", a, b, c, d),
    IpAddr::V6(addr) => println!("IPv6: {}", addr),
}
```

**利点:**
- 必要なデータだけ持つ
- 型システムで「どちらか1つ」を保証
- 意味が明確

---

### まとめ

```
Struct = 複数のデータを同時に持つ（AND）
Enum   = 複数の選択肢から1つ（OR）

使い分け:
- ユーザー情報（名前AND年齢ANDメール） → Struct
- IPアドレス（V4 OR V6）              → Enum
```

---

## Q2: Prelude とは？

**質問:** 「Option<T>は初期化処理(prelude)にさえ含まれています。つまり、明示的にスコープに導入する必要がないのです。」どゆこと？

**回答:** **自動的にインポートされる型や関数のセット**です

---

### Prelude に含まれるもの

| 型/トレイト | 説明 |
|---|---|
| `Option<T>` | `Some`, `None` |
| `Result<T, E>` | `Ok`, `Err` |
| `Vec<T>` | ベクタ型 |
| `String` | 文字列型 |
| `println!` | 出力マクロ |
| `Clone` | クローントレイト |

---

### Prelude に含まれないもの

```rust
// ❌ 明示的にインポートが必要
use std::collections::HashMap;

let mut map = HashMap::new();
```

---

### なぜ Prelude があるのか？

**よく使う型を毎回 `use` で書くのは面倒だから**

#### Prelude がなかったら...

```rust
use std::option::Option;
use std::option::Option::Some;
use std::option::Option::None;
use std::result::Result;
use std::string::String;
use std::vec::Vec;

// めんどくさい！
```

#### Prelude のおかげで...

```rust
// そのまま使える！
let x = Some(5);
let y: Option<i32> = None;
let v = Vec::new();
let s = String::from("hello");
```

---

### まとめ

```
Prelude = 自動インポートされる便利な型・関数

含まれる: Option, Result, Vec, String, println! など
含まれない: HashMap, File, BTreeMap など

理由: よく使うものだけ自動インポート
```

---

## Q3: None と null の違い

**質問:** None と null って何が違うの？

**回答:** **None は「無いことを表す値」、null は「ほんとに何もない」**

---

### null の問題（C/Java/Python等）

```python
# Python
def get_user(id):
    if id == 0:
        return None
    return User(id)

user = get_user(0)
print(user.name)  # ❌ 実行時エラー！
# AttributeError: 'NoneType' object has no attribute 'name'
```

**問題点:**
- 型システムの外側
- `None` チェック忘れでクラッシュ
- コンパイル時に検出できない

---

### None の利点（Rust）

```rust
fn get_user(id: u32) -> Option<User> {
    if id == 0 {
        None
    } else {
        Some(User { id })
    }
}

let user = get_user(0);
// println!("{}", user.id);  // ❌ コンパイルエラー！
// Option<User> は User ではない

// match で安全に処理
match user {
    Some(u) => println!("{}", u.id),
    None => println!("ユーザーなし"),
}
```

**利点:**
- 型システムの内側（`Option<T>` 型）
- コンパイラが強制チェック
- 実行前に検出できる

---

### 比較表

| 項目 | null | None |
|---|---|---|
| **本質** | 何もない | 無いことを表す値 |
| **型安全性** | ❌ なし | ✅ あり |
| **コンパイラチェック** | ❌ なし | ✅ あり |
| **値の種類** | 特別な値 | Enum の1バリアント |
| **使用前チェック** | 任意（忘れる） | 強制（matchで処理） |

---

### まとめ

```
null = ほんとに何もない（型システムの外）
None = 無いことを表す値（型システムの内）

Rust:
- None は Option<T> の一部
- 使用前に必ず存在チェックが必要
- nullポインタエラーが起きない
```

---

## Q4: Option<T> で None に型注釈が必要な理由

**質問:** なぜ `None` には型注釈が必要で、`Some(5)` には不要なの？

**回答:** **値から型を推論できるかどうか**の違いです

---

### Some(5) の場合

```rust
let x = Some(5);  // Option<i32> と推論される
```

**推論の流れ:**
1. `5` は `i32` 型
2. `Some(5)` は `Option<i32>` 型
3. 型注釈不要 ✅

---

### None の場合

```rust
let x = None;  // ❌ エラー！型が分からない
// error: type annotations needed
```

**問題:**
1. `None` は値を持たない
2. `T` が何か推論できない
3. 型注釈が必要

---

### 解決策

```rust
let x: Option<i32> = None;  // ✅ 型を明示
```

---

### 実際の例

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,           // ✅ 戻り値の型から推論される
        Some(i) => Some(i + 1),
    }
}

// 関数の引数や戻り値で型が明確なら注釈不要
let result = plus_one(Some(5));  // Option<i32>
```

---

### まとめ

```
Some(値) → 値から型を推論できる
None     → 値がないので型を推論できない

対処法:
1. 型注釈: let x: Option<i32> = None;
2. 文脈から推論: 関数の戻り値など
```

---

## Q5: パターンマッチングで値を取り出す流れ

**質問:** `value_in_cents(Coin::Quarter(UsState::Alaska))` を呼び出すと、どういう流れで処理される？

**回答:** 以下の流れです

---

### コード

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

---

### 実行の流れ

```rust
value_in_cents(Coin::Quarter(UsState::Alaska));
```

**ステップ1:** `coin` に `Coin::Quarter(UsState::Alaska)` が渡される

**ステップ2:** match で各パターンをチェック
- `Coin::Penny` → マッチしない
- `Coin::Nickel` → マッチしない
- `Coin::Dime` → マッチしない
- `Coin::Quarter(state)` → ✅ **マッチ！**

**ステップ3:** パターン束縛
- `UsState::Alaska` が `state` 変数に束縛される

**ステップ4:** ブロック内の処理
```rust
println!("State quarter from {:?}!", state);
// 出力: State quarter from Alaska!
```

**ステップ5:** 戻り値
```rust
25  // ← この値が関数の戻り値
```

---

### 図解

```
Coin::Quarter(UsState::Alaska)
       ↓
match coin {
    Coin::Quarter(state) ← マッチ！
                 ↓
         state = UsState::Alaska
                 ↓
         println!("State quarter from {:?}!", state)
                 ↓
              出力: State quarter from Alaska!
                 ↓
              戻り値: 25
}
```

---

### まとめ

```
1. match で Coin::Quarter パターンにマッチ
2. 内部の UsState を state 変数に束縛
3. println! で state を出力
4. 25 を戻り値として返す
```

---

## Q6: 数値リテラルの型サフィックス

**質問:** `0u8` って何？

**回答:** **数値リテラルに型を明示的に指定する記法**です

---

### 構造

```
0u8
│└─ 型（u8 = 8ビット符号なし整数）
└── 値（0）
```

---

### 他の例

| リテラル | 型 | 説明 |
|---|---|---|
| `0u8` | `u8` | 8ビット符号なし整数 |
| `42i32` | `i32` | 32ビット符号付き整数 |
| `100u64` | `u64` | 64ビット符号なし整数 |
| `3.14f32` | `f32` | 32ビット浮動小数点数 |
| `2.0f64` | `f64` | 64ビット浮動小数点数 |

---

### なぜ使う？

#### ケース1: 型推論が難しい場合

```rust
let x = 0u8;  // u8 型を明示
```

#### ケース2: コードの意図を明確にしたい

```rust
fn get_default() -> u8 {
    0u8  // u8 型であることを明示
}
```

---

### 実用例

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,    // 型推論で u8
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => 25,
    }
}
```

**ほとんどの場合、型推論で十分なので省略OK**

---

### まとめ

```
0u8 = 値0を u8型として明示

使う場面:
1. 型推論が難しい時
2. 明示的に型を示したい時
3. コンパイラエラーを避ける時

ほとんどの場合は型推論で十分
```

---

## Q7: if let と Python の if の違い

**質問:** Rust の `if let` と Python の `if` って一緒？

**回答:** **全く違います！**

---

### Python の if: 条件分岐

```python
x = 5

if x > 3:
    print("大きい")
else:
    print("小さい")
```

**やること:** 真偽値（True/False）で分岐

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

### まとめ

```
Python の if        = 真偽値で分岐
Rust の if let      = パターンマッチで値を取り出しつつ分岐
Rust の普通の if    = Python の if と同じ（真偽値で分岐）

if let は Python にはない Rust 特有の機能
```

---

## Q8: if let の包括性チェック

**質問:** `if let` は包括性チェックは働かないですよね？

**回答:** **その通り！** `if let` は包括性チェックが働きません

---

### match: 包括性チェック あり

```rust
let x = Some(5);

match x {
    Some(value) => println!("値: {}", value),
    // None がない！
}
// ❌ コンパイルエラー: non-exhaustive patterns
```

**全パターンを網羅する必要がある**

---

### if let: 包括性チェック なし

```rust
let x = Some(5);

if let Some(value) = x {
    println!("値: {}", value);
}
// None のケースがなくても ✅ OK！
```

**1つのパターンだけ処理できる**

---

### 比較表

| 項目 | match | if let |
|---|---|---|
| **包括性チェック** | ✅ 働く | ❌ 働かない |
| **パターン数** | 全て必須 | 1つでOK |
| **用途** | 全パターン処理 | 1パターン処理 |

---

### まとめ

```
match    = 全パターン網羅（包括性チェック あり）
if let   = 部分的に処理（包括性チェック なし）

if let の利点:
- 1パターンだけ処理したい時に簡潔
- 他のケースは無視してOK
- 包括性チェックから解放される
```

---

## 重要な概念のまとめ

### Enum vs Struct

```
Struct = AND関係（全てのフィールドを持つ）
Enum   = OR関係（1つのバリアントを持つ）
```

---

### Prelude

```
自動インポートされる便利な型・関数のセット
Option, Result, Vec, String など
```

---

### None vs null

```
null = 何もない（型システムの外）
None = 無いことを表す値（型システムの内）
```

---

### Option<T> の型推論

```
Some(5) → 値から型を推論できる
None    → 値がないので型注釈が必要
```

---

### match vs if let

```
match    = 全パターン処理（包括性チェック あり）
if let   = 1パターン処理（包括性チェック なし）
普通の if = 条件分岐（bool）
```

---

### パターンマッチングの流れ

```
1. パターンにマッチ
2. 値を変数に束縛
3. 処理を実行
4. 値を返す
```
