# 第12章 Q&A (3/3): 機能実装編

## Q1: to_lowercase() は新しいメモリを作る？U を u に変えるのにメモリが必要？

### A: はい。Rust の文字列は不変なので、新しい String を作る必要がある

---

### なぜメモリが必要？

```rust
let original = "rUsT";  // メモリA: ['r', 'U', 's', 'T']
```

**問題:**
- Rust の文字列は**不変（immutable）**
- `'U'` を `'u'` に、`'T'` を `'t'` に**直接変更できない**

**解決:**
- 新しいメモリを確保
- 小文字に変換した文字列を作る

---

### to_lowercase() の動作

```rust
let original = "rUsT";           // メモリA: ['r', 'U', 's', 'T']
let lower = original.to_lowercase();  // メモリB: ['r', 'u', 's', 't']
//                                    ↑ 新しい String を作成
```

**流れ:**
1. 元の `"rUsT"` はそのまま（変更されない）
2. 新しいメモリ領域を確保
3. 小文字に変換した `"rust"` を書き込む
4. 新しい `String` を返す

---

### メモリのイメージ

```
元の文字列（変更不可）:
メモリ 0x1000:
┌───┬───┬───┬───┐
│ r │ U │ s │ T │  ← "rUsT" (元のデータ)
└───┴───┴───┴───┘
変更不可！

to_lowercase() で新しいメモリ確保:
メモリ 0x2000:
┌───┬───┬───┬───┐
│ r │ u │ s │ t │  ← "rust" (新しく作成)
└───┴───┴───┴───┘
```

---

### 直接変更できない理由

```rust
let s = "Hello";
// s[0] = 'h';  // ❌ コンパイルエラー！
// 文字列は不変
```

**Rust の設計思想:**
1. **安全性**: 複数箇所から参照されている時の競合を防ぐ
2. **効率性**: 不変なら複数箇所で共有できる
3. **予測可能性**: 値が勝手に変わらない

---

### minigrep での使用例

```rust
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();  // String型（新しいメモリ）
    //  ^^^^^
    //  もう &str ではなく String

    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            //  ^^^^^^^^^^^^^^^^
            //  毎回新しい String を作る
            results.push(line);
        }
    }

    results
}
```

---

### なぜ & が必要？

```rust
line.to_lowercase().contains(&query)
//                          ^^^^^^
```

**理由:**
- `query` は `String` 型
- `contains()` は `&str` を期待
- `&` で参照を渡す（自動的に `&str` に変換）

---

### 型の変化

```rust
// 引数として受け取る
query: &str

// to_lowercase() で変換
let query = query.to_lowercase();  // String型

// 使用時
line.to_lowercase()    // String型
    .contains(&query)  // &String → &str に自動変換
```

---

### 実際の例

```rust
fn main() {
    let original = "RuSt";
    println!("元: {}", original);  // RuSt

    let lower = original.to_lowercase();
    println!("小文字: {}", lower);      // rust
    println!("元: {}", original);      // RuSt ← 変わっていない！

    // lower は String型
    // original は &str型
}
```

---

### Python との比較

**Python:**
```python
# Python も文字列は不変
original = "RuSt"
lower = original.lower()  # 新しい文字列を作成
print(original)  # "RuSt" ← 変わらない
print(lower)     # "rust" ← 新しい文字列
```

**Rust:**
```rust
let original = "RuSt";
let lower = original.to_lowercase();  // 新しい String
println!("{}", original);  // "RuSt"
println!("{}", lower);     // "rust"
```

---

### まとめ

```
質問: U を u にするのに新しいメモリが必要？
答え: はい！

理由:
✅ 文字列は不変（変更できない）
✅ "rUsT" の 'U' を直接 'u' に変更できない
✅ 新しく "rust" という String を作る
✅ 新しいメモリ領域を確保する

to_lowercase():
1. 元の文字列はそのまま
2. 新しいメモリを確保
3. 小文字に変換
4. String型として返す

使用時の注意:
✅ query は String型になる
✅ contains() には &query を渡す
✅ &String は自動的に &str に変換される
```

---

## Q2: if 式で値を返せる？results = if ... ってどういうこと？

### A: Rust の if は「式」なので、値を返せる

---

### Rust の if は「式」

```rust
let results = if config.case_sensitive {
    search(&config.query, &contents)           // この値を返す
} else {
    search_case_insensitive(&config.query, &contents)  // または、この値を返す
};
```

**ポイント:**
- `if` は**式（expression）**
- 式は**値を返す**
- その値を変数に代入できる

---

### 基本例

```rust
let x = 5;
let result = if x > 3 {
    10  // ← x > 3 なら 10
} else {
    20  // ← x <= 3 なら 20
};
// result = 10
```

---

### 文字列の場合

```rust
let is_cold = true;
let message = if is_cold {
    "寒いです"
} else {
    "暑いです"
};
// message = "寒いです"
```

---

### minigrep の場合

```rust
let results = if config.case_sensitive {
    search(&config.query, &contents)  // Vec<&str> を返す
} else {
    search_case_insensitive(&config.query, &contents)  // Vec<&str> を返す
};
// results: Vec<&str>
```

**動作:**
- `config.case_sensitive` が `true` → `search()` の結果
- `config.case_sensitive` が `false` → `search_case_insensitive()` の結果

---

### 同じ型を返す必要がある

#### ✅ OK: 両方とも Vec&lt;&str&gt;

```rust
let results = if config.case_sensitive {
    search(&config.query, &contents)           // Vec<&str>
} else {
    search_case_insensitive(&config.query, &contents)  // Vec<&str>
};
```

---

#### ❌ エラー: 型が違う

```rust
let x = if true {
    10        // i32型
} else {
    "hello"   // &str型  ← 型が違う！
};
// コンパイルエラー
```

---

### match 式で書いた場合

#### if 式

```rust
let results = if config.case_sensitive {
    search(&config.query, &contents)
} else {
    search_case_insensitive(&config.query, &contents)
};
```

---

#### match 式（同じ意味）

```rust
let results = match config.case_sensitive {
    true => search(&config.query, &contents),
    false => search_case_insensitive(&config.query, &contents),
};
```

---

### 従来の書き方との比較

#### if 式を使わない場合

```rust
let results;

if config.case_sensitive {
    results = search(&config.query, &contents);
} else {
    results = search_case_insensitive(&config.query, &contents);
}
```

**デメリット:**
- `results` を先に宣言
- 冗長

---

#### if 式を使う場合（推奨）

```rust
let results = if config.case_sensitive {
    search(&config.query, &contents)
} else {
    search_case_insensitive(&config.query, &contents)
};
```

**メリット:**
- 簡潔
- `results` は常に初期化される（安全）

---

### 実際の動作例

#### ケース1: case_sensitive = true

```rust
let config = Config {
    case_sensitive: true,  // ← true
    ...
};

let results = if config.case_sensitive {  // true なので
    search(&config.query, &contents)  // ← こちらが実行される
} else {
    search_case_insensitive(&config.query, &contents)
};
```

---

#### ケース2: case_sensitive = false

```rust
let config = Config {
    case_sensitive: false,  // ← false
    ...
};

let results = if config.case_sensitive {  // false なので
    search(&config.query, &contents)
} else {
    search_case_insensitive(&config.query, &contents)  // ← こちらが実行される
};
```

---

### Python との比較

**Python の三項演算子:**
```python
results = (
    search(query, contents) if case_sensitive
    else search_case_insensitive(query, contents)
)
```

**Rust の if 式:**
```rust
let results = if case_sensitive {
    search(&query, &contents)
} else {
    search_case_insensitive(&query, &contents)
};
```

---

### まとめ

```
if 式:
✅ Rust の if は「式」（値を返す）
✅ 変数に代入できる
✅ 両方の分岐は同じ型を返す必要がある

使い方:
let x = if 条件 { 値A } else { 値B };

minigrep:
let results = if config.case_sensitive {
    search(...)           // 大文字小文字を区別
} else {
    search_case_insensitive(...)  // 区別しない
};
```

---

## Q3: macOS で環境変数を設定するには？PowerShell のコマンドがエラーになる

### A: macOS/Linux は zsh/bash なので、コマンドが違う

---

### エラーの原因

```bash
$ $env:CASE_INSENSITIVE=1
zsh: command not found: :CASE_INSENSITIVE=1
```

**原因:**
- `$env:CASE_INSENSITIVE=1` は **PowerShell**（Windows）のコマンド
- macOS は **zsh** または **bash**
- コマンドの書き方が違う

---

## macOS/Linux での環境変数設定

### 方法1: export コマンド（セッション全体）

```bash
export CASE_INSENSITIVE=1
cargo run -- to poem.txt
```

**効果:**
- 現在のターミナルセッション全体で有効
- 別のコマンドでも使える

---

### 方法2: コマンド実行時に設定（推奨）

```bash
CASE_INSENSITIVE=1 cargo run -- to poem.txt
```

**効果:**
- そのコマンドだけで有効
- 他のコマンドには影響しない

---

## 実際の使い方

### 大文字小文字を区別しない検索

```bash
# 方法1: export
export CASE_INSENSITIVE=1
cargo run -- to poem.txt

# 方法2: 1行で（推奨）
CASE_INSENSITIVE=1 cargo run -- to poem.txt
```

**結果:** "to", "To", "TO" すべてマッチ

---

### 大文字小文字を区別する検索（デフォルト）

```bash
# 環境変数を設定しない
cargo run -- to poem.txt

# または環境変数を削除
unset CASE_INSENSITIVE
cargo run -- to poem.txt
```

**結果:** "to" だけマッチ

---

## 環境変数の確認と削除

### 確認

```bash
echo $CASE_INSENSITIVE
# 何も表示されない → 設定されていない
# 1 → 設定されている
```

---

### すべての環境変数を表示

```bash
env | grep CASE_INSENSITIVE
```

---

### 削除

```bash
unset CASE_INSENSITIVE
```

---

## lib.rs の動作

```rust
let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
```

**動作:**
- `CASE_INSENSITIVE` が**設定されていない** → `is_err()` = `true` → `case_sensitive = true`（区別する）
- `CASE_INSENSITIVE` が**設定されている** → `is_err()` = `false` → `case_sensitive = false`（区別しない）

**重要:** 値は何でもOK（`1` でも `true` でも `foo` でも同じ）

---

## 実例

### ケース1: 環境変数なし

```bash
cargo run -- to poem.txt
```

**内部:**
```rust
env::var("CASE_INSENSITIVE")  // Err（設定されていない）
.is_err()                     // true
// case_sensitive = true → search() を使う
```

---

### ケース2: 環境変数あり

```bash
CASE_INSENSITIVE=1 cargo run -- to poem.txt
```

**内部:**
```rust
env::var("CASE_INSENSITIVE")  // Ok("1")
.is_err()                     // false
// case_sensitive = false → search_case_insensitive() を使う
```

---

## OS別のコマンド

### macOS/Linux (zsh/bash)

```bash
# セッション全体
export CASE_INSENSITIVE=1

# 1コマンドだけ（推奨）
CASE_INSENSITIVE=1 cargo run -- to poem.txt
```

---

### Windows PowerShell

```powershell
$env:CASE_INSENSITIVE=1
cargo run -- to poem.txt
```

---

### Windows cmd

```cmd
set CASE_INSENSITIVE=1
cargo run -- to poem.txt
```

---

## よくある間違い

### ❌ macOS で PowerShell のコマンド

```bash
$ $env:CASE_INSENSITIVE=1
zsh: command not found: :CASE_INSENSITIVE=1
```

---

### ✅ macOS での正しいコマンド

```bash
$ CASE_INSENSITIVE=1 cargo run -- to poem.txt
```

---

## まとめ

```
macOS/Linux:
✅ CASE_INSENSITIVE=1 cargo run ...  （推奨）
✅ export CASE_INSENSITIVE=1
   cargo run ...

Windows PowerShell:
✅ $env:CASE_INSENSITIVE=1
   cargo run ...

確認と削除:
✅ echo $CASE_INSENSITIVE  （確認）
✅ unset CASE_INSENSITIVE  （削除）

動作:
✅ 設定なし → 大文字小文字を区別
✅ 設定あり → 大文字小文字を区別しない
```

---

### 試してみる

```bash
# 1. デフォルト（区別する）
cargo run -- to poem.txt

# 2. 区別しない
CASE_INSENSITIVE=1 cargo run -- to poem.txt

# 3. 環境変数を確認
echo $CASE_INSENSITIVE

# 4. 削除
unset CASE_INSENSITIVE
```
