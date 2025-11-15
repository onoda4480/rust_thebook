# Chapter 9-1: panic! と回復不可能なエラー

## panic! とは？

**プログラムを即座に停止させるマクロ**

```rust
panic!("致命的なエラーが発生しました！");
```

---

## panic! の特徴

| 特徴 | 説明 |
|------|------|
| **回復不可能** | プログラムが停止する |
| **エラーメッセージ** | 指定したメッセージを表示 |
| **バックトレース** | エラーの発生箇所を追跡 |
| **スタック巻き戻し** | メモリをクリーンアップ |

---

## 基本的な使い方

### 明示的な panic!

```rust
fn main() {
    panic!("クラッシュして炎上");
}
```

**出力:**
```
thread 'main' panicked at 'クラッシュして炎上', src/main.rs:2:5
```

---

### 条件付き panic!

```rust
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("0で割ることはできません！");
    }
    a / b
}
```

---

## 暗黙的な panic!

### 配列の範囲外アクセス

```rust
fn main() {
    let v = vec![1, 2, 3];

    let element = v[99];  // panic!
}
```

**出力:**
```
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99'
```

---

### unwrap() での panic!

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
    // ファイルが存在しない → panic!
}
```

**出力:**
```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ...'
```

---

## バックトレース

### バックトレースとは？

**エラーが発生するまでに呼び出された全ての関数のリスト**

---

### バックトレースの有効化

```bash
RUST_BACKTRACE=1 cargo run
```

**出力:**
```
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:19
stack backtrace:
   0: rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::panic_bounds_check
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
   6: panic_example::main
             at ./src/main.rs:4:19
   7: core::ops::function::FnOnce::call_once
             at /rustc/.../library/core/src/ops/function.rs:227:5
...
```

---

### バックトレースの読み方

```
1. 自分のコードを探す
   → panic_example::main at ./src/main.rs:4:19

2. 該当行を確認
   → 4行目、19文字目

3. 原因を特定
   → v[99] で範囲外アクセス

4. 修正
   → v.get(99) を使うなど
```

---

## スタック巻き戻し vs アボート

### スタック巻き戻し（デフォルト）

```toml
# Cargo.toml
[profile.release]
# デフォルト設定
```

**動作:**
1. panic! が発生
2. スタックを巻き戻す
3. メモリをクリーンアップ
4. プログラム終了

**特徴:**
- メモリが適切に解放される
- バイナリサイズが大きい

---

### アボート

```toml
# Cargo.toml
[profile.release]
panic = 'abort'
```

**動作:**
1. panic! が発生
2. 即座にプログラム終了
3. OS がメモリをクリーンアップ

**特徴:**
- より高速
- バイナリサイズが小さい
- メモリクリーンアップは OS 任せ

---

## unwrap と expect

### unwrap()

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
    // エラー時: デフォルトのメッセージで panic!
}
```

**エラー時の出力:**
```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ...'
```

---

### expect()

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")
        .expect("hello.txt を開けませんでした");
    // エラー時: カスタムメッセージで panic!
}
```

**エラー時の出力:**
```
thread 'main' panicked at 'hello.txt を開けませんでした: ...'
```

---

### unwrap vs expect

| メソッド | メッセージ | 推奨度 |
|---|---|---|
| `unwrap()` | デフォルト | ❌ 避ける |
| `expect()` | カスタム | ✅ 推奨 |

**expect() を使う理由:**
- エラーの原因が明確
- デバッグしやすい

---

## panic! を使うべき場面

### 1. プロトタイプやテスト

```rust
#[test]
fn test_something() {
    let result = some_function();
    assert_eq!(result, 42);  // 失敗したら panic!
}
```

---

### 2. 論理的に起こり得ないケース

```rust
fn get_first_element(v: &Vec<i32>) -> i32 {
    if v.is_empty() {
        panic!("空のベクタは渡さないという前提");
    }
    v[0]
}
```

---

### 3. 回復不可能なエラー

```rust
fn initialize_system() {
    if !critical_resource_available() {
        panic!("必須リソースが利用できません");
    }
}
```

---

## panic! を避けるべき場面

### 1. ファイル操作

```rust
// ❌ panic! を使う
let f = File::open("hello.txt").unwrap();

// ✅ Result で処理
match File::open("hello.txt") {
    Ok(file) => file,
    Err(e) => {
        println!("Error: {:?}", e);
        return;
    }
}
```

---

### 2. ユーザー入力

```rust
// ❌ panic! を使う
let num: i32 = input.parse().unwrap();

// ✅ Result で処理
match input.parse::<i32>() {
    Ok(num) => num,
    Err(e) => {
        println!("無効な入力です");
        return;
    }
}
```

---

### 3. ネットワーク操作

```rust
// ❌ panic! を使う
let response = http_get(url).unwrap();

// ✅ Result で処理
match http_get(url) {
    Ok(response) => response,
    Err(e) => {
        println!("接続エラー: {:?}", e);
        return;
    }
}
```

---

## Python との比較

### Python

```python
# 例外を投げる
raise Exception("エラーが発生しました")

# キャッチされない例外 → プログラム停止
def divide(a, b):
    if b == 0:
        raise ZeroDivisionError("0で割れません")
    return a / b

# try-catch なし → プログラム停止
result = divide(10, 0)
```

---

### Rust

```rust
// panic! を呼ぶ
panic!("エラーが発生しました");

// panic! → プログラム停止
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("0で割れません");
    }
    a / b
}

// panic! を止められない → プログラム停止
let result = divide(10, 0);
```

---

## まとめ

### panic! の特徴

```
✅ プログラムを即座に停止
✅ エラーメッセージを表示
✅ バックトレースで原因を追跡
✅ 回復不可能なエラー用
```

---

### 使い分け

```
panic! を使う:
✅ プロトタイプ・テスト
✅ 論理的に起こり得ないケース
✅ 回復不可能なエラー

panic! を避ける:
❌ ファイル操作
❌ ユーザー入力
❌ ネットワーク操作
→ Result を使う
```

---

### unwrap と expect

```
✅ expect("理由") を推奨
❌ unwrap() は避ける

理由:
- エラーメッセージが明確
- デバッグしやすい
```

---

### バックトレース

```bash
# バックトレースを表示
RUST_BACKTRACE=1 cargo run

# 読み方:
1. 自分のコードを探す
2. 該当行を確認
3. 原因を特定
4. 修正
```
