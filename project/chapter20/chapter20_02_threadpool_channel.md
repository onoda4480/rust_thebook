# 第20章 Part 2: スレッドプールとチャネル通信

## ThreadPool の実装

### 完成形のコード（lib.rs）

```rust
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}
```

---

## エラーハンドリングの2つのアプローチ

### 1. assert! でパニック（採用されたアプローチ）

```rust
pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);  // size が 0 ならパニック
    // ...
}
```

**使い方**:
```rust
let pool = ThreadPool::new(4);  // ✅ OK
let pool = ThreadPool::new(0);  // ❌ パニック！プログラム停止
```

**doc comment で明記**:
```rust
/// # パニック
///
/// sizeが0なら、`new`関数はパニックします。
```

---

### 2. Result を返す（代替案）

```rust
pub enum PoolCreationError {
    ZeroSize,
}

pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
    if size == 0 {
        return Err(PoolCreationError::ZeroSize);
    }
    // ...
    Ok(ThreadPool { workers, sender })
}
```

**使い方**:
```rust
let pool = ThreadPool::new(4)?;  // ✅ OK
match ThreadPool::new(0) {
    Ok(p) => { /* 使う */ },
    Err(e) => { /* エラー処理 */ },
}
```

---

### どちらを選ぶべきか？

| 観点 | `assert!` | `Result` |
|------|----------|---------|
| 適している場面 | **プログラミングエラー** | 回復可能なエラー |
| エラー時 | プログラム停止 | エラーを返す |
| 使いやすさ | シンプル | エラー処理が必要 |

**今回の判断**: スレッド数0は**プログラマのミス**なので、`assert!` でパニックさせる方が適切。

---

## Worker 構造体

### Worker の役割

各スレッドを管理する構造体：

```rust
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job.call_box();
                    }
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

**ポイント**:
- `id`: ワーカーの識別番号
- `thread`: スレッドのハンドル（`Option` で包む理由は後述）
- 無限ループでジョブを待ち受ける

---

## チャネル通信（mpsc）

### mpsc とは？

**mpsc** = **M**ultiple **P**roducer, **S**ingle **C**onsumer

```rust
use std::sync::mpsc;

let (sender, receiver) = mpsc::channel();
```

| 役割 | 型 | Clone可能？ | 数 |
|------|-----|-----------|-----|
| **sender** | `Sender<T>` | ✅ Yes | **複数OK** |
| **receiver** | `Receiver<T>` | ❌ No | **1つだけ** |

---

### なぜ receiver はクローンできないのか？

```rust
// sender は Clone できる
let sender1 = sender.clone();  // ✅ OK
let sender2 = sender.clone();  // ✅ OK

// receiver は Clone できない
let receiver1 = receiver.clone();  // ❌ エラー！
// error: the trait `Clone` is not implemented for `Receiver<Job>`
```

**理由**:
- チャンネルから受信するのは**1つのスレッドのみ**
- 複数の receiver があると、どのスレッドがメッセージを受け取るか不明確

---

### 問題: 複数の Worker で receiver を共有したい

```rust
for id in 0..size {
    workers.push(Worker::new(id, receiver));  // ❌ 2回目以降で所有権エラー
}
```

**エラー**:
```
error[E0382]: use of moved value: `receiver`
```

1回目のループで `receiver` の所有権が移動
→ 2回目のループでは使えない

---

### 解決策: Arc<Mutex<Receiver>> を使う

```rust
let (sender, receiver) = mpsc::channel();
let receiver = Arc::new(Mutex::new(receiver));  // ← ポイント！

for id in 0..size {
    let receiver = Arc::clone(&receiver);  // ✅ Arc はクローンできる
    workers.push(Worker::new(id, receiver));
}
```

**なぜ Arc<Mutex<>> が必要？**

| 要素 | 理由 |
|------|------|
| **Arc** | 複数のスレッドで receiver を**所有**する |
| **Mutex** | 一度に**1つのスレッドだけ**が `recv()` を呼び出せる |

---

### receiver.lock().unwrap().recv() の動作

```rust
let message = receiver.lock().unwrap().recv().unwrap();
```

**ステップ**:
1. `receiver.lock()` → `Mutex` をロック、`MutexGuard` を取得
2. `.recv()` → チャネルからメッセージを受信（ブロッキング）
3. `MutexGuard` がスコープを抜けると自動的にロック解放

**重要**: `recv()` 中は他のスレッドは待機する（1つのスレッドだけが受信）

---

## Job 型の定義

### FnBox トレイト

```rust
trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;
```

**なぜ FnBox が必要？**

- `FnOnce` クロージャは `self` を消費する
- `Box<dyn FnOnce()>` を直接呼び出すことはできない
- `FnBox` トレイトで `Box` から取り出して呼び出す

**`dyn` キーワード**:
- Rust 2018以降、トレイトオブジェクトには `dyn` が必要
- `Box<FnBox + Send>` → `Box<dyn FnBox + Send>` ✅

---

## Message 列挙型

### ジョブと終了シグナル

```rust
enum Message {
    NewJob(Job),
    Terminate,
}
```

**用途**:
- `NewJob(Job)`: ワーカーに実行させるジョブ
- `Terminate`: ワーカーに終了を指示

### Worker でのメッセージ処理

```rust
loop {
    let message = receiver.lock().unwrap().recv().unwrap();

    match message {
        Message::NewJob(job) => {
            println!("Worker {} got a job; executing.", id);
            job.call_box();
        }
        Message::Terminate => {
            println!("Worker {} was told to terminate.", id);
            break;  // ループを抜けてスレッド終了
        }
    }
}
```

---

## execute メソッドの実装

```rust
pub fn execute<F>(&self, f: F)
where
    F: FnOnce() + Send + 'static,
{
    let job = Box::new(f);
    self.sender.send(Message::NewJob(job)).unwrap();
}
```

**動作**:
1. クロージャを `Box` に包む
2. `Message::NewJob` でラップ
3. チャネルに送信
4. いずれかの Worker が受信して実行

---

## まとめ

### ThreadPool の構造
- **ThreadPool**: Worker のコレクションと sender を持つ
- **Worker**: 各スレッドを管理、receiver からジョブを受信
- **Message**: NewJob（ジョブ）と Terminate（終了）

### チャネル通信
- **mpsc**: Multiple Producer, Single Consumer
- **sender**: Clone 可能（複数の送信者）
- **receiver**: Clone 不可（単独の受信者）
- **Arc<Mutex<Receiver>>**: 複数スレッドで receiver を共有

### エラーハンドリング
- **assert!**: プログラミングエラーはパニック
- **Result**: 回復可能なエラーは Result を返す
- **doc comment**: パニック条件を `# パニック` セクションで明記

### Job の実行
- `FnBox` トレイトで `Box<dyn FnOnce()>` を呼び出し可能に
- `execute` でジョブをチャネルに送信
- Worker がジョブを受信して実行
