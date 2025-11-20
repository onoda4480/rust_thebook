# 第13章 まとめ (3/3): パフォーマンスとゼロコスト抽象化

## ゼロコスト抽象化とは

**ゼロコスト抽象化（Zero-Cost Abstraction）** = 高レベルで書いても、低レベルと同じ速度

```
高レベルなコード（読みやすい）
     ↓
コンパイラが最適化
     ↓
低レベルなコード（高速）

実行時のオーバーヘッドなし！
```

---

## パフォーマンス比較

### ベンチマーク結果

```
forループ:    19,620,300 ns/iter
イテレータ:    19,234,900 ns/iter
```

**イテレータの方が少し速い！**

---

## 具体例

### 1. 合計の計算

#### forループ

```rust
let mut sum = 0;
for i in 0..1000 {
    sum += i;
}
```

---

#### イテレータ

```rust
let sum: i32 = (0..1000).sum();
```

**結果:** ほぼ同じ速度（イテレータが少し速いことも）

---

### 2. フィルタと変換

#### forループ

```rust
let mut result = Vec::new();
for i in 0..1000 {
    if i % 2 == 0 {
        result.push(i * 2);
    }
}
```

---

#### イテレータ

```rust
let result: Vec<i32> = (0..1000)
    .filter(|x| x % 2 == 0)
    .map(|x| x * 2)
    .collect();
```

**結果:** ほぼ同じ速度

---

## コンパイラの最適化

### 1. ループ展開（Loop Unrolling）

#### あなたが書くコード

```rust
let sum: i32 = (0..4)
    .map(|x| x * 2)
    .sum();
```

---

#### コンパイラが生成するコード

```rust
let sum = 0 + 2 + 4 + 6;
// ループすら無い！
```

---

### 2. インライン化

#### あなたが書くコード

```rust
let result: Vec<i32> = vec![1, 2, 3]
    .iter()
    .map(|x| x * 2)
    .collect();
```

---

#### コンパイラが生成するコード（イメージ）

```rust
let mut result = Vec::with_capacity(3);
result.push(1 * 2);
result.push(2 * 2);
result.push(3 * 2);
```

---

### 3. 境界チェックの省略

```rust
// イテレータを使うと...
for x in vec.iter() {
    // 境界チェック不要！
}

// 手動でループを書くと...
for i in 0..vec.len() {
    // 毎回境界チェックが必要
    let x = vec[i];
}
```

---

## オーディオデコーダの例

### 高レベルなコード

```rust
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
        .zip(&buffer[i - 12..i])
        .map(|(&c, &s)| c * s as i64)
        .sum::<i64>() >> qlp_shift;

    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}
```

**読みやすい！**

---

### 低レベルなコード（コンパイラが生成）

```rust
// ループ展開
let mut prediction = 0;
prediction += coefficients[0] * buffer[i - 12];
prediction += coefficients[1] * buffer[i - 11];
prediction += coefficients[2] * buffer[i - 10];
prediction += coefficients[3] * buffer[i - 9];
prediction += coefficients[4] * buffer[i - 8];
prediction += coefficients[5] * buffer[i - 7];
prediction += coefficients[6] * buffer[i - 6];
prediction += coefficients[7] * buffer[i - 5];
prediction += coefficients[8] * buffer[i - 4];
prediction += coefficients[9] * buffer[i - 3];
prediction += coefficients[10] * buffer[i - 2];
prediction += coefficients[11] * buffer[i - 1];
prediction >>= qlp_shift;
```

**超高速！**

---

## 他の言語との比較

### Python（遅い）

```python
# リスト内包表記は遅い
result = [x * 2 for x in range(1000000)]

# 実行時にオーバーヘッドがある
# インタープリタ言語なので最適化が限定的
```

---

### JavaScript（遅い）

```javascript
// map は実行時にオーバーヘッドがある
const result = Array.from({length: 1000000}, (_, i) => i)
    .map(x => x * 2);

// JIT コンパイラが最適化するが、Rust ほどではない
```

---

### C++（速い）

```cpp
// イテレータを使っても速い
std::vector<int> v(1000000);
std::iota(v.begin(), v.end(), 0);

std::vector<int> result(v.size());
std::transform(v.begin(), v.end(), result.begin(),
    [](int x) { return x * 2; });

// Rust と同様にゼロコスト抽象化
```

---

### Rust（速い！）

```rust
// コンパイル時に最適化される
let result: Vec<i32> = (0..1_000_000)
    .map(|x| x * 2)
    .collect();

// 実行時のオーバーヘッドなし
```

---

## ゼロコスト抽象化の利点

### 1. 読みやすさ

```rust
// 読みやすい
let sum: i32 = numbers.iter()
    .filter(|x| *x > 0)
    .map(|x| x * 2)
    .sum();

// vs

// 読みにくい
let mut sum = 0;
for &num in &numbers {
    if num > 0 {
        sum += num * 2;
    }
}
```

---

### 2. 保守性

```rust
// 変更しやすい
let result: Vec<i32> = numbers.iter()
    .filter(|x| *x > 0)     // ← 条件を変更
    .map(|x| x * 2)         // ← 変換を変更
    .take(10)               // ← 簡単に追加
    .collect();
```

---

### 3. 安全性

```rust
// イテレータは境界チェックが不要
for x in vec.iter() {
    // 安全
}

// インデックスは境界チェックが必要
for i in 0..vec.len() {
    let x = vec[i];  // 境界チェック
}
```

---

## 実測例

### ケース1: 単純な合計

```rust
use std::time::Instant;

// forループ
let start = Instant::now();
let mut sum = 0;
for i in 0..10_000_000 {
    sum += i;
}
println!("for: {:?}", start.elapsed());

// イテレータ
let start = Instant::now();
let sum: i64 = (0..10_000_000).sum();
println!("iter: {:?}", start.elapsed());
```

**結果:** ほぼ同じ（イテレータが少し速い）

---

### ケース2: フィルタと変換

```rust
// forループ
let start = Instant::now();
let mut result = Vec::new();
for i in 0..1_000_000 {
    if i % 2 == 0 {
        result.push(i * 2);
    }
}
println!("for: {:?}", start.elapsed());

// イテレータ
let start = Instant::now();
let result: Vec<i32> = (0..1_000_000)
    .filter(|x| x % 2 == 0)
    .map(|x| x * 2)
    .collect();
println!("iter: {:?}", start.elapsed());
```

**結果:** ほぼ同じ

---

## ビャーネ・ストロヴストルップの定義

> **ゼロオーバーヘッド原則:**
>
> 1. 使用しないものには、支払わなくてよい
> 2. 使っているものに対して、コードをそれ以上うまく書くことはできない

---

## Rust での実現

```
Rust のゼロコスト抽象化:

1. 使用しないものには、支払わなくてよい
   → 使わない機能はコンパイルされない
   → バイナリサイズが小さい

2. 使っているものに対して、コードをそれ以上うまく書くことはできない
   → イテレータは手書きループと同じ速度
   → むしろ最適化されて速いことも
```

---

## minigrep の改善

### Before（forループ）

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

---

### After（イテレータ）

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

**結果:**
- 読みやすい
- 短い
- 速度は同じ

---

## まとめ

```
ゼロコスト抽象化:
✅ 高レベルで書いても速い
✅ 低レベルと同じ速度
✅ コンパイラが最適化
✅ 実行時のオーバーヘッドなし

ベンチマーク:
forループ:    19,620,300 ns/iter
イテレータ:    19,234,900 ns/iter
→ イテレータが少し速い！

最適化の例:
- ループ展開
- インライン化
- 境界チェックの省略
- レジスタ最適化

利点:
✅ 読みやすい
✅ 保守しやすい
✅ 安全
✅ 速い

結論:
イテレータとクロージャを恐れずに使おう！
読みやすくて、速度も落ちない。
これが Rust の魔法！
```

---

## Python との比較

### Python

```python
# 読みやすいが遅い
result = [x * 2 for x in range(1000000) if x % 2 == 0]

# 実行時にオーバーヘッドがある
```

---

### Rust

```rust
// 読みやすくて速い
let result: Vec<i32> = (0..1_000_000)
    .filter(|x| x % 2 == 0)
    .map(|x| x * 2)
    .collect();

// コンパイル時に最適化される
```

---

## 実践での推奨事項

```
イテレータを使うべき場合:
✅ 読みやすさが重要
✅ コレクションの操作
✅ 関数型スタイル

forループを使うべき場合:
✅ 複雑な制御フロー
✅ 早期リターン
✅ 複数のコレクションを同時に操作

基本方針:
まずイテレータで書く
→ 必要ならforループに戻す
→ パフォーマンスは気にしない（ほぼ同じだから）
```
