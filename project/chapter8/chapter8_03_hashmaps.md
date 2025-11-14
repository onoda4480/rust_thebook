# Chapter 8-3: ハッシュマップ (HashMap<K, V>)

## ハッシュマップとは？

**キーと値のペアを格納するコレクション**

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

---

## Python との対応

| Rust | Python |
|------|--------|
| `HashMap<K, V>` | `dict` |
| `map.insert(k, v)` | `d[k] = v` |
| `map.get(&k)` | `d.get(k)` |
| `map.contains_key(&k)` | `k in d` |

---

## ハッシュマップの作成

### 方法1: `HashMap::new()`

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

**注意:** `use std::collections::HashMap;` が必要

---

### 方法2: `collect()` を使う

```rust
use std::collections::HashMap;

let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> =
    teams.iter().zip(initial_scores.iter()).collect();
```

**ポイント:**
- `zip()` で2つのベクタを組み合わせる
- `collect()` で HashMap に変換
- `HashMap<_, _>` で型推論

---

## 値の取得

### `get()` メソッド

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

let team_name = String::from("Blue");
let score = scores.get(&team_name);
// score = Some(&10)

match score {
    Some(s) => println!("Score: {}", s),
    None => println!("Team not found"),
}
```

**戻り値:** `Option<&V>`

---

### 添え字アクセス

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

// ❌ エラー: 型が違う
// let score = scores["Blue"];  // &str は使えない

// ✅ OK: String を使う
let score = &scores[&String::from("Blue")];  // 10
```

**注意:**
- キーの型と一致する必要がある
- キーが存在しないとパニック

---

## 値の更新

### パターン1: 上書き

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);  // 上書き

println!("{:?}", scores);  // {"Blue": 25}
```

---

### パターン2: キーが存在しない場合のみ挿入

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

// Blue が存在しないなら挿入（既に存在するので何もしない）
scores.entry(String::from("Blue")).or_insert(50);
// Yellow は存在しないので挿入
scores.entry(String::from("Yellow")).or_insert(50);

println!("{:?}", scores);
// {"Blue": 10, "Yellow": 50}
```

**ポイント:**
- `entry()` でエントリを取得
- `or_insert()` で存在しない場合のみ挿入

---

### パターン3: 古い値に基づいて更新

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
// {"hello": 1, "world": 2, "wonderful": 1}
```

**ポイント:**
- `or_insert()` は `&mut V` を返す
- `*count` で参照を外して値を更新

---

## 所有権

### 所有権を取る型

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// ← field_name と field_value の所有権が map に移動

// ❌ エラー！
// println!("{}", field_name);
// println!("{}", field_value);
```

**String などの Copy でない型は所有権が移動**

---

### 参照を挿入

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(&field_name, &field_value);  // 参照を挿入

// ✅ OK！所有権は移動しない
println!("{}", field_name);
println!("{}", field_value);
```

**制約:** 参照が指す値は HashMap より長生きする必要がある

---

### Copy 型

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
let key = 10;
let value = 20;

map.insert(key, value);  // Copy される

// ✅ OK！i32 は Copy トレイト
println!("{}", key);
println!("{}", value);
```

---

## イテレーション

### キーと値を走査

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
// Blue: 10
// Yellow: 50
```

**注意:** 順序は保証されない

---

## その他のメソッド

### `contains_key()` - キーの存在確認

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

if scores.contains_key("Blue") {
    println!("Blue team exists!");
}
```

---

### `remove()` - 削除

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.remove("Blue");
// scores = {}
```

---

### `len()` - 要素数

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

println!("Length: {}", scores.len());  // 2
```

---

### `clear()` - 全削除

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.clear();
// scores = {}
```

---

## entry API

### `entry()` とは？

**キーがあるかどうかで処理を分けるための API**

```rust
pub enum Entry<'a, K, V> {
    Occupied(OccupiedEntry<'a, K, V>),  // キーが存在
    Vacant(VacantEntry<'a, K, V>),      // キーが存在しない
}
```

---

### `or_insert()` - デフォルト値で挿入

```rust
let mut scores = HashMap::new();

// Blue が存在しないので 50 を挿入
scores.entry(String::from("Blue")).or_insert(50);

// Blue が存在するので何もしない
scores.entry(String::from("Blue")).or_insert(100);

println!("{:?}", scores);  // {"Blue": 50}
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

### 単語カウントの例

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
// {"hello": 1, "world": 2, "wonderful": 1}
```

**動作:**
1. `entry(word)` でエントリを取得
2. `or_insert(0)` でキーが存在しなければ 0 を挿入、存在すれば既存の値への可変参照を返す
3. `*count += 1` で値をインクリメント

---

## Python との比較

### Python

```python
# 作成
scores = {"Blue": 10, "Yellow": 50}

# アクセス
score = scores.get("Blue")  # 10
score = scores["Blue"]       # 10

# 更新
scores["Blue"] = 25

# デフォルト値で取得
score = scores.get("Green", 0)

# 存在しない場合のみ挿入
scores.setdefault("Green", 0)

# 単語カウント
from collections import Counter
text = "hello world wonderful world"
counts = Counter(text.split())
```

---

### Rust

```rust
use std::collections::HashMap;

// 作成
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// アクセス
let score = scores.get("Blue");  // Some(&10)
let score = &scores[&String::from("Blue")];  // &10

// 更新
scores.insert(String::from("Blue"), 25);

// 存在しない場合のみ挿入
scores.entry(String::from("Green")).or_insert(0);

// 単語カウント
let text = "hello world wonderful world";
let mut map = HashMap::new();
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
```

---

## ハッシュ関数

### デフォルト: SipHash

**Rust はデフォルトで SipHash を使用**

- セキュアだが少し遅い
- DoS 攻撃に強い

---

### カスタムハッシュ関数

```rust
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::collections::hash_map::DefaultHasher;

let mut map: HashMap<i32, i32, BuildHasherDefault<DefaultHasher>> =
    HashMap::default();
```

**通常は不要**

---

## まとめ

### HashMap の特徴

```
✅ キーと値のペア
✅ キーは一意
✅ 順序は保証されない
✅ ヒープに格納
✅ 所有権のルールが適用される
```

---

### 基本操作

```rust
use std::collections::HashMap;

// 作成
let mut map = HashMap::new();

// 挿入
map.insert(String::from("Blue"), 10);

// 取得
let value = map.get("Blue");  // Option<&V>
let value = &map[&String::from("Blue")];  // &V (パニックの可能性)

// 更新
map.insert(String::from("Blue"), 25);  // 上書き

// 存在チェック
map.contains_key("Blue");

// 削除
map.remove("Blue");

// イテレーション
for (key, value) in &map {
    println!("{}: {}", key, value);
}
```

---

### entry API

```rust
// 存在しない場合のみ挿入
map.entry(key).or_insert(default_value);

// 古い値に基づいて更新
let count = map.entry(key).or_insert(0);
*count += 1;
```

---

### 所有権

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

### ベストプラクティス

```
✅ get() で安全にアクセス
✅ entry().or_insert() で存在チェック
✅ 所有権の移動に注意
✅ 型推論を活用
```
