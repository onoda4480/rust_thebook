# Chapter 8 Q&A Part 3: ハッシュマップ

## Q1: HashMap が標準で use が必要な理由は？

**質問:** Vec や String は use なしで使えるのに、HashMap だけ `use std::collections::HashMap;` が必要なのはなぜ？

**回答:** **prelude に含まれていないからです。**

---

### Prelude とは？

**Rust が自動的にインポートする標準ライブラリの一部**

```rust
// これらは自動的に使える（use 不要）
let v = Vec::new();
let s = String::new();
let opt = Some(5);
let res: Result<i32, ()> = Ok(10);
```

---

### HashMap は prelude にない

```rust
// ❌ エラー
// let map = HashMap::new();

// ✅ OK
use std::collections::HashMap;
let map = HashMap::new();
```

**理由:** 全てのプログラムで使うわけではないから

---

## Q2: なぜ map["Blue"] でアクセスできない？

**質問:** `map["Blue"]` でアクセスしようとするとエラーになるのはなぜ？

**回答:** **キーの型が違うからです。**

---

### 問題のコード

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert(String::from("Blue"), 10);

// ❌ エラー
// let score = map["Blue"];
```

**エラー:**
```
error: the type `HashMap<String, i32>` cannot be indexed by `&str`
```

---

### 理由

```
map のキー: String 型
"Blue": &str 型

型が違う！
```

---

### 解決策1: `get()` を使う（推奨）

```rust
if let Some(score) = map.get("Blue") {
    println!("{}", score);
}
```

**`get()` は自動的に型変換してくれる**

---

### 解決策2: String を作る

```rust
let score = &map[&String::from("Blue")];
println!("{}", score);
```

**面倒！**

---

### 解決策3: 最初から &str をキーにする

```rust
let mut map = HashMap::new();
map.insert("Blue", 10);  // &str をキー

let score = &map["Blue"];  // ✅ OK
```

---

## Q3: insert 後に元の変数が使えないのはなぜ？

**質問:** `map.insert(key, value)` の後、key と value が使えないのはなぜ？

**回答:** **所有権が HashMap に移動したからです。**

---

### 問題のコード

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);

// ❌ エラー！
// println!("{}", field_name);
// println!("{}", field_value);
```

**エラー:**
```
error: borrow of moved value: `field_name`
```

---

### なぜ移動する？

**HashMap の `insert()` は所有権を取る**

```rust
pub fn insert(&mut self, key: K, value: V) -> Option<V>
//                        ^      ^
//                        所有権を取る
```

**理由:**
- HashMap が値を保持し続ける必要がある
- 外部で値が変更されたら困る
- だから所有権を奪う

---

### 解決策1: `get()` で HashMap から取得

```rust
let mut map = HashMap::new();
map.insert(String::from("Blue"), 10);

// ✅ get() で取得
if let Some(value) = map.get("Blue") {
    println!("{}", value);
}
```

---

### 解決策2: `clone()` して残す

```rust
let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name.clone(), field_value.clone());

// ✅ OK！clone したので残っている
println!("{}", field_name);
println!("{}", field_value);
```

---

### Copy 型の場合

```rust
let key = 10;
let value = 20;

let mut map = HashMap::new();
map.insert(key, value);

// ✅ OK！i32 は Copy トレイト
println!("{}", key);
println!("{}", value);
```

---

## Q4: 参照を insert したらどうなる？

**質問:** 値ではなく参照を insert したら？

**回答:** **所有権は移動しませんが、ライフタイムの制約があります。**

---

### 参照を insert

```rust
let key = String::from("Blue");
let value = 10;

let mut map = HashMap::new();
map.insert(&key, &value);  // 参照を挿入

// ✅ OK！所有権は移動しない
println!("{}", key);
println!("{}", value);
```

---

### ライフタイムの制約

```rust
let mut map = HashMap::new();

{
    let key = String::from("Blue");
    let value = 10;

    map.insert(&key, &value);

}  // ← key と value がスコープを抜ける

// ❌ エラー！map が無効な参照を持っている
// println!("{:?}", map);
```

**エラー:**
```
error: `key` does not live long enough
error: `value` does not live long enough
```

---

### ルール

```
参照を insert する場合:
元の値は HashMap より長生きする必要がある
```

---

## Q5: entry().or_insert() とは？

**質問:** `entry().or_insert()` は何をしている？

**回答:** **キーが存在しない場合のみ値を挿入します。**

---

### 基本的な使い方

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

// "Blue" が存在しないので 50 を挿入
scores.entry(String::from("Blue")).or_insert(50);

// "Blue" が存在するので何もしない
scores.entry(String::from("Blue")).or_insert(100);

println!("{:?}", scores);  // {"Blue": 50}
```

---

### Python との比較

#### Python

```python
scores = {}

# setdefault: キーが存在しない場合のみ設定
scores.setdefault("Blue", 50)
scores.setdefault("Blue", 100)

print(scores)  # {'Blue': 50}
```

---

#### Rust

```rust
let mut scores = HashMap::new();
scores.entry("Blue".to_string()).or_insert(50);
scores.entry("Blue".to_string()).or_insert(100);
```

---

### `or_insert()` の戻り値

**可変参照 `&mut V` を返す**

```rust
let mut scores = HashMap::new();

let value = scores.entry(String::from("Blue")).or_insert(50);
*value += 10;  // 値を更新

println!("{:?}", scores);  // {"Blue": 60}
```

---

## Q6: 単語カウントのコードを理解したい

**質問:** 単語カウントのコードはどう動いている？

```rust
let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
```

**回答:** **ステップバイステップで見てみましょう。**

---

### ステップ1: "hello"

```rust
// 1回目: word = "hello"
let count = map.entry("hello").or_insert(0);
// "hello" が存在しないので 0 を挿入
// count = &mut 0

*count += 1;
// count を 1 に更新

// map = {"hello": 1}
```

---

### ステップ2: "world"

```rust
// 2回目: word = "world"
let count = map.entry("world").or_insert(0);
// "world" が存在しないので 0 を挿入
// count = &mut 0

*count += 1;
// count を 1 に更新

// map = {"hello": 1, "world": 1}
```

---

### ステップ3: "wonderful"

```rust
// 3回目: word = "wonderful"
let count = map.entry("wonderful").or_insert(0);
// "wonderful" が存在しないので 0 を挿入
// count = &mut 0

*count += 1;
// count を 1 に更新

// map = {"hello": 1, "world": 1, "wonderful": 1}
```

---

### ステップ4: "world" (2回目)

```rust
// 4回目: word = "world"
let count = map.entry("world").or_insert(0);
// "world" が既に存在するので既存の値への参照を返す
// count = &mut 1

*count += 1;
// count を 2 に更新

// map = {"hello": 1, "world": 2, "wonderful": 1}
```

---

### Python で書くと

```python
text = "hello world wonderful world"
word_count = {}

for word in text.split():
    word_count[word] = word_count.get(word, 0) + 1

# または
from collections import defaultdict
word_count = defaultdict(int)
for word in text.split():
    word_count[word] += 1
```

---

## Q7: HashMap の順序は？

**質問:** HashMap の要素の順序は保証されている？

**回答:** **いいえ、順序は保証されていません。**

---

### 順序は不定

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("a", 1);
map.insert("b", 2);
map.insert("c", 3);

for (k, v) in &map {
    println!("{}: {}", k, v);
}
// 出力順序は不定
// b: 2
// a: 1
// c: 3
// など（実行ごとに変わる可能性）
```

---

### 順序が必要な場合

**BTreeMap を使う**

```rust
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
map.insert("c", 3);
map.insert("a", 1);
map.insert("b", 2);

for (k, v) in &map {
    println!("{}: {}", k, v);
}
// a: 1
// b: 2
// c: 3
// ✅ キーの順序で並ぶ
```

---

### Python との比較

#### Python 3.7+

```python
# Python 3.7+ では挿入順序が保持される
d = {}
d["c"] = 3
d["a"] = 1
d["b"] = 2

for k, v in d.items():
    print(f"{k}: {v}")
# c: 3
# a: 1
# b: 2
# ✅ 挿入順
```

---

#### Rust

```rust
// HashMap: 順序不定
// BTreeMap: キーの順序
// 挿入順序が必要なら外部クレート（indexmap）
```

---

## まとめ

### HashMap の重要なポイント

```
✅ use std::collections::HashMap; が必要
✅ get() で安全にアクセス
✅ insert() は所有権を取る
✅ entry().or_insert() で条件付き挿入
✅ 順序は保証されない
```

---

### 所有権のルール

```
値を insert:
✅ 所有権が移動（Copy でない型）
❌ 元の変数は使えない

参照を insert:
✅ 所有権は移動しない
✅ 元の変数も使える
❌ HashMap より長生きする必要

Copy 型を insert:
✅ コピーされる
✅ 元の変数も使える
```

---

### entry API

```rust
// 存在しない場合のみ挿入
map.entry(key).or_insert(default);

// 古い値に基づいて更新
let count = map.entry(key).or_insert(0);
*count += 1;
```

---

### Python との対応

| 操作 | Python | Rust |
|------|--------|------|
| **作成** | `d = {}` | `HashMap::new()` |
| **挿入** | `d[k] = v` | `map.insert(k, v)` |
| **取得** | `d.get(k)` | `map.get(&k)` |
| **デフォルト** | `d.setdefault(k, v)` | `map.entry(k).or_insert(v)` |
| **存在確認** | `k in d` | `map.contains_key(&k)` |
| **削除** | `del d[k]` | `map.remove(&k)` |

---

### ベストプラクティス

```
✅ get() で安全にアクセス
✅ entry().or_insert() を活用
✅ 所有権の移動に注意
✅ clone() は必要な時だけ
✅ 順序が必要なら BTreeMap
```
