# 第10章 Q&A (3/3): ライフタイム

## Q1: ライフタイム注釈は生きる時間を伸ばすもの？

### A: いいえ、違います。ライフタイム注釈は参照の有効期間をコンパイラに**説明するだけ**で、実際の寿命は変わりません。

**誤解:**
```
ライフタイム = 生きる時間を伸ばす ❌
```

**正解:**
```
ライフタイム = 参照がどれくらい有効かをコンパイラに教える注釈 ✅
```

---

**例:**

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//        ^^^^
//        これは「説明書」
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("long");   // string1 の寿命
    let string2 = String::from("short");  // string2 の寿命

    let result = longest(string1.as_str(), string2.as_str());
    // 'a は string1 と string2 のうち短い方と同じ

    println!("{}", result);  // result も同じ寿命
}
```

**ポイント:**
- `'a` は string1 と string2 の寿命を**変えない**
- `'a` はコンパイラに「戻り値は引数と同じ期間有効」と**説明している**
- 実際の寿命は変数のスコープで決まる

---

**ライフタイム = タグ・目印**

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
//        ^^^^   ^^          ^^           ^^
//        'a というタグを定義
//              ↓           ↓            ↓
//           タグ'a      タグ'a       タグ'a
```

**意味:** 「全部同じ 'a タグ = 同じライフタイムグループ」

---

## Q2: なぜライフタイム注釈が必要？書かないとどうなる？

### A: コンパイラが参照の有効期間を判断できない場合に必要。書かないとコンパイルエラーになる。

**問題のあるコード:**

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```

**エラー:**
```
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:33
  |
1 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value,
          but the signature does not say whether it is borrowed from `x` or `y`
```

**コンパイラの困惑:**
```
「戻り値が x から来るのか y から来るのかわからない
→ どのライフタイムを使えばいいかわからない
→ 教えて！」
```

---

**解決: ライフタイム注釈**

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

**コンパイラの理解:**
```
「OK！戻り値は x と y のうち短い方と同じライフタイム
→ 呼び出し側をチェックできる
→ 安全性を保証できる」
```

---

**ライフタイム注釈がないと検出できない危険:**

```rust
fn main() {
    let string1 = String::from("long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }  // ← string2 が破棄される
    println!("{}", result);  // ← result が無効な参照を指すかも
}
```

**ライフタイム注釈があれば:**
```
error[E0597]: `string2` does not live long enough
```

**コンパイラがエラーを検出してくれる！**

---

## Q3: ライフタイム注釈は常に必要？

### A: いいえ、多くの場合は自動的に推論されます。ライフタイム省略規則により、よくあるパターンでは書かなくて済みます。

**ライフタイム省略規則（3つ）:**

---

### 規則1: 各引数に独自のライフタイム

```rust
fn foo(x: &str, y: &str)
// ↓ 自動的に
fn foo<'a, 'b>(x: &'a str, y: &'b str)
```

---

### 規則2: 引数が1つなら、戻り値も同じライフタイム

```rust
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
// ↓ 自動的に
fn first_word<'a>(s: &'a str) -> &'a str {
    s.split_whitespace().next().unwrap_or("")
}
```

**だから書かなくていい！**

---

### 規則3: メソッドで &self があれば、戻り値は &self と同じ

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
    //                          ^^^^^               ^^^^^      ^^^^
    //                          'a                  'b         'a（自動）
        println!("Attention: {}", announcement);
        self.part
    }
}
```

**だから書かなくていい！**

---

**書く必要がある場合:**

```
✅ 複数の参照を受け取って参照を返す関数
✅ 構造体が参照を持つ場合
✅ コンパイラが推論できない複雑な場合

❌ 引数が1つの関数（規則2）
❌ メソッドで &self から返す場合（規則3）
❌ 所有権を返す場合（参照ではない）
```

---

## Q4: 構造体が参照を持つ場合、なぜライフタイム注釈が必要？

### A: 構造体のインスタンスが、参照しているデータより長生きしないことを保証するため。

**ライフタイムなしの構造体（❌ エラー）:**

```rust
struct ImportantExcerpt {
    part: &str,  // ❌ この参照はどれくらい有効？
}
```

**エラー:**
```
error[E0106]: missing lifetime specifier
```

---

**ライフタイムありの構造体（✅ OK）:**

```rust
struct ImportantExcerpt<'a> {
//                     ^^^^
//                     ライフタイム 'a を定義
    part: &'a str,
//        ^^^^^^^
//        'a タグ付き参照
}
```

**意味:** 「ImportantExcerpt のインスタンスは、part が参照しているデータより長生きできない」

---

**正常なケース:**

```rust
fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    //  ^^^^^
    //  元のデータ

    let first_sentence = novel.split('.').next().unwrap();
    //  ^^^^^^^^^^^^^^
    //  novel への参照

    let i = ImportantExcerpt {
        part: first_sentence,
    };
    //  ^
    //  インスタンス

    println!("{}", i.part);  // ✅ OK
}
```

**ライフタイム:**
```
novel ━━━━━━━━━━━━━━━━━━━━━━━━━┓
    ↑                            ┃
first_sentence ━━━━━━━━━━━━━━━━┫ ← 'a
    ↑                            ┃
i (ImportantExcerpt) ━━━━━━━━━━┫
                                 ┃
println!(...) ← OK              ┃
                                 ┃
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

---

**エラーケース:**

```rust
fn main() {
    let i;
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().unwrap();

        i = ImportantExcerpt {
            part: first_sentence,
        };
    }  // ← novel が破棄される

    println!("{}", i.part);  // ❌ エラー
}
```

**エラー:**
```
error[E0597]: `novel` does not live long enough
```

**ライフタイム注釈のおかげでコンパイラが検出！**

---

## Q5: ローカル変数への参照は返せない？どうすればいい？

### A: 返せません。ローカル変数はスコープを抜けると破棄されるため、ダングリングポインタになります。所有権を返すか、静的ライフタイムを使います。

**❌ ダメな例:**

```rust
fn dangle<'a>() -> &'a str {
    let s = String::from("hello");
    &s  // ❌ エラー！s はスコープを抜ける
}
```

**エラー:**
```
error[E0515]: cannot return reference to local variable `s`
```

---

**なぜダメ？**

```
関数実行中:
┌─────────────────────┐
│ dangle 関数のスタック │
│                     │
│ s: String           │
│   └→ ヒープ         │
│                     │
│ &s を返そうとする    │
└─────────────────────┘

関数終了後:
┌─────────────────────┐
│ dangle 関数のスタック │
│                     │
│ ❌ s は破棄         │
│ ❌ ヒープも解放     │
│                     │
│ 返した &s は無効    │  ← ダングリングポインタ
└─────────────────────┘
```

---

**✅ 解決方法1: 所有権を返す（推奨）**

```rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // ✅ OK（所有権を返す）
}

fn main() {
    let result = no_dangle();
    println!("{}", result);  // ✅ OK
}
```

---

**✅ 解決方法2: 静的ライフタイム**

```rust
fn static_str() -> &'static str {
    "hello"  // ✅ OK（文字列リテラルは 'static）
}

fn main() {
    let result = static_str();
    println!("{}", result);  // ✅ OK
}
```

**'static とは:** プログラムの実行中ずっと有効なライフタイム

---

**✅ 解決方法3: 引数を返す**

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
    s.split_whitespace().next().unwrap_or("")
    // ✅ OK（引数を返す）
}

fn main() {
    let sentence = String::from("hello world");
    let result = first_word(&sentence);
    println!("{}", result);  // ✅ OK
}
```

---

## まとめ

### ライフタイムの重要ポイント

```
✅ ライフタイム注釈は説明書（寿命は変わらない）
✅ コンパイラに参照の有効期間を伝える
✅ ダングリングポインタを防ぐ
✅ 多くの場合は自動推論される
✅ 構造体が参照を持つ場合は必須
✅ ローカル変数への参照は返せない
```

---

### いつ必要？

```
✅ 複数の参照を受け取って参照を返す関数
✅ 構造体が参照を持つ場合
✅ コンパイラが推論できない場合

❌ 引数が1つの関数（自動推論）
❌ メソッドで &self から返す場合（自動推論）
❌ 所有権を返す場合（参照ではない）
```

---

### 覚えておくこと

```
✅ ライフタイム = タグ・目印
✅ 'a は任意の名前（慣習的に 'a, 'b, ... を使う）
✅ 'static = プログラム全体で有効
✅ コンパイラがエラーで教えてくれる
✅ ローカル変数への参照は返せない → 所有権を返す
```
