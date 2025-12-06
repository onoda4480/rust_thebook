# 第20章 質問まとめ

このドキュメントは、第20章の学習中にあなたが質問した内容とその回答をまとめたものです。

---

## 質問1: ライブラリクレートとバイナリクレートの分離

### あなたの質問

> それから新しいディレクトリ、src/binを作成し、src/main.rsに根付くバイナリクレートをsrc/bin/main.rsに移動してください。 そうすると、ライブラリクレートがhelloディレクトリ内で主要クレートになります; それでも、 cargo runでsrc/bin/main.rsのバイナリを実行することはできます。main.rsファイルを移動後、 編集してライブラリクレートを持ち込み、以下のコードをsrc/bin/main.rsの先頭に追記してThreadPoolをスコープに導入してください:
> って書いてあったがどゆこと？

---

### 回答

The Book が指示しているのは、プロジェクト構造を変更してライブラリクレートとバイナリクレートを分離することです。

#### 変更前の構造
```
hello/
└── src/
    └── main.rs  (すべてのコードがここに)
```

#### 変更後の構造
```
hello/
└── src/
    ├── lib.rs       (ThreadPool などのライブラリコード)
    └── bin/
        └── main.rs  (Webサーバーを実行するコード)
```

#### なぜこうするのか？

| ファイル | 役割 | 内容 |
|---------|------|------|
| **lib.rs** | ライブラリクレート | `ThreadPool`, `Worker` などの再利用可能なコード |
| **bin/main.rs** | バイナリクレート | Webサーバーのエントリーポイント |

**利点**:
1. **再利用性**: `ThreadPool` を他のプロジェクトから使える
2. **テスト**: ライブラリコードを簡単にテストできる
3. **責任の分離**: ロジックと実行を分ける

#### bin/main.rs の書き方

```rust
use hello::ThreadPool;  // ← ライブラリクレートをインポート

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
```

**ポイント**:
- `use hello::ThreadPool;` で lib.rs で定義した `ThreadPool` をインポート
- `hello` はクレート名（Cargo.toml の `[package]` の `name`）
- `cargo run` で `src/bin/main.rs` が実行される

---

## 質問2: assert! vs Result のエラーハンドリング

### あなたの質問

> doc commentでThreadPoolにドキュメンテーションを追加しました。第14章で議論したように、 関数がパニックすることもある場面を声高に叫ぶセクションを追加することで、 いいドキュメンテーションの実践に倣ならっていることに注意してください。 試しにcargo doc --openを実行し、ThreadPool構造体をクリックして、newの生成されるドキュメンテーションがどんな見た目か確かめてください！
>
> ここでしたようにassert!マクロを追加する代わりに、リスト12-9のI/OプロジェクトのConfig::newのように、 newにResultを返させることもできるでしょう。しかし、今回の場合、スレッドなしでスレッドプールを作成しようとするのは、 回復不能なエラーであるべきと決定しました。野心を感じるのなら、以下のシグニチャのnewも書いてみて、両者を比較してみてください:
>
> pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
> どゆこと？

---

### 回答

The Book は ThreadPool のエラーハンドリングについて2つのアプローチを比較しています。

#### アプローチ1: assert! でパニック（採用された方法）

```rust
/// # パニック
///
/// sizeが0なら、`new`関数はパニックします。
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

---

#### アプローチ2: Result を返す（代替案）

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
    Ok(p) => { /* プールを使う */ },
    Err(e) => { /* エラーハンドリング */ },
}
```

---

#### どちらを選ぶべきか？

| 観点 | `assert!`（パニック） | `Result`（エラー返す） |
|------|---------------------|---------------------|
| 使い方 | `ThreadPool::new(4)` | `ThreadPool::new(4)?` |
| エラー時 | プログラム停止 | エラーを返す |
| 適している場面 | **プログラミングエラー** | 回復可能なエラー |

**The Book の判断**:

> スレッドなしでスレッドプールを作成しようとするのは、**回復不能なエラー**であるべきと決定しました。

**理由**:
- スレッド数0は**プログラマのミス**
- パニックで即座に気づかせる方が良い
- 回復する意味がない（スレッド0のプールは使い物にならない）

**doc comment で明記**:
```rust
/// # パニック
///
/// sizeが0なら、`new`関数はパニックします。
```

これにより、利用者はドキュメントでパニック条件を確認できる。

---

## 質問3: mpsc と receiver.clone() できない理由

### あなたの質問

> Rustが提供するチャンネル実装は、複数の生成者、単独の消費者です。要するに、 チャンネルの消費側をクローンするだけでこのコードを修正することはできません。
> どゆこと？

---

### 回答

Rust のチャンネルの特性について説明している部分です。

#### mpsc の意味

**mpsc** = **M**ultiple **P**roducer, **S**ingle **C**onsumer（複数生成者、単独消費者）

```rust
use std::sync::mpsc;

let (sender, receiver) = mpsc::channel();
```

| 役割 | 型 | Clone可能？ | 数 |
|------|-----|-----------|-----|
| **sender** (送信側) | `Sender<T>` | ✅ Yes | **複数OK** |
| **receiver** (受信側) | `Receiver<T>` | ❌ No | **1つだけ** |

---

#### なぜ receiver はクローンできないのか？

**sender は Clone できる**（Multiple Producer）:
```rust
let sender1 = sender.clone();  // ✅ OK
let sender2 = sender.clone();  // ✅ OK

// 複数のスレッドから送信できる
thread::spawn(move || sender1.send(job1));
thread::spawn(move || sender2.send(job2));
```

**receiver は Clone できない**（Single Consumer）:
```rust
let receiver1 = receiver.clone();  // ❌ コンパイルエラー！
// error: the trait `Clone` is not implemented for `Receiver<Job>`
```

**理由**:
- チャンネルから受信するのは**1つのスレッドのみ**
- 複数の receiver があると、どっちがメッセージを受け取るか不明確になる

---

#### 問題: 複数の Worker に receiver を渡したい

```rust
let (sender, receiver) = mpsc::channel();

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

#### 解決策: Arc<Mutex<Receiver>> を使う

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
| **Mutex** | 一度に**1つのスレッドだけ**が `recv()` を呼び出せるようにする |

**Worker での使い方**:
```rust
let message = receiver.lock().unwrap().recv().unwrap();
//            ^^^^^^^ Mutex をロック
//                              ^^^^^^ メッセージを受信
```

**まとめ**:

The Book が言っているのは：
> 「receiver.clone() するだけでは解決できない。なぜなら Receiver は Clone トレイトを実装していないから。代わりに Arc<Mutex<>> を使う必要がある」

---

## 質問4: コンパイルエラー（Job の重複定義と dyn キーワード）

### あなたの質問

（コンパイルエラーを提示）

```
error[E0428]: the name `Job` is defined multiple times
error[E0782]: expected a type, found a trait
error[E0308]: mismatched types
error[E0599]: no method named `call_box` found
```

> どゆこと？

---

### 回答

エラーを1つずつ解説します。

#### エラー1: `Job` が2回定義されている

```rust
10 | struct Job;                              // ← これは不要！削除
...
21 | type Job = Box<FnBox + Send + 'static>;  // ← これが正しい定義
```

**問題**: `Job` という名前が2回定義されている

**修正**: 10行目の `struct Job;` を削除

---

#### エラー2: `dyn` キーワードが必要

```rust
21 | type Job = Box<FnBox + Send + 'static>;  // ❌ 古い書き方
```

Rust 2018以降、トレイトオブジェクトには `dyn` キーワードが必要：

```rust
21 | type Job = Box<dyn FnBox + Send + 'static>;  // ✅ 正しい
```

**理由**:
- `dyn` = "dynamic dispatch"（動的ディスパッチ）
- トレイトオブジェクトであることを明示する
- Rust 2015では不要だったが、2018以降は必須

---

#### 修正後のコード

```rust
// struct Job;  ← 削除

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;  // ✅ dyn を追加
```

これでコンパイルエラーが解消されます！

---

## まとめ

### 主な学習ポイント

1. **プロジェクト構造**
   - lib.rs: ライブラリコード
   - bin/main.rs: バイナリコード
   - `cargo run` で bin/main.rs が実行される

2. **エラーハンドリング**
   - プログラミングエラー → `assert!` でパニック
   - 回復可能なエラー → `Result` を返す
   - doc comment で `# パニック` セクションを追加

3. **mpsc チャネル**
   - Multiple Producer, Single Consumer
   - sender は Clone 可能、receiver は Clone 不可
   - `Arc<Mutex<Receiver>>` で複数スレッドで共有

4. **Rust 2018 の変更**
   - トレイトオブジェクトに `dyn` キーワードが必要
   - `Box<Trait>` → `Box<dyn Trait>`

これらの質問を通じて、第20章の核心部分を深く理解できました！
