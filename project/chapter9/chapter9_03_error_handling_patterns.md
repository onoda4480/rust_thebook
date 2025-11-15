# Chapter 9-3: エラー処理のパターンとカスタム型

## panic! vs Result の使い分け

### panic! を使うべき場合

```rust
// ① テストコード
#[test]
fn test_something() {
    assert_eq!(some_function(), expected);  // 失敗したら panic!
}

// ② プロトタイプ
fn main() {
    let config = load_config().unwrap();  // 開発中は unwrap でOK
}

// ③ 論理的に起こり得ないケース
fn process_data(data: &[i32]) {
    if data.is_empty() {
        panic!("データは空でないという前提");
    }
    let first = data[0];  // 安全
}
```

---

### Result を使うべき場合

```rust
// ① ファイル操作
fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

// ② ユーザー入力
fn parse_age(input: &str) -> Result<u32, ParseIntError> {
    input.parse()
}

// ③ ネットワーク操作
fn fetch_data(url: &str) -> Result<String, reqwest::Error> {
    // HTTP リクエスト
}
```

---

## カスタム型でエラーを防ぐ

### 問題：範囲チェックが必要

```rust
fn process_guess(guess: u32) {
    // 毎回チェックが必要
    if guess < 1 || guess > 100 {
        panic!("Invalid!");
    }
    // 処理...
}

fn another_function(guess: u32) {
    // またチェックが必要
    if guess < 1 || guess > 100 {
        panic!("Invalid!");
    }
    // 処理...
}
```

**問題:**
- チェックを忘れるかもしれない
- 面倒

---

### 解決策：Guess 型

```rust
pub struct Guess {
    value: u32,  // 非公開
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        // 一度だけチェック
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
```

---

### 使用例

```rust
fn process_guess(guess: Guess) {
    // チェック不要！
    // Guess 型 = 必ず 1〜100
    println!("Processing: {}", guess.value());
}

fn another_function(guess: Guess) {
    // ここでもチェック不要！
    println!("Value: {}", guess.value());
}

fn main() {
    let guess = Guess::new(50);  // ここで一度だけ検証
    process_guess(guess);
}
```

---

## 型で不変条件を保証

### 不変条件とは？

**常に真であるべき条件**

```rust
pub struct Guess {
    value: u32,
}
```

**Guess 型の不変条件:**
```
Guess のインスタンスが存在する
→ value は必ず 1〜100
```

---

### カプセル化による保証

```rust
pub struct Guess {
    value: u32,  // ← 非公開（重要）
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        // 検証
        if value < 1 || value > 100 {
            panic!("Invalid!");
        }
        Guess { value }  // OK なら作成
    }

    // ゲッター（読み取りのみ）
    pub fn value(&self) -> u32 {
        self.value
    }

    // セッターは実装しない（変更不可）
}
```

**保証の仕組み:**
1. フィールドが非公開
2. `new()` でしか作れない
3. `new()` は検証する
4. → Guess 型 = 常に正しい値

---

## エラー処理のパターン

### パターン1: 早期リターン

```rust
fn process_data(data: &str) -> Result<i32, String> {
    if data.is_empty() {
        return Err(String::from("データが空です"));
    }

    let num = match data.parse::<i32>() {
        Ok(n) => n,
        Err(_) => return Err(String::from("パースエラー")),
    };

    if num < 0 {
        return Err(String::from("負の数は不可"));
    }

    Ok(num)
}
```

---

### パターン2: ? 演算子

```rust
fn process_data(data: &str) -> Result<i32, Box<dyn Error>> {
    if data.is_empty() {
        return Err("データが空です".into());
    }

    let num = data.parse::<i32>()?;  // エラーなら早期リターン

    if num < 0 {
        return Err("負の数は不可".into());
    }

    Ok(num)
}
```

---

### パターン3: 複数の ? 演算子

```rust
fn read_and_process() -> Result<String, io::Error> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // 処理...

    let mut output = File::create("output.txt")?;
    output.write_all(contents.as_bytes())?;

    Ok(String::from("Success"))
}
```

---

### パターン4: メソッドチェーン

```rust
fn process_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)?
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join("\n")
        .pipe(Ok)
}
```

---

## エラー型の選択

### 具体的なエラー型

```rust
use std::io;

fn read_file() -> Result<String, io::Error> {
    //                        ^^^^^^^^
    //                        具体的な型
    fs::read_to_string("file.txt")
}
```

**利点:**
- エラーの種類が明確
- エラーハンドリングが正確

---

### Box<dyn Error>

```rust
use std::error::Error;

fn do_something() -> Result<String, Box<dyn Error>> {
    //                          ^^^^^^^^^^^^^^^^^
    //                          どんなエラーでもOK
    let file = File::open("file.txt")?;
    let num: i32 = "42".parse()?;
    Ok(String::from("Success"))
}
```

**利点:**
- 複数の異なるエラー型を扱える
- 柔軟

**欠点:**
- エラーの種類が不明確
- 型情報が失われる

---

## ガイドライン

### いつ panic! を使うか？

```
✅ 例・プロトタイプ・テスト
✅ 契約違反（論理的に起こり得ない）
✅ 回復不可能なエラー

❌ ライブラリコード
❌ 予期されるエラー
❌ ユーザー入力
```

---

### いつ Result を使うか？

```
✅ 失敗が予期される操作
✅ ファイル・ネットワークIO
✅ パース・変換
✅ ライブラリの公開API

❌ テストコード（unwrap でOK）
❌ 論理的エラー（panic! でOK）
```

---

### コードの例

```rust
// ✅ 良い例：Result を返す
pub fn read_config(path: &str) -> Result<Config, ConfigError> {
    let contents = fs::read_to_string(path)?;
    parse_config(&contents)
}

// ❌ 悪い例：unwrap を使う
pub fn read_config(path: &str) -> Config {
    let contents = fs::read_to_string(path).unwrap();  // NG!
    parse_config(&contents).unwrap()  // NG!
}
```

---

## Python との比較

### Python

```python
class AgeValidator:
    def __init__(self, age):
        # バリデーション
        if age < 0 or age > 150:
            raise ValueError("Invalid age")
        self._age = age  # 非公開（慣習）

    @property
    def age(self):
        return self._age

# 使用
try:
    validator = AgeValidator(25)
    print(validator.age)
except ValueError as e:
    print(f"Error: {e}")
```

---

### Rust

```rust
pub struct Age {
    value: u32,  // 非公開
}

impl Age {
    pub fn new(value: u32) -> Result<Age, String> {
        // バリデーション
        if value > 150 {
            return Err(String::from("Invalid age"));
        }
        Ok(Age { value })
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

// 使用
match Age::new(25) {
    Ok(age) => println!("{}", age.value()),
    Err(e) => println!("Error: {}", e),
}
```

---

## まとめ

### panic! vs Result

```
panic!:
✅ テスト・プロトタイプ
✅ 論理的エラー
✅ 回復不可能

Result:
✅ 予期されるエラー
✅ IO操作
✅ ライブラリAPI
```

---

### カスタム型

```
型で不変条件を保証

struct Guess {
    value: u32,  // 非公開
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        // 検証
        if value < 1 || value > 100 {
            panic!("Invalid!");
        }
        Guess { value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

Guess 型 = 必ず 1〜100
```

---

### ? 演算子

```
エラーを簡潔に伝播

let value = result?;

展開:
let value = match result {
    Ok(v) => v,
    Err(e) => return Err(e),
};

条件:
✅ 戻り値が Result または Option
```

---

### ベストプラクティス

```
✅ expect() を使う (unwrap() より)
✅ エラーを適切に伝播
✅ 型で不変条件を保証
✅ 公開APIは Result を返す
❌ 本番コードで unwrap() を使わない
❌ panic! を多用しない
```
