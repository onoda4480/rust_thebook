# 第20章 Part 3: グレースフルシャットダウン

## グレースフルシャットダウンとは？

**グレースフルシャットダウン**: プログラム終了時に、実行中のタスクを完了させてからクリーンに終了すること

**対義語**: アブラプト（abrupt）シャットダウン = 即座に強制終了

---

## Drop トレイトの実装

### Drop トレイトとは？

値がスコープから抜けるときに自動的に呼ばれる：

```rust
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // ThreadPool が破棄されるときに呼ばれる
    }
}
```

**いつ呼ばれる？**
- 変数がスコープから抜けるとき
- プログラムが終了するとき

---

## グレースフルシャットダウンの実装

### 完成形のコード

```rust
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        // 1. すべての Worker に終了メッセージを送信
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        // 2. すべての Worker のスレッドを join
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

---

## 実装の詳細解説

### ステップ1: 終了メッセージを送信

```rust
for _ in &self.workers {
    self.sender.send(Message::Terminate).unwrap();
}
```

**ポイント**:
- すべての Worker に `Terminate` メッセージを送信
- 各 Worker は `Terminate` を受信したらループを抜ける

**なぜ2つのループ？**

❌ **間違った実装**（1つのループ）:
```rust
for worker in &mut self.workers {
    self.sender.send(Message::Terminate).unwrap();
    worker.thread.join().unwrap();  // ← ここで待ってしまう
}
```

**問題**:
- Worker 0 に Terminate を送信 → Worker 0 の終了を待つ
- この間、Worker 1, 2, 3 はまだジョブを処理している可能性
- 順番に1つずつ終了させることになり、並列性が失われる

✅ **正しい実装**（2つのループ）:
```rust
// ループ1: すべてに Terminate を送信
for _ in &self.workers {
    self.sender.send(Message::Terminate).unwrap();
}

// ループ2: すべての終了を待つ
for worker in &mut self.workers {
    worker.thread.join().unwrap();
}
```

**効果**: すべての Worker が並行して終了処理を行える

---

### ステップ2: スレッドを join

```rust
for worker in &mut self.workers {
    println!("Shutting down worker {}", worker.id);

    if let Some(thread) = worker.thread.take() {
        thread.join().unwrap();
    }
}
```

---

### なぜ Option<JoinHandle> を使うのか？

```rust
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,  // ← なぜ Option？
}
```

**理由**: `join()` は `JoinHandle` の所有権を消費する

---

#### 問題: join() は所有権を消費する

```rust
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,  // Option なし
}

// drop 内で join しようとすると...
for worker in &mut self.workers {
    worker.thread.join().unwrap();  // ❌ エラー！
}
```

**エラー**:
```
error[E0507]: cannot move out of borrowed content
```

`&mut self.workers` は借用なので、所有権を移動できない。

---

#### 解決策: Option::take() を使う

```rust
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,  // ✅ Option で包む
}

// drop 内で
if let Some(thread) = worker.thread.take() {
    thread.join().unwrap();
}
```

**`take()` の動作**:
```rust
pub fn take(&mut self) -> Option<T>
```

- `Some(T)` から `T` を取り出して返す
- 元の `Option` は `None` になる

**例**:
```rust
let mut opt = Some(5);
let value = opt.take();  // value = Some(5), opt = None
```

---

### Worker での処理

```rust
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
                        break;  // ← ループを抜けてスレッド終了
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),  // ✅ Some で包む
        }
    }
}
```

---

## 実行の流れ

### 1. プログラム終了時

```rust
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {  // 2つの接続だけ処理
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    // ← ここで pool がスコープから抜ける
    // → Drop::drop が自動的に呼ばれる
}
```

---

### 2. Drop::drop の実行

```
1. "Sending terminate message to all workers." を出力
2. Worker 0, 1, 2, 3 に Terminate メッセージを送信
3. "Shutting down all workers." を出力
4. Worker 0 のスレッドを join（終了を待つ）
   - "Shutting down worker 0" を出力
5. Worker 1 のスレッドを join
   - "Shutting down worker 1" を出力
6. Worker 2 のスレッドを join
   - "Shutting down worker 2" を出力
7. Worker 3 のスレッドを join
   - "Shutting down worker 3" を出力
8. プログラム終了
```

---

### 3. 実際の出力例

```
$ cargo run
   Compiling hello v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 1.0s
     Running `target/debug/hello`
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Sending terminate message to all workers.
Shutting down all workers.
Worker 0 was told to terminate.
Worker 1 was told to terminate.
Worker 2 was told to terminate.
Worker 3 was told to terminate.
Shutting down worker 0
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```

---

## まとめ

### Drop トレイト
- 値がスコープから抜けるときに自動的に呼ばれる
- リソースのクリーンアップに使う
- `ThreadPool` の場合、すべてのスレッドを安全に終了させる

### グレースフルシャットダウンの手順
1. すべての Worker に `Terminate` メッセージを送信
2. すべての Worker のスレッドを `join()` で待つ
3. 並行して終了処理を行うため、2つのループに分ける

### Option::take()
- `Option<T>` から `T` を取り出す
- 元の `Option` は `None` になる
- 所有権を消費するメソッド（`join()` など）を呼ぶときに便利

### 2つのループが必要な理由
- **ループ1**: すべてに Terminate を送信（並行処理を維持）
- **ループ2**: すべての終了を待つ（順番に join）
- 1つのループだと順次処理になり、並列性が失われる

---

# 第20章で学んだこと全体のまとめ

## 技術要素

### ネットワーク
- `TcpListener` と `TcpStream`
- HTTPプロトコルの基礎
- リクエスト/レスポンスの処理

### 並行プログラミング
- スレッドプール
- チャネル通信（mpsc）
- `Arc<Mutex<T>>` による共有状態
- グレースフルシャットダウン

### Rustの機能
- トレイトオブジェクト（`Box<dyn Trait>`）
- `Drop` トレイト
- `Option::take()`
- ライブラリクレートとバイナリクレートの分離

---

## 設計パターン

### スレッドプールパターン
- 固定数のワーカースレッドを事前作成
- チャネルでジョブを配布
- ワーカーはジョブを待ち受けて実行

### メッセージパッシング
- スレッド間でデータを共有せず、メッセージで通信
- `mpsc::channel` を使った実装
- 終了シグナルもメッセージとして送信

### RAIIパターン
- `Drop` トレイトでリソースを自動的にクリーンアップ
- スコープベースのリソース管理

---

## 第20章の意義

これまで学んだ内容を統合：
- **所有権**: スレッド間での所有権の移動
- **トレイト**: `Drop`, `FnOnce`, `Send`, `Sync`
- **エラーハンドリング**: `Result`, `Option`, `unwrap()`
- **並行性**: `Arc`, `Mutex`, チャネル
- **モジュール**: ライブラリクレートとバイナリクレート

実践的なプロジェクトを通じて、Rustの強みを実感！
