# Chapter 5-1: 構造体の定義と使用

## 構造体とは？

**関連するデータをまとめて扱う型**

---

## 構造体の定義

### 基本構文

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

**構成要素:**
- `struct` キーワード
- 構造体名（大文字で始める）
- フィールド（名前と型）

---

## インスタンスの作成

### 基本的な作成

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

**ポイント:**
- 全てのフィールドに値を設定
- 順序は定義と違ってもOK

---

## フィールドへのアクセス

### ドット記法

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

println!("Email: {}", user1.email);
println!("Username: {}", user1.username);
```

### 可変インスタンス

```rust
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```

**重要:** インスタンス全体が可変か不変か（一部だけ可変はできない）

---

## フィールド初期化省略記法

### Before: 冗長な書き方

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

### After: 省略記法

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,      // email: email と同じ
        username,   // username: username と同じ
        active: true,
        sign_in_count: 1,
    }
}
```

**条件:** 変数名とフィールド名が同じ時

---

## 構造体更新記法

### Before: 全フィールドを書く

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: user1.username,
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
```

### After: `..` 記法

```rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1  // 残りのフィールドはuser1から
};
```

---

### 所有権のムーブに注意！

**重要:** `String` などのヒープデータは**ムーブ**される

#### `..` 記法を使った場合

```rust
let user1 = User {
    email: String::from("user1@example.com"),
    username: String::from("user1"),
    active: true,
    sign_in_count: 1,
};

let user2 = User {
    email: String::from("user2@example.com"),
    ..user1  // username がムーブ！
};

// println!("{}", user1.username);  // ❌ エラー！ムーブ済み
println!("{}", user1.active);       // ✅ OK（Copy型）
```

#### 直接代入でも同じ！

```rust
let user1 = User {
    email: String::from("user1@example.com"),
    username: String::from("user1"),
    active: true,
    sign_in_count: 1,
};

let user2 = User {
    email: String::from("another@example.com"),
    username: user1.username,  // ← ここでムーブ！
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};

// println!("{}", user1.username);  // ❌ エラー！ムーブ済み
println!("{}", user1.email);        // ✅ OK（emailはまだ持っている）
println!("{}", user1.active);       // ✅ OK（Copy型）
```

**なぜ？** `username: user1.username` は所有権を移動している

---

### フィールドごとの挙動

| フィールド | 型 | Copy? | 挙動 |
|---|---|---|---|
| `username` | `String` | ❌ | **ムーブ** |
| `email` | `String` | ❌ | **ムーブ** |
| `active` | `bool` | ✅ | コピー |
| `sign_in_count` | `u64` | ✅ | コピー |

---

### ムーブを避ける方法

#### 方法1: `clone()` を使う（推奨）

```rust
let user1 = User {
    email: String::from("user1@example.com"),
    username: String::from("user1"),
    active: true,
    sign_in_count: 1,
};

let user2 = User {
    email: String::from("user2@example.com"),
    username: user1.username.clone(),  // クローン！
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};

println!("{}", user1.username);  // ✅ OK！
println!("{}", user2.username);  // ✅ OK！
```

#### 方法2: 構造体全体を `clone()`

```rust
#[derive(Clone)]  // Clone トレイトを追加
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

let user1 = User {
    email: String::from("user1@example.com"),
    username: String::from("user1"),
    active: true,
    sign_in_count: 1,
};

let user2 = User {
    email: String::from("user2@example.com"),
    ..user1.clone()  // user1 全体をクローン
};

println!("{}", user1.username);  // ✅ OK！
```

#### 方法3: 参照を使う（読み取り専用）

```rust
let user1 = User {
    email: String::from("user1@example.com"),
    username: String::from("user1"),
    active: true,
    sign_in_count: 1,
};

// 新しい User は作らず、参照だけ持つ
let user1_ref = &user1;

println!("{}", user1.username);      // ✅ OK
println!("{}", user1_ref.username);  // ✅ OK
```

---

### まとめ：構造体更新記法と所有権

```
直接代入も .. 記法も、String型フィールドはムーブする！

避ける方法:
1. clone() でコピー（メモリコストあり）
2. 参照を使う（読み取り専用）
3. 構造体全体に Clone トレイトを付ける
```

---

## タプル構造体

### 定義

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
```

**特徴:**
- フィールド名がない
- 型に意味を持たせる

### 使用例

```rust
let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

// アクセスはインデックス
println!("Black: {}, {}, {}", black.0, black.1, black.2);

// 型が違うので混同しない
// let wrong: Color = origin;  // ❌ エラー
```

### タプルとの違い

```rust
// タプル: 型が同じなら互換性あり
let point: (i32, i32, i32) = (10, 20, 30);
let color: (i32, i32, i32) = (255, 0, 0);
let mixed = point;  // OK（でも意味的におかしい）

// タプル構造体: 型が違うので互換性なし
struct Point(i32, i32, i32);
struct Color(i32, i32, i32);

let point = Point(10, 20, 30);
let color = Color(255, 0, 0);
// let mixed: Point = color;  // ❌ エラー
```

---

## ユニット様構造体

### 定義

```rust
struct AlwaysEqual;
```

**特徴:**
- フィールドがない
- トレイト実装に使う

### 使用例

```rust
struct AlwaysEqual;

let subject = AlwaysEqual;

// 後でトレイトを実装できる
impl SomeTrait for AlwaysEqual {
    // ...
}
```

---

## Debug トレイト

### 問題: そのままは表示できない

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

let rect = Rectangle { width: 30, height: 50 };
// println!("{}", rect);  // ❌ エラー
```

### 解決策1: `{:?}` を使う

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

let rect = Rectangle { width: 30, height: 50 };
println!("{:?}", rect);  // Rectangle { width: 30, height: 50 }
```

### 解決策2: `{:#?}` で整形

```rust
println!("{:#?}", rect);
// Rectangle {
//     width: 30,
//     height: 50,
// }
```

### dbg! マクロ

```rust
let scale = 2;
let rect = Rectangle {
    width: dbg!(30 * scale),  // 式の値とファイル・行番号を出力
    height: 50,
};

dbg!(&rect);  // rectの内容を出力（所有権は維持）
```

---

## 構造体の種類まとめ

### 1. 通常の構造体

```rust
struct User {
    username: String,
    email: String,
    active: bool,
}
```

**用途:** 名前付きフィールドが必要な時

---

### 2. タプル構造体

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
```

**用途:** 型に意味を持たせたい時

---

### 3. ユニット様構造体

```rust
struct AlwaysEqual;
```

**用途:** トレイト実装のみ必要な時

---

## まとめ

### 構造体の重要ポイント

| 概念 | 説明 |
|---|---|
| **定義** | `struct Name { field: Type }` |
| **インスタンス** | 全フィールドに値を設定 |
| **アクセス** | ドット記法 |
| **可変性** | インスタンス全体が可変か不変 |
| **省略記法** | 変数名=フィールド名なら省略可 |
| **更新記法** | `..other` で残りをコピー |

### タプル構造体

```
通常のタプル: (i32, i32, i32)
タプル構造体: Point(i32, i32, i32)

違い: 型安全性（混同を防ぐ）
```

### Debug トレイト

```
#[derive(Debug)]  // 自動実装
println!("{:?}", instance);  // デバッグ表示
dbg!(&instance);  // デバッグマクロ
```
