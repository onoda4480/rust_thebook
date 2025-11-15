# Chapter 9 Q&A Part 3: カスタム型と Guess

## Q1: Guess 型は何のため？

**質問:** `Guess` 型を作る目的は？普通に `u32` じゃダメ？

**回答:** **型で値の正当性を保証するためです。**

---

### 問題：u32 を使う場合

```rust
fn process_guess(guess: u32) {
    // 毎回チェックが必要
    if guess < 1 || guess > 100 {
        panic!("Invalid!");
    }
    // 処理...
}

fn save_guess(guess: u32) {
    // またチェックが必要
    if guess < 1 || guess > 100 {
        panic!("Invalid!");
    }
    // 処理...
}

fn display_guess(guess: u32) {
    // またチェックが必要
    if guess < 1 || guess > 100 {
        panic!("Invalid!");
    }
    // 処理...
}
```

**問題:**
- チェックが散在
- チェックを忘れる可能性
- 面倒

---

### 解決策：Guess 型

```rust
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        // ここで一度だけチェック
        if value < 1 || value > 100 {
            panic!("Invalid!");
        }
        Guess { value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

fn process_guess(guess: Guess) {
    // チェック不要！
    // Guess 型 = 必ず 1〜100
}

fn save_guess(guess: Guess) {
    // チェック不要！
}

fn display_guess(guess: Guess) {
    // チェック不要！
}
```

**利点:**
- チェックが一箇所
- チェック漏れがない
- 型が保証する

---

## Q2: フィールドを非公開にする理由は？

**質問:** なぜ `value` フィールドを非公開にするの？

**回答:** **外部から直接変更されないようにするためです。**

---

### 公開フィールドの問題

```rust
pub struct Guess {
    pub value: u32,  // 公開
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Invalid!");
        }
        Guess { value }
    }
}

// 使用
let mut guess = Guess::new(50);  // OK: 50 は 1〜100

// ❌ 問題：外から直接変更できる
guess.value = 200;  // 検証をバイパス！
println!("{}", guess.value);  // 200（範囲外）
```

**問題:** 検証が無意味になる

---

### 非公開フィールドの利点

```rust
pub struct Guess {
    value: u32,  // 非公開
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Invalid!");
        }
        Guess { value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

// 使用
let guess = Guess::new(50);  // OK

// ✅ 読み取りはできる
let val = guess.value();

// ❌ 変更はできない
// guess.value = 200;  // コンパイルエラー
```

**保証:** Guess 型 = 必ず 1〜100

---

## Q3: ゲッターとセッター、なぜセッターがない？

**質問:** ゲッター `value()` はあるのに、セッターがないのはなぜ？

**回答:** **値を変更させたくないからです。**

---

### セッターがない理由

```rust
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Invalid!");
        }
        Guess { value }
    }

    // ゲッター（読み取り）
    pub fn value(&self) -> u32 {
        self.value
    }

    // セッターは実装しない！
}
```

**理由:**
- 一度作ったら変更不可
- 不変条件を保持
- 値を変えたいなら新しい Guess を作る

---

### もしセッターを実装するなら

```rust
impl Guess {
    pub fn set_value(&mut self, value: u32) {
        // セッターでも検証が必要
        if value < 1 || value > 100 {
            panic!("Invalid!");
        }
        self.value = value;
    }
}
```

**でも実装しない:**
- 不変性を保つ
- シンプルに保つ

---

## Q4: new() と value() の違いは？

**質問:** `new()` と `value()` は何が違うの？

**回答:** **役割が違います。**

---

### new() - コンストラクタ

```rust
pub fn new(value: u32) -> Guess {
    //                    ^^^^^
    //                    Guess 型を返す

    // 検証
    if value < 1 || value > 100 {
        panic!("Invalid!");
    }

    // Guess インスタンスを作成
    Guess { value }
}

// 使用
let guess = Guess::new(50);
//  ^^^^^   ^^^^^^^^^^^^^^
//  Guess型  Guess を作成
```

**役割:** Guess インスタンスを作成

---

### value() - ゲッター

```rust
pub fn value(&self) -> u32 {
    //                 ^^^
    //                 u32 を返す

    self.value  // フィールドの値を返す
}

// 使用
let val = guess.value();
//  ^^^   ^^^^^^^^^^^^^
//  u32型  値を取得
```

**役割:** フィールドの値を取得

---

### 対応表

| メソッド | 役割 | 戻り値 | 検証 |
|---|---|---|---|
| `new()` | インスタンス作成 | `Guess` | ✅ あり |
| `value()` | 値の取得 | `u32` | ❌ なし |

---

## Q5: 型で保証する利点は？

**質問:** 型で不変条件を保証する利点は？

**回答:** **コンパイル時にチェックできるからです。**

---

### 通常の方法（実行時チェック）

```rust
fn process_guess(guess: u32) {
    // 実行時にチェック
    if guess < 1 || guess > 100 {
        panic!("Invalid!");
    }
    // 処理...
}

fn main() {
    process_guess(200);  // 実行してエラー
}
```

**問題:**
- 実行するまでエラーが分からない
- チェックを忘れる可能性

---

### 型で保証（コンパイル時チェック）

```rust
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Invalid!");
        }
        Guess { value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

fn process_guess(guess: Guess) {
    //            ^^^^^^^^^^^^
    //            Guess 型を要求
    //            = 必ず 1〜100

    // チェック不要
    let val = guess.value();
}

fn main() {
    let guess = Guess::new(50);  // ✅ OK
    process_guess(guess);

    // let invalid = Guess::new(200);  // panic!
    // process_guess(invalid);

    // process_guess(200);  // ❌ コンパイルエラー
    //               ^^^
    //               u32 は Guess 型ではない
}
```

**利点:**
- 型システムが保証
- チェック漏れがない
- コンパイル時に検出

---

## Python との比較

### Python

```python
class Guess:
    def __init__(self, value):
        if value < 1 or value > 100:
            raise ValueError("Invalid")
        self._value = value

    @property
    def value(self):
        return self._value

# 使用
guess = Guess(50)  # OK
print(guess.value)

# 問題：変更できてしまう
guess._value = 200  # 慣習的に非公開だが、技術的には可能
print(guess._value)  # 200
```

**問題:**
- `_value` は慣習的に非公開
- でも技術的には変更できる

---

### Rust

```rust
pub struct Guess {
    value: u32,  // 非公開（強制）
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Invalid");
        }
        Guess { value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

// 使用
let guess = Guess::new(50);  // OK
println!("{}", guess.value());

// ❌ 変更不可（コンパイルエラー）
// guess.value = 200;
```

**利点:**
- 非公開が強制される
- コンパイラが保証

---

## まとめ

### Guess 型の目的

```
型で不変条件を保証

Guess 型のインスタンスが存在する
= value は必ず 1〜100

これにより:
✅ チェックが一箇所
✅ チェック漏れがない
✅ 型システムが保証
```

---

### カプセル化

```rust
pub struct Guess {
    value: u32,  // 非公開（重要）
}

impl Guess {
    pub fn new(value: u32) -> Guess { }  // コンストラクタ
    pub fn value(&self) -> u32 { }       // ゲッター
    // セッターなし（変更不可）
}
```

---

### 役割分担

```
new():
  役割: インスタンス作成
  検証: ✅ あり
  戻り値: Guess

value():
  役割: 値の取得
  検証: ❌ なし（不要）
  戻り値: u32
```

---

### 利点

```
✅ 型で保証
✅ コンパイル時チェック
✅ チェック漏れ防止
✅ コードが簡潔
✅ 意図が明確
```
