# Chapter 4: Q&A まとめ

第4章で出た質問と回答のまとめ

---

## Q1: 整数のコピーが特別な理由

**質問:** 整数はスタックに保存されるから自動的にコピーされるってこと？

**回答:** その通り！

### 整数型の特徴
- **固定長**: たった数バイト（例: i32は4バイト）
- **スタック**: 全データがスタックに収まる
- **高速**: コピーが超高速
- **Copy トレイト**: 自動的にコピーされる

```rust
let x = 5;
let y = x;  // 自動的にコピー

println!("{} {}", x, y);  // 両方使える！
```

### String との違い
```rust
let s1 = String::from("hello");
let s2 = s1;  // ムーブ（コピーではない）

// println!("{}", s1);  // エラー！
```

**理由:** String はヒープを使うので、ムーブが発生する

---

## Q2: Copy 型の判定基準

**質問:** 大きなデータと小さいデータの線引きはある？

**回答:** 線引きは**サイズではなく「ヒープを使うか」**

### Copy 型の条件

✅ **スタックだけで完結する型**
- 全データがスタック上にある
- メモリ確保が不要

❌ **ヒープを使う型**
- ヒープにデータを持つ
- メモリ確保が必要

### 実例

```rust
// 128バイトの配列もCopy可能！（スタックだけだから）
let big_array: [i32; 32] = [0; 32];  // 128バイト
let copy = big_array;  // コピーされる！

// でも5バイトのStringはCopyできない！（ヒープを使うから）
let small_string = String::from("hello");  // 実質5バイト
let moved = small_string;  // コピーではなくムーブ！
```

### Copy 型の一覧

| 型 | Copy? | 理由 |
|---|---|---|
| `i32`, `u64` など | ✅ | スカラー値 |
| `bool` | ✅ | スカラー値 |
| `f64` | ✅ | スカラー値 |
| `char` | ✅ | 固定4バイト |
| `(i32, bool)` | ✅ | 中身が全てCopy |
| `[i32; 100]` | ✅ | スタックに収まる |
| `String` | ❌ | ヒープ使用 |
| `Vec<T>` | ❌ | ヒープ使用 |

---

## Q3: char が Copy な理由

**質問:** char は固定長だから copy?

**回答:** その通り！

### char の特徴
```rust
let c1: char = 'a';      // 4バイト
let c2: char = 'あ';     // 4バイト
let c3: char = '🦀';     // 4バイト（絵文字も！）

// 全て同じサイズ = 固定長 = Copy ✅
```

### String との違い
```rust
// char: 固定4バイト
let c = 'あ';
let d = c;  // コピー
println!("{} {}", c, d);  // OK!

// String: 可変長
let s1 = String::from("あ");
let s2 = s1;  // ムーブ
// println!("{}", s1);  // エラー！
```

**まとめ:** 固定長 = コンパイル時にサイズが分かる = スタック = Copy 可能

---

## Q4: スコープと変数の有効期間

**質問:** x はそれ以降使われていないからスコープが終了していますか？

**回答:** いいえ、違います！

### スコープの定義

**スコープは「使われているかどうか」ではなく、「どこまで生きているか」で決まる**

```rust
fn main() {
    let x = 5;
    println!("{}", x);  // 最後の使用

    // ここから先、xは使われていないが...
    // xはまだ「生きている」（スコープ内）

} // ← ここでxのスコープ終了
```

### ブロック `{}` がスコープを決める

```rust
fn main() {  // ← ブロック開始
    let x = 5;
    // xが使われようが使われまいが、
    // ここまでxは「生きている」
} // ← ブロック終了 = xのスコープ終了
```

### Rust の最適化

実は、Rustコンパイラは賢くて、最後の使用位置を知っています。でも**概念的には `}` までスコープは続いています**。

---

## Q5: ムーブ済み変数の drop

**質問:** ムーブ済みなら drop しないと Rust が判断してくれるのですか？

**回答:** その通り！Rust コンパイラが自動的に判断します。

### コンパイラの追跡

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // ← コンパイラ: "s1はムーブされた"と記録

    // コンパイラは知っている:
    // - s1: 無効（ムーブ済み）
    // - s2: 有効

} // スコープ終了
  // コンパイラ: "s1はムーブ済み → dropしない"
  // コンパイラ: "s2は有効 → dropする"
```

### コンパイル時に全て解決

```rust
// ムーブ後に使おうとすると...
let s1 = String::from("hello");
let s2 = s1;

println!("{}", s1);  // ❌ コンパイルエラー！
```

**エラー:**
```
error[E0382]: borrow of moved value: `s1`
```

コンパイラが「s1はムーブ済みだよ！」と教えてくれる。

### まとめ

| 処理 | 担当 | タイミング |
|---|---|---|
| 所有権の追跡 | Rustコンパイラ | コンパイル時 |
| ムーブの検出 | Rustコンパイラ | コンパイル時 |
| drop の判断 | Rustコンパイラ | コンパイル時 |
| drop 実行 | 生成されたコード | 実行時 |

---

## Q6: 参照と drop の責任

**質問:** 参照が指しているものをドロップすることはありませんとはどのようなことですか？

**回答:** 借りるだけで drop の責任がないってこと

### 図書館の例え

```rust
fn main() {
    let s = String::from("本");  // 図書館（所有者）
    let s2 = &s;                  // 借りている人（借用者）

    println!("{}", s2);  // 借りて使うだけ

} // s2のスコープ終了 → 何もしない（借りてるだけだから）
  // sのスコープ終了 → drop実行（所有者の責任！）
```

### 所有者 vs 借用者

| 役割 | 所有権 | drop の責任 |
|---|---|---|
| **所有者** | ✅ 持っている | ✅ 自分がdropする |
| **借用者（参照）** | ❌ 持っていない | ❌ dropしない |

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
} // ← sは参照、dropしない
  // 実際のStringをdropするのは「所有者」の仕事
```

---

## Q7: 可変参照の条件

**質問:** これは参照もとが不変だからだめってこと？

**回答:** その通り！元の変数も参照も両方可変である必要がある。

### NG な例

```rust
let s = String::from("hello");  // ← mut がない（不変）
change(&s);  // ← &s（不変参照）

fn change(some_string: &String) {  // ← &String（不変参照）
    some_string.push_str(", world");  // ❌ 変更できない！
}
```

### OK な例

```rust
let mut s = String::from("hello");  // ① mut 必須

change(&mut s);  // ② &mut 必須

fn change(some_string: &mut String) {  // ③ &mut 必須
    some_string.push_str(", world");  // ✅ 変更できる！
}
```

**必要な3つの条件:**
1. ✅ 元の変数が可変（`let mut s`）
2. ✅ 可変参照で渡す（`&mut s`）
3. ✅ 関数も可変参照を受け取る（`&mut String`）

---

## Q8: 可変参照の制限

**質問:** 特定のスコープで、ある特定のデータに対しては、一つしか可変な参照を持てないからエラー？

**回答:** その通り！完璧な理解です！

### NG な例

```rust
let mut s = String::from("hello");

let r1 = &mut s;  // 1つ目の可変参照
let r2 = &mut s;  // 2つ目の可変参照 ❌ エラー！

println!("{}, {}", r1, r2);
```

**エラー:**
```
error[E0499]: cannot borrow `s` as mutable more than once at a time
```

### なぜこのルールがあるのか？

データ競合（data race）を防ぐため。

```rust
// もし複数の可変参照が許されたら...
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;

r1.push_str(" world");  // r1で変更
r2.push_str(" Rust");   // r2で変更
// どっちが先？結果は？混乱する！
```

### 解決策：スコープを分ける

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;
    println!("{}", r1);
} // ← r1のスコープ終了

let r2 = &mut s;  // ✅ OK! r1はもう使われていない
println!("{}", r2);
```

---

## Q9: 不変参照と可変参照の混在

**質問:** 可変は一つしかないがなぜだめ？

**回答:** 可変参照は1つだけですが、**不変参照と同時に存在している**からダメです。

### NG な例

```rust
let mut s = String::from("hello");

let r1 = &s;      // 不変参照1
let r2 = &s;      // 不変参照2
let r3 = &mut s;  // 可変参照 ❌

println!("{}, {}, {}", r1, r2, r3);  // 全部同時に存在
```

**エラー:**
```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
```

### Rust の借用ルール

| 状況 | OK? | 理由 |
|---|---|---|
| 不変参照のみ（複数） | ✅ | 読み取りだけなら安全 |
| 可変参照のみ（1つ） | ✅ | 排他的アクセス |
| 可変参照（複数） | ❌ | データ競合 |
| **不変参照 + 可変参照** | **❌** | **読み取り中の変更は危険** |

### なぜ危険？

```rust
let r1 = &s;      // r1は "hello" を読んでいる
let r2 = &s;      // r2も "hello" を読んでいる
let r3 = &mut s;  // r3が値を変更しようとする

// もし許されたら...
r3.push_str(" world");  // r3が変更
println!("{}", r1);     // r1は何が表示される？
                        // "hello"? "hello world"?
                        // 予測不可能で危険！
```

---

## Q10: Non-Lexical Lifetimes

**質問:** Rust の新しいバージョンでは、コンパイル時に r1 は「もう使わない」と Rust が判断するってこと？

**回答:** その通り！

### 最後の使用を追跡

```rust
let mut s = String::from("hello");

let r1 = &s;
println!("{}", r1);  // ← r1の最後の使用
// コンパイラ: "r1はここで終わり"と判断

let r2 = &mut s;  // ✅ OK! r1はもう使われない
println!("{}", r2);
```

### コンパイラの判断

```
r1のライフタイム: let r1 ~ println!
r2のライフタイム: let r2 ~ println!

重複なし → OK!
```

### NG な例

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &mut s;  // エラー！

println!("{}", r1);  // ← r1をここで使う
println!("{}", r2);
```

**理由:** r1 は println! まで使われる → r2 と重複

---

## Q11: ダングリングポインタ

**質問:** dangle 内の変数 s がスコープを抜けたら drop するのに参照して返しているので「何もないものを貸している」ってこと？

**回答:** その通り！完璧な理解です！

### ダングリングポインタとは

**「すでに解放されたメモリを指しているポインタ」**

```rust
fn dangle() -> &String {  // ❌ コンパイルエラー
    let s = String::from("hello");
    &s  // sへの参照を返そうとする
} // ← sがdropされる → 参照が無効に！

fn main() {
    let reference_to_nothing = dangle();
    // reference_to_nothing は解放済みメモリを指している！
}
```

### 時系列

```
1. dangle() 内で s が誕生 → ヒープ["hello"]
2. &s を返そうとする
3. スコープ終了 → s が drop → ヒープ解放
4. main() で受け取る → 存在しないメモリを指す（危険！）
```

### Rust の解決策

```rust
fn no_dangle() -> String {  // ✅ OK
    let s = String::from("hello");
    s  // 所有権を返す
}  // sはムーブ済みなのでdropされない
```

---

## Q12: 文字列リテラルと参照

**質問:** なぜ `let word = first_word(&my_string);` は参照で `let word = first_word(my_literal);` はそのまま？

**回答:** 変数の型の違いです。

### 型の確認

```rust
// String型
let my_string = String::from("hello world");
// 型: String

// 文字列リテラル
let my_literal = "hello world";
// 型: &str（すでに参照！）
```

### なぜ違うのか

```rust
fn first_word(s: &str) -> &str {
    // ...
}

// String → & を付けて &String → 自動的に &str に変換
first_word(&my_string);

// &str → そのまま渡せる
first_word(my_literal);
```

### 型の流れ

```
String の場合:
String
  ↓ &を付ける
&String
  ↓ 自動変換
&str ← 関数が要求する型

文字列リテラルの場合:
&str ← すでにこの型
  ↓ そのまま
&str ← 関数が要求する型
```

---

## Q13: &str を引数に使う理由

**質問:** なぜ `&String` ではなく `&str` を使うのか？

**回答:** `&str` の方が柔軟だから。

### Before: `&String`（制限あり）

```rust
fn first_word(s: &String) -> &str {
    // ...
}

// Stringの参照しか渡せない
let my_string = String::from("hello");
first_word(&my_string);  // ✅ OK

let my_literal = "hello";
first_word(my_literal);  // ❌ エラー
```

### After: `&str`（柔軟）

```rust
fn first_word(s: &str) -> &str {
    // ...
}

// 全部OK！
first_word(&my_string);      // String
first_word(my_literal);      // リテラル
first_word(&my_string[..]);  // スライス
```

### なぜ自動変換されるのか？

**Deref coercion（参照外し型強制）**

```rust
&String → &str の変換は自動
&str → &String の変換はできない
```

---

## Q14: Python の list との比較

**質問:** Python でいう list みたいなもの？

**回答:** 似ているけど、ちょっと違います。

### Python の list = Rust の Vec

```python
# Python
a = [1, 2, 3]
a.append(4)
```

```rust
// Rust
let mut v = vec![1, 2, 3];
v.push(4);
```

### Python の list[1:3] ≠ Rust の &slice[1..3]

**Python（コピー）:**
```python
a = [1, 2, 3, 4, 5]
slice = a[1:3]  # [2, 3] の新しいリスト
slice[0] = 999  # OK
```

**Rust（参照）:**
```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];  // 参照（ポインタ）
// slice[0] = 999;  // ❌ エラー
```

### まとめ

```
Python list     ≈ Rust Vec<T>
Python list[:]  ≠ Rust &[T]（コピー vs 参照）
```

---

## Q15: ベクタとコピー

**質問:** スライスは参照、ベクタ（Python での list みたいなもの）はコピーってことですね

**回答:** 微妙に違います！正しくは：

### ✅ スライス = 参照

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];  // 参照

println!("{:?}", a);      // ✅ OK
println!("{:?}", slice);  // ✅ OK
```

### ❌ ベクタ = コピー（間違い）

### ⭕ ベクタ = ムーブ

```rust
let v = vec![1, 2, 3, 4, 5];
let v2 = v;  // ムーブ（コピーではない）

// println!("{:?}", v);  // ❌ エラー！
println!("{:?}", v2);  // ✅ OK
```

### 明示的にコピーする場合

```rust
let v = vec![1, 2, 3, 4, 5];
let v2 = v.clone();  // 明示的にコピー

println!("{:?}", v);   // ✅ OK
println!("{:?}", v2);  // ✅ OK
```

### 正しい理解

```
✅ スライス = 参照（ポインタ）
❌ ベクタ = コピー
✅ ベクタ = ムーブ（デフォルト）
✅ ベクタ = コピー（.clone() した場合のみ）
```

---

## 重要な概念のまとめ

### 所有権の基本
- 各値は1つの所有者を持つ
- スタック型はコピー、ヒープ型はムーブ
- スコープ終了時に自動drop

### 参照と借用
- 参照 = 借りるだけ、所有権を取らない
- 不変参照は複数OK、可変参照は1つだけ
- 不変と可変は同時に持てない

### スライス
- コレクションの一部への参照
- 文字列スライス（&str）と配列スライス（&[T]）
- Python のスライスとは違う（参照 vs コピー）
