# Chapter 18-02: パターンの記法

## パターンの種類

### 1. リテラルにマッチ

```rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

### 2. 名前付き変数

```rust
let x = Some(5);
let y = 10;

match x {
    Some(50) => println!("Got 50"),
    Some(y) => println!("Matched, y = {:?}", y),
    //   ^ 新しい変数 y（外側の y を隠す）
    _ => println!("Default case, x = {:?}", x),
}

println!("at the end: x = {:?}, y = {:?}", x, y);
// x = Some(5), y = 10（外側の y は影響なし）
```

### 3. 複数のパターン（`|`）

```rust
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    //  ^ OR
    3 => println!("three"),
    _ => println!("anything"),
}
```

### 4. 範囲にマッチ（`..=`）

```rust
let x = 5;

match x {
    1..=5 => println!("one through five"),
    //^^^^ 1から5まで（両端含む）
    _ => println!("something else"),
}
```

**文字の範囲:**
```rust
let x = 'c';

match x {
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("something else"),
}
```

### 5. 構造体の分解

```rust
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 7 };

match p {
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x: 0, y } => println!("On the y axis at {}", y),
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
}
```

**省略記法:**
```rust
let Point { x, y } = p;
// Point { x: x, y: y } と同じ
```

**一部だけマッチ:**
```rust
match p {
    Point { x, .. } => println!("x is {}", x),
    //         ^^ y を無視
}
```

### 6. 列挙型の分解

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let msg = Message::ChangeColor(0, 160, 255);

match msg {
    Message::Quit => {
        println!("Quit");
    }
    Message::Move { x, y } => {
        println!("Move to x: {}, y: {}", x, y);
    }
    Message::Write(text) => {
        println!("Text message: {}", text);
    }
    Message::ChangeColor(r, g, b) => {
        println!("Change color to RGB({}, {}, {})", r, g, b);
    }
}
```

**ネストした列挙型:**
```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

match msg {
    Message::ChangeColor(Color::Rgb(r, g, b)) => {
        println!("RGB({}, {}, {})", r, g, b);
    }
    Message::ChangeColor(Color::Hsv(h, s, v)) => {
        println!("HSV({}, {}, {})", h, s, v);
    }
    _ => {}
}
```

### 7. タプルの分解

```rust
let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
```

### 8. 値を無視する（`_`）

**値全体を無視:**
```rust
fn foo(_: i32, y: i32) {
    println!("y = {}", y);
}

foo(3, 4);  // 3 は無視される
```

**名前の一部を無視:**
```rust
let _x = 5;  // 未使用警告が出ない
let y = 10;  // ⚠️ 未使用警告
```

**構造体の一部を無視:**
```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {}, {}, {}", first, third, fifth);
    }
}
```

### 9. 残りを無視（`..`）

```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
    //         ^^ y と z を無視
}
```

**タプルで:**
```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, .., last) => {
        println!("first: {}, last: {}", first, last);
    }
}
```

**曖昧さはエラー:**
```rust
// ❌ エラー
match numbers {
    (.., second, ..) => {
        //         ^^ どこまでが .. か不明
    }
}
```

### 10. 参照のパターン

#### `&` パターン（参照を外す）

```rust
let points = vec![
    Point { x: 0, y: 0 },
    Point { x: 1, y: 5 },
    Point { x: 10, y: -3 },
];

let sum_of_squares: i32 = points
    .iter()  // &Point を返す
    .map(|&Point { x, y }| x * x + y * y)
    //   ^ 参照を外す
    .sum();
```

**2つの書き方:**
```rust
// 1. 明示的な参照外し
.map(|&Point { x, y }| x * x + y * y)

// 2. 自動参照外し
.map(|Point { x, y }| x * x + y * y)
```

#### `ref` パターン（参照を作る）

```rust
let robot_name = Some(String::from("Bors"));

match robot_name {
    Some(ref name) => println!("Found: {}", name),
    //   ^^^ 参照を作る（所有権を奪わない）
    None => (),
}

// robot_name はまだ使える
println!("robot_name: {:?}", robot_name);
```

**`ref` なしだと:**
```rust
match robot_name {
    Some(name) => println!("Found: {}", name),
    //   ^^^^ 所有権を奪う
    None => (),
}

// ❌ エラー: robot_name は使えない
// println!("robot_name: {:?}", robot_name);
```

#### `&` vs `ref` まとめ

| 場所 | `&x` の意味 | 参照を作る |
|------|-----------|----------|
| **普通の式** | 参照を作る | `&x` |
| **パターン** | 参照を外す | `ref x` |

```rust
// 普通の式
let x = 5;
let y = &x;  // 参照を作る

// パターン
let x = &5;
match x {
    &n => {}  // 参照を外す
}

let x = 5;
match x {
    ref n => {}  // 参照を作る
}
```

## まとめ

| パターン | 例 | 用途 |
|---------|-----|------|
| リテラル | `1`, `'a'` | 特定の値にマッチ |
| 変数 | `x`, `name` | 値を変数に束縛 |
| OR | `1 \| 2` | 複数パターン |
| 範囲 | `1..=5` | 範囲にマッチ |
| 構造体分解 | `Point { x, y }` | 構造体を分解 |
| 列挙型分解 | `Some(x)` | 列挙型を分解 |
| タプル分解 | `(x, y, z)` | タプルを分解 |
| `_` | `_`, `_x` | 値を無視 |
| `..` | `Point { x, .. }` | 残りを無視 |
| `&` | `&Point { x, y }` | 参照を外す |
| `ref` | `ref name` | 参照を作る |
