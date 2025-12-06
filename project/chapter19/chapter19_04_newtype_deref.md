# 第19章 Part 4: Newtypeパターンと Deref トレイト

## Orphan Rule（孤児ルール）

### 問題

外部の型に外部のトレイトを実装することは**できない**:

```rust
use std::fmt::Display;

// ❌ エラー！Vec<T> も Display も外部クレートで定義されている
impl Display for Vec<String> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.join(", "))
    }
}
```

**エラー**: orphan rule（孤児ルール）違反

### Orphan Rule とは？

トレイトを実装するには、以下のいずれかが**自分のクレート内で定義**されている必要がある:
1. トレイト自体
2. 型自体

| トレイト | 型 | 実装可能？ |
|---------|-----|----------|
| 自作 | 自作 | ✅ Yes |
| 自作 | 外部 | ✅ Yes |
| 外部 | 自作 | ✅ Yes |
| 外部 | 外部 | ❌ No（orphan rule） |

---

## Newtypeパターン

### 解決策

外部の型を**新しい型でラップ**する:

```rust
use std::fmt;

// Wrapper は自分のクレートで定義した型
struct Wrapper(Vec<String>);  // タプル構造体

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
        // self.0 でタプルの最初の要素（Vec<String>）にアクセス
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);  // w = [hello, world]
}
```

### タプル構造体

```rust
struct Wrapper(Vec<String>);
//             ^^^^^^^^^^^
//             タプルのように値を持つ

let w = Wrapper(vec![...]);
let inner = w.0;  // .0 で最初の要素にアクセス
```

### Newtypeパターンの利点

1. **型安全性**: 異なる意味を持つ同じ型を区別できる
   ```rust
   struct Meters(f64);
   struct Seconds(f64);

   fn distance(m: Meters) { ... }

   let time = Seconds(5.0);
   distance(time);  // ❌ コンパイルエラー！型が違う
   ```

2. **カプセル化**: 内部実装を隠せる
3. **トレイト実装**: 外部の型に外部のトレイトを実装できる

---

## Deref トレイト

### 問題: Wrapper で Vec のメソッドが使えない

```rust
let w = Wrapper(vec![String::from("hello")]);
w.push(String::from("world"));  // ❌ エラー！Wrapper には push がない
```

### 解決策: Deref トレイトを実装

```rust
use std::ops::Deref;

struct Wrapper(Vec<String>);

impl Deref for Wrapper {
    type Target = Vec<String>;  // 参照外しの結果の型

    fn deref(&self) -> &Self::Target {
        &self.0  // 内部の Vec<String> への参照を返す
    }
}

fn main() {
    let mut w = Wrapper(vec![String::from("hello")]);

    // Deref により &Wrapper が自動的に &Vec<String> に変換される
    w.push(String::from("world"));  // ✅ Vec のメソッドが使える！
    println!("Length: {}", w.len());  // ✅ len() も使える
}
```

### Deref の仕組み

```rust
w.push(String::from("world"));

// コンパイラが以下のように展開する:
// 1. w.deref() を呼び出して &Vec<String> を取得
// 2. &Vec<String> の push メソッドを呼び出す
```

### type Target の意味

```rust
impl Deref for Wrapper {
    type Target = Vec<String>;  // ← これが重要！
    //            ^^^^^^^^^^^
    //            Deref で変換される先の型

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

- `type Target = Vec<String>` → `&Wrapper` は `&Vec<String>` に変換される
- 結果: `Vec<String>` の**すべてのメソッド**が `Wrapper` でも使える

### Deref の動作確認

```rust
use std::ops::Deref;

struct Wrapper(Vec<String>);

impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let w = Wrapper(vec![
        String::from("hello"),
        String::from("world")
    ]);

    // 以下すべて Vec<String> のメソッド
    println!("Length: {}", w.len());        // ✅
    println!("First: {}", &w[0]);           // ✅
    println!("Contains: {}", w.contains(&String::from("hello")));  // ✅

    // イテレータも使える
    for s in w.iter() {
        println!("{}", s);
    }
}
```

---

## DerefMut トレイト

可変参照の場合は `DerefMut` を実装:

```rust
use std::ops::{Deref, DerefMut};

impl DerefMut for Wrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let mut w = Wrapper(vec![String::from("hello")]);

    // 可変メソッドも使える
    w.push(String::from("world"));  // ✅
    w[0] = String::from("hi");      // ✅
}
```

---

## まとめ

### Newtypeパターン
- **目的**: 外部の型に外部のトレイトを実装する
- **方法**: タプル構造体でラップする
- **アクセス**: `.0` で内部の値にアクセス
- **利点**: 型安全性、カプセル化、orphan rule 回避

### Deref トレイト
- **目的**: 内部の型のメソッドを委譲する
- **仕組み**: `&Wrapper` → `&Vec<String>` に自動変換
- **結果**: 内部型のすべてのメソッドが使える
- **キーポイント**: `type Target` で変換先の型を指定

### 組み合わせの効果
Newtypeパターン + Deref = **型安全性を保ちつつ、利便性も確保**できる！
