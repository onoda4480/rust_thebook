# 第11章 Q&A (1/3): テストの書き方

## Q1: assert!, assert_eq!, assert_ne! の使い分けは？

### A: 用途によって使い分ける

---

### assert! - 条件を確認

```rust
assert!(condition);
```

**用途:** 真偽値を確認

**例:**
```rust
#[test]
fn test_greater() {
    let x = 5;
    assert!(x > 3);  // x が 3 より大きいか
}

#[test]
fn test_is_empty() {
    let s = String::new();
    assert!(s.is_empty());  // 空かどうか
}
```

---

### assert_eq! - 等しいことを確認

```rust
assert_eq!(left, right);
```

**用途:** 2つの値が等しいか確認

**例:**
```rust
#[test]
fn test_add() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_function_result() {
    let result = add_two(2);
    assert_eq!(result, 4);
}
```

---

### assert_ne! - 異なることを確認

```rust
assert_ne!(left, right);
```

**用途:** 2つの値が異なるか確認

**例:**
```rust
#[test]
fn test_not_equal() {
    assert_ne!(2 + 2, 5);
}

#[test]
fn test_different_strings() {
    let s1 = "hello";
    let s2 = "world";
    assert_ne!(s1, s2);
}
```

---

### 使い分けの例

```rust
#[test]
fn comprehensive_test() {
    let x = 10;
    let y = 20;

    // 条件の確認
    assert!(x < y);           // x が y より小さい
    assert!(x > 0);           // x が正の数

    // 等しいことを確認
    assert_eq!(x + 10, y);    // x + 10 が y と等しい

    // 異なることを確認
    assert_ne!(x, y);         // x と y が異なる
}
```

---

## Q2: #[should_panic] と Result<T, E> の違いは？どちらを使うべき？

### A: エラーの種類と詳細な検証の必要性によって使い分ける

---

### #[should_panic] - パニックを期待

```rust
#[test]
#[should_panic]
fn test_panic() {
    panic!("This should panic");
}
```

**用途:**
- パニックすることを確認
- シンプルなエラー処理のテスト

**制限:**
- エラーの詳細を検証しにくい
- パニックしたかどうかだけを確認

---

### #[should_panic(expected = "...")] - メッセージも確認

```rust
#[test]
#[should_panic(expected = "Guess value must be between 1 and 100")]
fn greater_than_100() {
    Guess::new(200);
}
```

**用途:**
- 特定のメッセージでパニックすることを確認
- エラーメッセージの部分一致

---

### Result<T, E> - 柔軟なエラー処理

```rust
#[test]
fn test_with_result() -> Result<(), String> {
    let result = risky_operation()?;

    if result == expected_value {
        Ok(())
    } else {
        Err(String::from("unexpected value"))
    }
}
```

**用途:**
- エラーの詳細を検証
- 複数のエラーケースを処理
- ? 演算子を使いたい

---

### 比較表

| 方法 | メリット | デメリット |
|------|---------|----------|
| **#[should_panic]** | シンプル | エラーの詳細を検証しにくい |
| **#[should_panic(expected)]** | メッセージも確認 | 部分一致のみ |
| **Result<T, E>** | 柔軟、詳細な検証 | 少し複雑 |

---

### 使い分けの例

```rust
// ケース1: パニックするだけ確認（シンプル）
#[test]
#[should_panic]
fn test_panic_simple() {
    panic!("error");
}

// ケース2: パニックメッセージも確認
#[test]
#[should_panic(expected = "value must be")]
fn test_panic_with_message() {
    Guess::new(200);
}

// ケース3: 詳細なエラー検証
#[test]
fn test_with_result() -> Result<(), String> {
    let result = parse_number("abc");

    match result {
        Err(e) if e.contains("invalid") => Ok(()),
        _ => Err(String::from("expected parse error")),
    }
}
```

---

## Q3: カスタムエラーメッセージはいつ使うべき？

### A: テストが失敗した時に、なぜ失敗したかわかりにくい場合

---

### カスタムメッセージなし

```rust
#[test]
fn test_greeting() {
    let result = greeting("Carol");
    assert!(result.contains("Carol"));
}
```

**失敗時:**
```
assertion failed: result.contains("Carol")
```

**問題:** `result` の値がわからない

---

### カスタムメッセージあり

```rust
#[test]
fn test_greeting() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}
```

**失敗時:**
```
Greeting did not contain name, value was `Hello!`
```

**メリット:** 実際の値がわかる

---

### いつ使うべき？

**使うべき:**
- 失敗の原因がわかりにくい時
- 実際の値を確認したい時
- 複雑な条件のテスト

**例:**
```rust
#[test]
fn test_complex_condition() {
    let user = create_test_user();

    assert!(
        user.age >= 18 && user.age <= 100,
        "User age should be between 18 and 100, but was {}",
        user.age
    );
}
```

---

**使わなくていい:**
- assert_eq! で十分な場合

```rust
#[test]
fn test_simple() {
    assert_eq!(2 + 2, 4);  // カスタムメッセージ不要
}
```

---

## Q4: テストで println! を使うべき？

### A: デバッグ目的なら使える。ただし通常は assert を使うべき。

---

### println! の使い方

```rust
#[test]
fn test_with_debug() {
    let x = expensive_calculation();
    println!("Calculated value: {}", x);

    assert_eq!(x, 42);
}
```

**実行:**
```bash
cargo test -- --show-output
```

**用途:**
- デバッグ
- 中間値の確認
- テストの進行状況の確認

---

### 注意点

```rust
#[test]
fn test_bad_practice() {
    let x = add_two(2);
    println!("x = {}", x);  // ❌ これだけでは不十分

    // assert がないとテストが常に成功する
}
```

**問題:** 出力があっても、assert がないと値の検証ができない

---

### 正しい使い方

```rust
#[test]
fn test_good_practice() {
    let x = add_two(2);
    println!("x = {}", x);  // デバッグ用

    assert_eq!(x, 4);  // ✅ assert で検証
}
```

---

### ベストプラクティス

```
✅ デバッグ目的で println! を使う
✅ でも必ず assert も使う
✅ 成功時の出力は表示されない（--show-output で表示）
❌ println! だけに頼らない
```

---

## Q5: #[ignore] はどういう時に使う？

### A: 時間がかかるテストや、特定の条件でのみ実行したいテスト

---

### 基本的な使い方

```rust
#[test]
#[ignore]
fn expensive_test() {
    // 時間のかかるテスト
    thread::sleep(Duration::from_secs(10));
    assert_eq!(2 + 2, 4);
}
```

**通常の実行:**
```bash
cargo test
```

**結果:** スキップされる

---

**無視されたテストを実行:**
```bash
cargo test -- --ignored
```

---

### 使用例1: 時間のかかるテスト

```rust
#[test]
#[ignore]
fn test_large_dataset() {
    let data = generate_million_records();
    // 処理に数分かかる...
    assert!(data.len() == 1_000_000);
}
```

**用途:**
- 開発中はスキップ
- CI/CD で定期的に実行

---

### 使用例2: 外部リソースが必要なテスト

```rust
#[test]
#[ignore]
fn test_database_connection() {
    // データベースが必要
    let conn = connect_to_database();
    assert!(conn.is_ok());
}
```

**用途:**
- ローカルでは実行しない
- CI/CD 環境でのみ実行

---

### 使用例3: 不安定なテスト

```rust
#[test]
#[ignore]
fn test_flaky() {
    // たまに失敗するテスト
    // 修正が必要だが、一時的に無視
    assert_eq!(random_value(), 42);
}
```

**用途:**
- 一時的に無視
- 後で修正予定

---

### まとめ

```
#[ignore] を使うべき場合:
✅ 時間のかかるテスト
✅ 外部リソースが必要なテスト
✅ 特定の環境でのみ実行したいテスト
✅ 一時的に無視したいテスト（修正予定）

実行方法:
cargo test              # 無視される
cargo test -- --ignored # 無視されたテストのみ
```
