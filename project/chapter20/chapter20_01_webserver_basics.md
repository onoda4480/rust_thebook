# 第20章 Part 1: Webサーバの基礎とプロジェクト構造

## 第20章の目標

**マルチスレッドのWebサーバを構築する**

これまで学んだ内容を統合して実践的なプロジェクトを作成：
- TCPリスナーとストリーム
- HTTPリクエストの解析
- スレッドプールの実装
- チャネルを使ったスレッド間通信
- グレースフルシャットダウン

---

## シングルスレッドWebサーバ

### TCPリスナー

```rust
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
```

**ポイント**:
- `TcpListener::bind()` で指定したアドレスでリッスン開始
- `listener.incoming()` で接続を待ち受ける
- 各接続ごとに `TcpStream` が返される

---

### HTTPリクエストの処理

```rust
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs::File;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        // 200 OK レスポンス
        let mut file = File::open("hello.html").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        // 404 NOT FOUND レスポンス
        let mut file = File::open("404.html").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
```

**HTTPレスポンスの形式**:
```
HTTP/1.1 200 OK\r\n
\r\n
<HTMLコンテンツ>
```

---

### 遅いリクエストのシミュレーション

```rust
use std::thread;
use std::time::Duration;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));  // 5秒スリープ
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

**問題点**: `/sleep` にアクセスすると5秒間サーバー全体が止まる
→ **解決策**: マルチスレッド化が必要！

---

## プロジェクト構造の変更

### ライブラリクレートとバイナリクレートの分離

**変更前**:
```
hello/
└── src/
    └── main.rs  (すべてのコードがここに)
```

**変更後**:
```
hello/
└── src/
    ├── lib.rs       (ThreadPool などのライブラリコード)
    └── bin/
        └── main.rs  (Webサーバーの実行コード)
```

---

### なぜこの構造にするのか？

| ファイル | 役割 | 内容 |
|---------|------|------|
| **lib.rs** | ライブラリクレート | `ThreadPool`, `Worker` などの再利用可能なコード |
| **bin/main.rs** | バイナリクレート | Webサーバーのエントリーポイント |

**利点**:
1. **再利用性**: `ThreadPool` を他のプロジェクトから使える
2. **テスト**: ライブラリコードを簡単にテストできる
3. **責任の分離**: ロジックと実行を分ける

---

### bin/main.rs の書き方

```rust
use hello::ThreadPool;  // ← ライブラリクレートをインポート

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);  // 4つのスレッドを持つプール

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
```

---

## Cargo のクレート構造

### デフォルトのクレート検出

Cargoは以下のパスを自動的に認識します：

| パス | クレート種類 | 用途 |
|------|------------|------|
| `src/lib.rs` | ライブラリクレート | 再利用可能なコード |
| `src/main.rs` | バイナリクレート | メインの実行ファイル |
| `src/bin/*.rs` | バイナリクレート | 追加の実行ファイル |

### 複数のバイナリ

```
src/
├── lib.rs
├── bin/
│   ├── main.rs      (cargo run で実行)
│   ├── server.rs    (cargo run --bin server で実行)
│   └── client.rs    (cargo run --bin client で実行)
```

---

## ThreadPool の理想的なインターフェイス

### 目標

`thread::spawn` と同じような使い勝手にする：

```rust
// thread::spawn の使い方
thread::spawn(|| {
    // 処理
});

// ThreadPool の使い方（同じようにしたい）
pool.execute(|| {
    // 処理
});
```

### ThreadPool の設計

```rust
pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // クロージャを実行する
    }
}
```

**型境界の意味**:
- `FnOnce()`: 引数なしで1回だけ呼び出せるクロージャ
- `Send`: スレッド間で転送できる
- `'static`: プログラムが生きている間有効

---

## まとめ

### シングルスレッドWebサーバ
- `TcpListener` でTCP接続を待ち受ける
- `TcpStream` でデータの読み書き
- HTTPリクエストを解析してレスポンスを返す
- **問題点**: 1つの遅いリクエストが全体を止める

### プロジェクト構造
- **lib.rs**: ライブラリコード（ThreadPool）
- **bin/main.rs**: アプリケーションコード（Webサーバー）
- ライブラリクレートをバイナリクレートから `use` でインポート

### ThreadPool の設計
- `new(size)`: スレッドプールを作成
- `execute(f)`: クロージャをスレッドプールで実行
- `thread::spawn` と同じインターフェイス
