# Chapter 16 Q&A Part 2: 共有状態と並行性のミス

## Q5: send() が所有権を奪うのはなぜ？

**質問:** `tx.send(val)` の後に `val` を使おうとするとエラーになるけど、これは並行性のミス？

**回答:** ✅ **本質的には並行性のミス！所有権システムがコンパイル時に防いでいる**

---

### コードの例

```rust
let val = String::from("hi");
tx.send(val).unwrap();     // val の所有権が移動
println!("val is {}", val); // ❌ エラー！
```

**エラー:**
```
error[E0382]: borrow of moved value: `val`
```

---

### なぜ並行性のミス？

#### もし send 後に使えたら...

```rust
// スレッド1
let mut val = String::from("hi");
tx.send(val).unwrap();
val.push_str(" world");  // もしこれができたら...

// スレッド2（受信側）
let mut received = rx.recv().unwrap();
received.push_str("!");  // 同時に変更

// 両方とも同じメモリを変更 → データ競合！💥
```

---

### Rust の所有権システムが防ぐ

```rust
tx.send(val).unwrap();  // 所有権が移動
val.push_str(" world"); // ❌ コンパイルエラー
```

**重要:**
- **表面:** 所有権のエラー
- **本質:** 並行性のミス（データ競合の可能性）
- **Rust:** 所有権システムで並行性の安全性を保証

---

### 他の言語との比較

#### C言語だったら（危険）
```c
char* val = "hi";
send(channel, val);
printf("val is %s\n", val);  // ✅ コンパイルできる
// でも実行時にクラッシュするかもしれない💥
```

#### Rust（安全）
```rust
let val = String::from("hi");
tx.send(val).unwrap();
println!("val is {}", val);  // ❌ コンパイルエラー
// 実行前に問題を発見！
```

---

## Q6: Mutex の説明を噛み砕いて

**質問:** Mutex の説明が難しい。もっと簡単に教えて。

**回答:** **1度に1人しか使えない箱、自動でカギが返される**

---

### トイレの例え

```
Mutex = トイレ 🚻
データ = トイレットペーパー 🧻

1. トイレに入る（ロック取得）🔓
2. トイレットペーパーを使う（データにアクセス）
3. トイレから出る（ロック解放）🔒

他の人が使っている時は待つ⏳
```

---

### 4ステップ

```rust
// 1. 作成
let m = Mutex::new(5);  // 箱に5を入れる🔒

{
    // 2. ロック取得
    let mut num = m.lock().unwrap();  // カギを取る🔓

    // 3. 変更
    *num = 6;  // 中身を変更

}  // 4. 自動解放（Drop）🔒

println!("m = {:?}", m);  // Mutex { data: 6 }
```

---

### 重要なポイント

1. **型システムが保護**: ロックなしでアクセスできない
2. **自動ロック解放**: Drop トレイトで自動
3. **ブロック**: 他のスレッドが使っていたら待つ

---

## Q7: なぜ `*num` で参照外し？

**質問:** `*num = 6` の `*` はなぜ必要？

**回答:** **num は MutexGuard というスマートポインタだから**

---

### 型を追う

```rust
let m = Mutex::new(5);          // Mutex<i32>
let num = m.lock().unwrap();    // MutexGuard<i32>
*num = 6;                       // i32 に代入
```

---

### MutexGuard はスマートポインタ

```
num  →  MutexGuard  →  i32 (5)
```

**Deref トレイトを実装**
- `*num` で参照外しして `i32` にアクセス

---

### もし `*` がなかったら？

```rust
num = 6;  // ❌ エラー！

// エラー: expected `MutexGuard`, found integer `6`
```

**理由:** `num` は `MutexGuard<i32>` 型、`6` は `i32` 型

---

### Box との比較

```rust
// Box も同じ
let mut b = Box::new(5);
*b = 6;  // ← * が必要

// 通常の変数
let mut x = 5;
x = 6;  // ← * は不要
```

---

## Q8: MutexGuard は参照？

**質問:** MutexGuard<i32> は参照だからその中身にアクセスするなら * が必要ってことだよね？

**回答:** **概念的にはその理解でOK！正確にはスマートポインタ**

---

### 正確には

#### ❌ 「MutexGuard は参照」
```
MutexGuard<i32> = 参照 ではない
```

#### ✅ 「MutexGuard は参照のように振る舞うスマートポインタ」
```
MutexGuard<i32> = スマートポインタ（構造体）
                  ↓ Deref トレイト
                &i32 のように振る舞う
```

---

### 実用上は

**あなたの理解で十分です！** ✅

**実際の動き:**
1. MutexGuard は内部に参照を持っている
2. `*` で参照外しすると中身にアクセスできる
3. 使い方は参照と同じ

---

## Q9: Arc と Rc の違いは？

**質問:** Arc と Rc の違いは？

**回答:** **スレッド安全かどうか**

---

### 比較表

| 項目 | Rc | Arc |
|------|-----|-----|
| **用途** | シングルスレッド | **マルチスレッド** |
| **スレッド安全** | ❌ | ✅ |
| **パフォーマンス** | 速い | 少し遅い |
| **名前の意味** | Reference Counted | **Atomically** Reference Counted |

---

### なぜ Rc はダメ？

```rust
let counter = Rc::new(Mutex::new(0));

thread::spawn(move || {
    // ...
});
// ❌ エラー！
// `Rc<Mutex<i32>>` cannot be sent between threads safely
```

**理由:** Rc の参照カウントがスレッド安全でない

---

### Arc なら OK

```rust
let counter = Arc::new(Mutex::new(0));

thread::spawn(move || {
    // ...
});
// ✅ OK！
```

**理由:** Arc はアトミック操作でスレッド安全

---

### コストの違い

#### Rc（速い）
```rust
count += 1;  // 単純な加算
```

#### Arc（少し遅い）
```rust
count.fetch_add(1, Ordering::SeqCst);  // アトミック操作
```

**でも実用上は気にならないレベル**

---

## まとめ

| 質問 | 答え |
|------|------|
| send() が所有権を奪う理由 | データ競合を防ぐため |
| これは並行性のミス？ | ✅ 本質的には YES |
| Mutex とは | 1度に1人しか使えない箱 |
| なぜ *num？ | MutexGuard がスマートポインタだから |
| MutexGuard は参照？ | 概念的には YES、正確にはスマートポインタ |
| Arc vs Rc | スレッド安全 vs シングルスレッド |
