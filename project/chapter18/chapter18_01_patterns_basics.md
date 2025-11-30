# Chapter 18-01: パターンマッチングの基礎

## パターンとは

**パターン**は、Rustにおける特別な構文で、値の構造にマッチさせるために使います。パターンと`match`式を組み合わせることで、プログラムの制御フローを強力に扱うことができます。

## パターンが使える場所

### 1. `match` アーム

最も一般的な使用場所です。

```rust
match value {
    Some(x) => println!("値: {}", x),
    None => println!("値なし"),
}
```

**特徴:**
- すべてのパターンを網羅する必要がある
- コンパイラが網羅性をチェックする

### 2. `if let` 式

```rust
let favorite_color: Option<&str> = None;
let is_tuesday = false;
let age: Result<u8, _> = "34".parse();

if let Some(color) = favorite_color {
    println!("好きな色: {}", color);
} else if is_tuesday {
    println!("火曜日は緑");
} else if let Ok(age) = age {
    if age > 30 {
        println!("紫を使います");
    } else {
        println!("オレンジを使います");
    }
} else {
    println!("青を使います");
}
```

**特徴:**
- 1つのパターンだけマッチさせる
- `else if` や `else` と組み合わせ可能
- 網羅性チェックなし

**制限:**
- `if let Ok(age) = age && age > 30` のような書き方はできない
- 新しい変数はブロック内でのみ有効

### 3. `while let` ループ

```rust
let mut stack = Vec::new();
stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
// 出力: 3, 2, 1
```

**特徴:**
- パターンがマッチし続ける限りループ
- マッチしなくなったら終了

### 4. `for` ループ

```rust
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{} は index {} にあります", value, index);
}
```

**パターン部分:**
```rust
for (index, value) in v.iter().enumerate() {
//  ^^^^^^^^^^^^^^ ← パターン（タプルを分解）
}
```

### 5. `let` 文

```rust
let x = 5;
//  ^ パターン

let (x, y, z) = (1, 2, 3);
//  ^^^^^^^^^ パターン（タプルを分解）

let Point { x, y } = Point { x: 0, y: 7 };
//  ^^^^^^^^^^^^^ パターン（構造体を分解）
```

### 6. 関数パラメータ

```rust
fn print_coordinates(&(x, y): &(i32, i32)) {
//                   ^^^^^^^ パターン
    println!("現在地: ({}, {})", x, y);
}

let point = (3, 5);
print_coordinates(&point);
```

**クロージャでも:**
```rust
let points = vec![Point { x: 0, y: 0 }, Point { x: 1, y: 5 }];

let sum_of_squares: i32 = points
    .iter()
    .map(|&Point { x, y }| x * x + y * y)
    //   ^^^^^^^^^^^^^^^ パターン
    .sum();
```

## 反駁可能性（Refutability）

パターンには2つの形式があります：

### 論駁不可能（Irrefutable）パターン

**常にマッチするパターン**

```rust
let x = 5;
//  ^ 常にマッチ

let (x, y) = (1, 2);
//  ^^^^^^ 常にマッチ

fn foo(x: i32) { }
//     ^ 常にマッチ
```

**使える場所:**
- `let` 文
- 関数パラメータ
- `for` ループ

### 論駁可能（Refutable）パターン

**マッチしない可能性があるパターン**

```rust
if let Some(x) = some_value {
//     ^^^^^^^ マッチしないかも（None の場合）
    println!("{}", x);
}

match value {
    Some(x) => println!("{}", x),
    //^^^^^^ マッチしないかも
    None => println!("なし"),
}
```

**使える場所:**
- `if let`
- `while let`
- `match` アーム

### コンパイラの警告

```rust
// ⚠️ 警告: 論駁不可能なパターン
if let x = 5 {
    println!("{}", x);
}
// help: consider replacing the `if let` with a `let`
```

**近年のRust:**
- 昔: エラー（コンパイルできない）
- 今: 警告（コンパイルできるが推奨されない）

### エラーになる例

```rust
// ❌ エラー: 論駁可能なパターン
let Some(x) = some_value;
//  ^^^^^^^ マッチしない可能性がある
```

**修正:**
```rust
// ✅ if let を使う
if let Some(x) = some_value {
    println!("{}", x);
}

// ✅ または match を使う
match some_value {
    Some(x) => println!("{}", x),
    None => println!("なし"),
}
```

## まとめ

| 場所 | 使用例 | 反駁可能性 | 網羅性チェック |
|------|--------|----------|--------------|
| `match` | `match x { ... }` | 可能 | あり |
| `if let` | `if let Some(x) = ...` | 可能 | なし |
| `while let` | `while let Some(x) = ...` | 可能 | なし |
| `for` | `for (i, v) in ...` | 不可能 | - |
| `let` | `let (x, y) = ...` | 不可能 | - |
| 関数引数 | `fn foo(x: i32)` | 不可能 | - |

**重要な原則:**
- 論駁不可能なパターンは常にマッチする場所で使う
- 論駁可能なパターンはマッチしない可能性を扱える場所で使う
- コンパイラが適切な使い方をチェックしてくれる
