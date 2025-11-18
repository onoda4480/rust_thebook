# 第11章 Q&A (3/3): テストの構成

## Q1: 単体テストと結合テストの違いは？どう使い分ける？

### A: 単体テストは実装の詳細を、結合テストは公開APIをテストする

---

### 単体テスト (Unit Tests)

**場所:** src/ 内（テスト対象と同じファイル）

**特徴:**
- プライベート関数もテストできる
- 実装の詳細をテスト
- `#[cfg(test)]` で囲む

**例:**
```rust
// src/lib.rs
pub fn add_two(a: i32) -> i32 {
    internal_add(a, 2)
}

fn internal_add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public() {
        assert_eq!(add_two(2), 4);
    }

    #[test]
    fn test_private() {
        assert_eq!(internal_add(2, 2), 4);  // ✅ プライベート関数もOK
    }
}
```

---

### 結合テスト (Integration Tests)

**場所:** tests/ ディレクトリ

**特徴:**
- 公開APIのみテストできる
- ライブラリ全体の動作をテスト
- 外部から使う時の動作を確認

**例:**
```rust
// tests/integration_test.rs
use my_project;

#[test]
fn test_public_api() {
    assert_eq!(my_project::add_two(2), 4);  // ✅ 公開APIのみ

    // my_project::internal_add(2, 2);  // ❌ プライベート関数は使えない
}
```

---

### 使い分け

| テスト種類 | 用途 | 対象 |
|-----------|------|------|
| **単体テスト** | 実装の詳細 | すべての関数 |
| **結合テスト** | 外部からの使い方 | 公開APIのみ |

---

### 実践例

```rust
// src/lib.rs
pub struct Calculator {
    value: i32,
}

impl Calculator {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn add(&mut self, x: i32) {
        self.value = self.internal_add(x);
    }

    fn internal_add(&self, x: i32) -> i32 {
        self.value + x
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

// 単体テスト
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_add() {
        let calc = Calculator::new();
        assert_eq!(calc.internal_add(5), 5);  // ✅ プライベートメソッド
    }
}
```

```rust
// tests/integration_test.rs
use my_project::Calculator;

#[test]
fn test_public_api() {
    let mut calc = Calculator::new();
    calc.add(5);
    assert_eq!(calc.get_value(), 5);  // ✅ 公開APIのみ
}
```

---

## Q2: なぜ #[cfg(test)] が必要？

### A: テストコードを本番ビルドから除外するため

---

### #[cfg(test)] なしの場合

```rust
// src/lib.rs
pub fn add_two(a: i32) -> i32 {
    a + 2
}

mod tests {  // #[cfg(test)] なし
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_two(2), 4);
    }
}
```

**問題:**
```bash
cargo build
```

**結果:** `tests` モジュールも本番ビルドに含まれる

---

### #[cfg(test)] ありの場合

```rust
// src/lib.rs
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {  // #[cfg(test)] あり
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_two(2), 4);
    }
}
```

**メリット:**

**cargo build:**
```bash
cargo build
```
→ `tests` モジュールは除外される

**cargo test:**
```bash
cargo test
```
→ `tests` モジュールがコンパイルされる

---

### バイナリサイズの違い

```rust
#[cfg(test)]
mod tests {
    fn setup() {
        // 大量のヘルパー関数
        // テストデータ
    }

    #[test]
    fn test1() { ... }
    #[test]
    fn test2() { ... }
    // ... 100個のテスト
}
```

**#[cfg(test)] なし:**
- 本番ビルドに全テストコードが含まれる
- バイナリサイズが大きくなる

**#[cfg(test)] あり:**
- 本番ビルドには含まれない
- バイナリサイズが小さくなる

---

### まとめ

```
#[cfg(test)] の効果:
✅ cargo test の時のみコンパイル
✅ cargo build では除外
✅ バイナリサイズを小さく保つ
✅ テスト専用のヘルパー関数も除外される
```

---

## Q3: tests/common/mod.rs はなぜ tests/common.rs ではダメ？

### A: tests/common.rs だとテストとして実行されてしまうから

---

### 問題: tests/common.rs

```
tests/
├── integration_test.rs
└── common.rs  ❌ これだとテストとして実行される
```

```rust
// tests/common.rs
pub fn setup() {
    // ヘルパー関数
}
```

**実行:**
```bash
cargo test
```

**出力:**
```
running 0 tests
test result: ok.

     Running tests/common.rs

running 0 tests
test result: ok.

     Running tests/integration_test.rs

running 1 test
test integration_test::test ... ok
```

**問題:** `common.rs` がテストとして実行される（テストがなくても）

---

### 解決: tests/common/mod.rs

```
tests/
├── integration_test.rs
└── common/
    └── mod.rs  ✅ テストとして実行されない
```

```rust
// tests/common/mod.rs
pub fn setup() {
    // ヘルパー関数
}
```

**実行:**
```bash
cargo test
```

**出力:**
```
     Running tests/integration_test.rs

running 1 test
test integration_test::test ... ok
```

**メリット:** `common/mod.rs` は実行されない

---

### 使い方

```rust
// tests/integration_test.rs
mod common;  // tests/common/mod.rs をインポート

#[test]
fn test_with_setup() {
    common::setup();  // ヘルパー関数を使う
    // テスト...
}
```

---

### なぜこうなる？

**Rust のルール:**
- `tests/` ディレクトリの `.rs` ファイルは結合テストとして扱われる
- サブディレクトリのファイルはモジュールとして扱われる

```
tests/
├── test1.rs       ← 結合テスト
├── test2.rs       ← 結合テスト
└── helpers/
    └── mod.rs     ← モジュール（テストではない）
```

---

### まとめ

```
ヘルパー関数の配置:

❌ tests/common.rs
   → テストとして実行される

✅ tests/common/mod.rs
   → モジュールとして扱われる
   → テストとして実行されない
```

---

## Q4: バイナリクレート（main.rs のみ）は結合テストできない？どうすればいい？

### A: lib.rs + main.rs パターンを使う

---

### 問題: main.rs のみ

```
project/
├── src/
│   └── main.rs  ❌ 結合テストできない
└── tests/
    └── integration_test.rs
```

```rust
// src/main.rs
fn main() {
    let result = add_two(2);
    println!("{}", result);
}

fn add_two(a: i32) -> i32 {
    a + 2
}
```

```rust
// tests/integration_test.rs
use my_project;  // ❌ エラー！

#[test]
fn test_add_two() {
    assert_eq!(my_project::add_two(2), 4);  // ❌ add_two は公開されていない
}
```

**エラー:**
```
error: can't find crate `my_project`
```

---

### 解決: lib.rs + main.rs パターン

```
project/
├── src/
│   ├── lib.rs   ✅ ロジック
│   └── main.rs  ✅ エントリーポイント
└── tests/
    └── integration_test.rs  ✅ lib.rs をテスト
```

---

#### src/lib.rs（ロジック）

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn run() {
    let result = add_two(2);
    println!("{}", result);
}
```

---

#### src/main.rs（薄いラッパー）

```rust
use my_project;

fn main() {
    my_project::run();
}
```

**ポイント:**
- main.rs は薄い（ロジックなし）
- ロジックは lib.rs にある

---

#### tests/integration_test.rs（結合テスト）

```rust
use my_project;

#[test]
fn test_add_two() {
    assert_eq!(my_project::add_two(2), 4);  // ✅ OK
}
```

---

### なぜこれが必要？

**Rust のルール:**
- ライブラリクレート（lib.rs）のみが外部から使える
- バイナリクレート（main.rs）は実行専用

---

### まとめ

```
バイナリクレートのテスト:

❌ main.rs のみ
   → 結合テストできない

✅ lib.rs + main.rs
   → lib.rs: ロジック（テスト可能）
   → main.rs: エントリーポイント（薄い）
   → tests/: lib.rs をテスト
```

---

## Q5: 単体テストと結合テストのどちらを書くべき？両方必要？

### A: 両方書くのがベストプラクティス

---

### 単体テストの役割

```rust
// src/lib.rs
pub struct Calculator {
    value: i32,
}

impl Calculator {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    fn validate(&self, x: i32) -> bool {
        x >= 0 && x <= 100
    }

    pub fn add(&mut self, x: i32) -> Result<(), String> {
        if self.validate(x) {
            self.value += x;
            Ok(())
        } else {
            Err(String::from("invalid value"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 実装の詳細をテスト
    #[test]
    fn test_validate() {
        let calc = Calculator::new();
        assert!(calc.validate(50));   // ✅ プライベートメソッド
        assert!(!calc.validate(101));
    }

    #[test]
    fn test_add_valid() {
        let mut calc = Calculator::new();
        assert!(calc.add(10).is_ok());
    }
}
```

**用途:**
- プライベート関数のテスト
- エッジケースのテスト
- 実装の詳細を細かくテスト

---

### 結合テストの役割

```rust
// tests/integration_test.rs
use my_project::Calculator;

#[test]
fn test_calculator_workflow() {
    // ユーザーの使い方をテスト
    let mut calc = Calculator::new();

    calc.add(10).unwrap();
    calc.add(20).unwrap();

    assert_eq!(calc.get_value(), 30);
}

#[test]
fn test_error_handling() {
    let mut calc = Calculator::new();

    // 公開APIのエラー処理をテスト
    assert!(calc.add(101).is_err());
}
```

**用途:**
- 公開APIの使い方をテスト
- ワークフローをテスト
- 外部から見た動作をテスト

---

### 両方書くべき理由

**単体テストのみ:**
- ❌ 公開APIの統合動作が未検証
- ❌ ユーザーの使い方が未検証

**結合テストのみ:**
- ❌ プライベート関数が未検証
- ❌ エッジケースが見逃される可能性

**両方:**
- ✅ 完全なテストカバレッジ
- ✅ 実装の詳細とAPIの両方を検証

---

### テストピラミッド

```
      /\
     /結合\      ← 少ない（ワークフロー）
    /------\
   /  統合  \    ← 中程度（モジュール間）
  /----------\
 /  単体テスト \  ← 多い（個々の関数）
/--------------\
```

**推奨:**
- 単体テスト: 多く書く（細かいケース）
- 結合テスト: 適度に書く（主要なワークフロー）

---

### まとめ

```
どちらを書くべき？
→ 両方

単体テスト:
✅ 実装の詳細
✅ プライベート関数
✅ エッジケース
✅ 多く書く

結合テスト:
✅ 公開API
✅ ワークフロー
✅ ユーザーの使い方
✅ 主要なケースを書く
```
