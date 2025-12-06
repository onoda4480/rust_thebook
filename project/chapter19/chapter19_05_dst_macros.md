# 第19章 Part 5: 動的サイズ決定型（DST）と手続き的マクロ

## 動的サイズ決定型（Dynamically Sized Types, DST）

### 問題: str はサイズ不明

```rust
let s1: str = "Hello";  // ❌ エラー！
// error: size for values of type `str` cannot be known at compilation time
```

**理由**: `str` のサイズはコンパイル時に決定できない
- `"Hello"` は 5 バイト
- `"Hello, world!"` は 13 バイト
- 文字列ごとにサイズが異なる

### 解決策: 参照を使う

```rust
let s1: &str = "Hello";  // ✅ OK！
```

**理由**: `&str` は固定サイズ（16バイト）
- ポインタ: 8バイト（メモリアドレス）
- 長さ: 8バイト（文字列の長さ）

### DSTの種類

Rustには3種類のDST（動的サイズ決定型）がある:

1. **str** - 文字列スライス
2. **[T]** - スライス
3. **dyn Trait** - トレイトオブジェクト

すべて**参照を通してのみ使用可能**:

```rust
// ❌ 直接は使えない
let s: str = ...;
let arr: [i32] = ...;
let obj: dyn Display = ...;

// ✅ 参照なら使える
let s: &str = ...;
let arr: &[i32] = ...;
let obj: &dyn Display = ...;
let obj: Box<dyn Display> = ...;
```

---

## Sized トレイト

### すべてのジェネリクスは暗黙的に Sized

```rust
fn generic<T>(t: T) {
    // ...
}

// 実際にはこう書かれている:
fn generic<T: Sized>(t: T) {
    // ...
}
```

Rustは自動的に `T: Sized` という境界を追加する。

### Sized の意味

```rust
fn generic<T: Sized>(t: T)
```

- `T` は**コンパイル時にサイズが分かる型**でなければならない
- `i32`, `String`, `Vec<T>` などはOK
- `str`, `[i32]`, `dyn Trait` はNG（サイズ不明）

---

## ?Sized - サイズ不明の型を許可

### 問題: 通常のジェネリクスでは str が使えない

```rust
fn generic<T: Sized>(t: T) {
    // ...
}

// ❌ str はサイズ不明なので使えない
generic::<str>("hello");
```

### 解決策: ?Sized を使う

```rust
fn generic<T: ?Sized>(t: &T) {
    //         ^^^^^^  ^^
    //         サイズ不明でもOK
    //                  参照にする必要がある
    println!("{:?}", std::mem::size_of_val(t));
}

fn main() {
    generic("hello");        // ✅ &str が使える
    generic(&[1, 2, 3]);     // ✅ &[i32] が使える
    generic(&42);            // ✅ &i32 も使える（Sized な型も OK）
}
```

### ?Sized の意味

```rust
fn generic<T: ?Sized>(t: &T)
```

- `?Sized` = "Maybe Sized"（サイズが分かるかもしれないし、分からないかもしれない）
- `T` はサイズ不明の型でもOK
- **重要**: サイズ不明なので、`t: T` ではなく `t: &T` にする必要がある

---

## DSTの実例

### 1. str と &str

```rust
// str: サイズ不明
let s: str = "hello";  // ❌

// &str: 16バイト固定（ポインタ + 長さ）
let s: &str = "hello";  // ✅
println!("Size of &str: {}", std::mem::size_of::<&str>());  // 16
```

### 2. [T] と &[T]

```rust
// [i32]: サイズ不明（要素数が不明）
let arr: [i32] = [1, 2, 3];  // ❌

// &[i32]: 16バイト固定（ポインタ + 長さ）
let arr: &[i32] = &[1, 2, 3];  // ✅
println!("Size of &[i32]: {}", std::mem::size_of::<&[i32]>());  // 16
```

### 3. dyn Trait と &dyn Trait

```rust
use std::fmt::Display;

// dyn Display: サイズ不明
let obj: dyn Display = 42;  // ❌

// &dyn Display: 16バイト固定（データポインタ + vtableポインタ）
let obj: &dyn Display = &42;  // ✅
println!("Size of &dyn Display: {}", std::mem::size_of::<&dyn Display>());  // 16
```

---

## 手続き的マクロ（Procedural Macros）

### 手続き的マクロとは？

- コードを入力として受け取り、コードを生成する
- コンパイル時に実行される
- 3種類: derive マクロ、attribute マクロ、function-like マクロ

### Derive マクロの実装

#### プロジェクト構成

```
macro_pord/
├── Cargo.toml
├── src/
│   └── main.rs
└── hello_macro/
    ├── Cargo.toml
    ├── src/
    │   └── lib.rs
    └── hello_macro_derive/
        ├── Cargo.toml
        └── src/
            └── lib.rs
```

3つのクレートが必要:
1. **macro_pord**: マクロを使うクレート
2. **hello_macro**: トレイト定義 + マクロの再エクスポート
3. **hello_macro_derive**: マクロの実装

---

#### 1. main.rs（マクロを使う）

```rust
use hello_macro::HelloMacro;

#[derive(HelloMacro)]  // ← このマクロで自動実装される
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
    // 出力: Hello, Macro! My name is Pancakes!
}
```

#### 2. hello_macro/src/lib.rs（トレイト定義）

```rust
pub trait HelloMacro {
    fn hello_macro();
}

// 手続き的マクロを再エクスポート
pub use hello_macro_derive::HelloMacro;
```

#### 3. hello_macro/Cargo.toml

```toml
[package]
name = "hello_macro"
version = "0.1.0"
edition = "2021"

[dependencies]
hello_macro_derive = { path = "./hello_macro_derive" }
```

#### 4. hello_macro_derive/src/lib.rs（マクロ実装）

```rust
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 操作可能な構文木としてのRustコードの表現を構築する
    let ast = syn::parse(input).unwrap();

    // トレイトの実装内容を構築
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;  // 構造体名を取得（例: Pancakes）

    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };

    gen.into()
}
```

#### 5. hello_macro_derive/Cargo.toml

```toml
[package]
name = "hello_macro_derive"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true  # ← これが重要！

[dependencies]
syn = "1.0"      # Rustコードをパースする
quote = "1.0"    # Rustコードを生成する
```

---

### マクロの処理フロー

1. **入力**: `#[derive(HelloMacro)]` がついた構造体
   ```rust
   struct Pancakes;
   ```

2. **パース**: `syn::parse()` で構文木に変換
   ```rust
   let ast = syn::parse(input).unwrap();
   // ast.ident = "Pancakes"
   ```

3. **コード生成**: `quote!` マクロでコードを生成
   ```rust
   quote! {
       impl HelloMacro for Pancakes {
           fn hello_macro() {
               println!("Hello, Macro! My name is Pancakes!");
           }
       }
   }
   ```

4. **出力**: 生成されたコードをコンパイラに返す

---

### 重要な依存クレート

#### syn クレート
- Rustコードを構文木にパースする
- `DeriveInput` 型で構造体の情報を取得

```rust
let ast: DeriveInput = syn::parse(input).unwrap();
let struct_name = &ast.ident;  // 構造体名
```

#### quote クレート
- Rustコードを生成する
- `#変数名` でテンプレート変数を埋め込める

```rust
let name = "Pancakes";
let code = quote! {
    println!("Hello, {}!", stringify!(#name));
};
```

---

## まとめ

### DST（動的サイズ決定型）
- **種類**: `str`, `[T]`, `dyn Trait`
- **特徴**: コンパイル時にサイズが不明
- **使用法**: 参照を通してのみ使用可能（`&str`, `&[T]`, `&dyn Trait`）
- **Sized トレイト**: ジェネリクスに自動的に付与される
- **?Sized**: サイズ不明の型も許可する

### 手続き的マクロ
- **3つのクレート構成**: 使用側、トレイト定義、マクロ実装
- **proc-macro = true**: マクロクレートに必須
- **syn**: コードのパース
- **quote**: コードの生成
- **動作**: コンパイル時にコードを自動生成
