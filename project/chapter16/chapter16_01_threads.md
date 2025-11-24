# Chapter 16-1: スレッド - 複数の処理を同時実行

## スレッドとは？

**プログラムの処理を複数同時に実行する仕組み**

```rust
use std::thread;

thread::spawn(|| {
    println!("別のスレッドで実行");
});
```

---

## スレッドの作成

### thread::spawn

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("スレッド: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("メイン: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

**問題:** メインスレッドが終了すると、スレッドも途中で止まる

---

## join で待つ

```rust
let handle = thread::spawn(|| {
    for i in 1..10 {
        println!("スレッド: {}", i);
    }
});

// スレッドが終わるまで待つ
handle.join().unwrap();
```

---

## クロージャの復習

### 基本構文
```rust
|引数| 処理
```

### 環境のキャプチャ
```rust
let y = 10;
let add = |x| x + y;  // y を外からキャプチャ

println!("{}", add(5));  // 15
```

---

## move クロージャ

### なぜ必要？

```rust
let v = vec![1, 2, 3];

thread::spawn(|| {
    println!("{:?}", v);  // ❌ エラー！
});
// v の借用だけだと、メインスレッドがvをdropする可能性がある
```

### 解決策：move

```rust
let v = vec![1, 2, 3];

thread::spawn(move || {  // move で所有権を奪う
    println!("{:?}", v);  // ✅ OK
});

// v はもう使えない
```

---

## move の動き

### メモリ図

#### 借用の場合（危険）
```
メインスレッド              スレッド
┌─────────┐              ┌─────────┐
│ v       │              │ &v      │ ← 参照だけ
│ [1,2,3] │              │         │
└─────────┘              └─────────┘
     ↓
   drop(v)  ← v が消える

                         ┌─────────┐
                         │ &v ???  │ ← 参照先が消えた！💥
                         └─────────┘
```

#### move の場合（安全）
```
メインスレッド              スレッド
┌─────────┐              ┌─────────┐
│ v       │─────移動────>│ v       │ ← 所有権を奪う
│ [1,2,3] │              │ [1,2,3] │
└─────────┘              └─────────┘
     ↓
   もう使えない

メインスレッドはvを使えない
→ drop(v) できない
→ スレッドは安全！✅
```

---

## Rust の所有権システムが並行性を保証

### 問題：借用だけだと危険
```rust
let v = vec![1, 2, 3];

thread::spawn(|| {
    println!("{:?}", v);  // v を借用
});

drop(v);  // メインスレッドで削除！
// スレッドが実行される時、v は存在しない💥
```

### コンパイラが防ぐ
```
error: closure may outlive the current function
```

### move で解決
```rust
thread::spawn(move || {
    println!("{:?}", v);  // 所有権を奪う
});
// drop(v);  // ❌ エラー！v はもうない
```

---

## スレッドの返り値

```rust
let handle = thread::spawn(|| {
    42  // 返り値
});

let result = handle.join().unwrap();
println!("結果: {}", result);  // 42
```

---

## Python との対応

| Rust | Python |
|------|--------|
| `thread::spawn(\\|\\| {})` | `Thread(target=lambda: ...)` |
| `handle.join()` | `thread.join()` |
| `move` | なし（すべて参照） |

**重要:** Pythonには GIL があるため、真の並列実行は難しい

---

## まとめ

| 項目 | 説明 |
|------|------|
| **thread::spawn** | スレッドを作成 |
| **join()** | スレッドの終了を待つ |
| **クロージャ** | `\\|引数\\| 処理` |
| **move** | 所有権を奪う |
| **所有権システム** | 並行性の安全性を保証 |

**重要:** Rustは所有権システムでデータ競合を防ぐ！
