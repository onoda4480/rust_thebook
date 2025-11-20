# 第13章 まとめ (2/3): イテレータ

## イテレータとは

**イテレータ（Iterator）** = 要素を順番に処理するもの

```rust
let v = vec![1, 2, 3];

// イテレータを作成
let mut iter = v.iter();

// 要素を取得
println!("{:?}", iter.next());  // Some(1)
println!("{:?}", iter.next());  // Some(2)
println!("{:?}", iter.next());  // Some(3)
println!("{:?}", iter.next());  // None
```

---

## 3種類のイテレータ

### 1. iter() - 不変参照

```rust
let v = vec![1, 2, 3];

for val in v.iter() {
    //     ^
    //     &i32（参照）
    println!("{}", val);
}

// ✅ v はまだ使える
println!("{:?}", v);
```

**特徴:**
- 借用するだけ
- 所有権を奪わない
- 元の変数は使える

---

### 2. iter_mut() - 可変参照

```rust
let mut v = vec![1, 2, 3];

for val in v.iter_mut() {
    //     ^
    //     &mut i32（可変参照）
    *val += 10;
}

// ✅ v はまだ使える（変更されている）
println!("{:?}", v);  // [11, 12, 13]
```

**特徴:**
- 可変借用
- 値を変更できる
- 元の変数は使える

---

### 3. into_iter() - 所有権を奪う

```rust
let v = vec![1, 2, 3];

for val in v.into_iter() {
    //     ^
    //     i32（所有権あり）
    println!("{}", val);
}

// ❌ v はもう使えない
// println!("{:?}", v);
```

**特徴:**
- 所有権を奪う
- その後、元の変数は使えない

---

## for ループでの自動変換

```rust
let v = vec![1, 2, 3];

// これは into_iter() が呼ばれる（所有権を奪う）
for val in v {
    println!("{}", val);
}

// これは iter() が呼ばれる（借用するだけ）
for val in &v {
    println!("{}", val);
}
```

**注意:** `for val in v` は所有権を奪う！

---

## イテレータアダプタ

**イテレータアダプタ** = イテレータを変換して新しいイテレータを返す

### 1. map() - 各要素を変換

```rust
let v = vec![1, 2, 3];

let v2: Vec<i32> = v.iter()
    .map(|x| x * 2)  // 各要素を2倍
    .collect();

println!("{:?}", v2);  // [2, 4, 6]
```

---

### 2. filter() - 条件に合う要素だけ

```rust
let v = vec![1, 2, 3, 4, 5];

let evens: Vec<&i32> = v.iter()
    .filter(|x| *x % 2 == 0)  // 偶数だけ
    .collect();

println!("{:?}", evens);  // [2, 4]
```

---

### 3. take() - 最初のN個

```rust
let v = vec![1, 2, 3, 4, 5];

let first_three: Vec<&i32> = v.iter()
    .take(3)  // 最初の3個
    .collect();

println!("{:?}", first_three);  // [1, 2, 3]
```

---

### 4. skip() - 最初のN個をスキップ

```rust
let v = vec![1, 2, 3, 4, 5];

let after_two: Vec<&i32> = v.iter()
    .skip(2)  // 最初の2個をスキップ
    .collect();

println!("{:?}", after_two);  // [3, 4, 5]
```

---

### 5. zip() - 2つのイテレータをペアに

```rust
let a = vec![1, 2, 3];
let b = vec!['a', 'b', 'c'];

let pairs: Vec<(&i32, &char)> = a.iter()
    .zip(b.iter())
    .collect();

println!("{:?}", pairs);  // [(1, 'a'), (2, 'b'), (3, 'c')]
```

---

## 消費アダプタ

**消費アダプタ** = イテレータを消費して最終的な値を返す

### 1. sum() - 合計

```rust
let v = vec![1, 2, 3];
let total: i32 = v.iter().sum();
println!("{}", total);  // 6
```

---

### 2. collect() - コレクションに集める

```rust
let v = vec![1, 2, 3];

let v2: Vec<i32> = v.iter()
    .map(|x| x * 2)
    .collect();

println!("{:?}", v2);  // [2, 4, 6]
```

---

### 3. count() - 個数

```rust
let v = vec![1, 2, 3, 4, 5];

let count = v.iter()
    .filter(|x| *x > &2)
    .count();

println!("{}", count);  // 3
```

---

### 4. any() - 1つでも条件を満たす

```rust
let v = vec![1, 2, 3, 4, 5];

let has_even = v.iter().any(|x| x % 2 == 0);
println!("{}", has_even);  // true
```

---

### 5. all() - すべて条件を満たす

```rust
let v = vec![2, 4, 6, 8];

let all_even = v.iter().all(|x| x % 2 == 0);
println!("{}", all_even);  // true
```

---

### 6. find() - 最初の要素を探す

```rust
let v = vec![1, 2, 3, 4, 5];

let first_even = v.iter().find(|x| *x % 2 == 0);
println!("{:?}", first_even);  // Some(2)
```

---

## 遅延評価

### イテレータアダプタは遅延評価

```rust
let v = vec![1, 2, 3];

// まだ何も実行されない
let iter = v.iter()
    .map(|x| {
        println!("mapping {}", x);
        x * 2
    });

println!("Before collect");

// ここで初めて実行される
let result: Vec<i32> = iter.collect();
```

**出力:**
```
Before collect
mapping 1
mapping 2
mapping 3
```

---

## カスタムイテレータ

### Iterator トレイトを実装

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
```

**使用例:**
```rust
let mut counter = Counter::new();

println!("{:?}", counter.next());  // Some(1)
println!("{:?}", counter.next());  // Some(2)
println!("{:?}", counter.next());  // Some(3)
println!("{:?}", counter.next());  // Some(4)
println!("{:?}", counter.next());  // Some(5)
println!("{:?}", counter.next());  // None
```

---

## 実用例

### 1. フィルタリングと変換

```rust
#[derive(Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}
```

---

### 2. 複雑なチェーン

```rust
let sum: u32 = Counter::new()
    .zip(Counter::new().skip(1))
    .map(|(a, b)| a * b)
    .filter(|x| x % 3 == 0)
    .sum();
// 18
```

**ステップ:**
1. zip: (1,2), (2,3), (3,4), (4,5)
2. map: 2, 6, 12, 20
3. filter: 6, 12
4. sum: 18

---

## Python との比較

### Python

```python
# map
doubled = list(map(lambda x: x * 2, [1, 2, 3]))

# filter
evens = list(filter(lambda x: x % 2 == 0, [1, 2, 3, 4, 5]))

# sum
total = sum([1, 2, 3])

# リスト内包表記
result = [x * 2 for x in [1, 2, 3] if x % 2 == 0]
```

---

### Rust

```rust
// map
let doubled: Vec<i32> = vec![1, 2, 3]
    .iter()
    .map(|x| x * 2)
    .collect();

// filter
let evens: Vec<&i32> = vec![1, 2, 3, 4, 5]
    .iter()
    .filter(|x| *x % 2 == 0)
    .collect();

// sum
let total: i32 = vec![1, 2, 3].iter().sum();

// メソッドチェーン
let result: Vec<i32> = vec![1, 2, 3]
    .iter()
    .filter(|x| *x % 2 == 0)
    .map(|x| x * 2)
    .collect();
```

---

## まとめ

```
イテレータ:
✅ 要素を順番に処理
✅ next() で1つずつ取得
✅ 遅延評価

3種類:
iter()      → &T（借用）
iter_mut()  → &mut T（可変借用）
into_iter() → T（所有権を奪う）

イテレータアダプタ（新しいイテレータを返す）:
map()    → 各要素を変換
filter() → 条件に合う要素だけ
take()   → 最初のN個
skip()   → 最初のN個をスキップ
zip()    → 2つをペアに

消費アダプタ（最終的な値を返す）:
sum()     → 合計
collect() → コレクションに集める
count()   → 個数
any()     → 1つでも条件を満たす
all()     → すべて条件を満たす
find()    → 最初の要素を探す

遅延評価:
イテレータアダプタは遅延評価
消費アダプタで初めて実行される

カスタムイテレータ:
Iterator トレイトを実装
next() を定義
```
