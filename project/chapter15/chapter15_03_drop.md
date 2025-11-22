# Chapter 15-3: Drop トレイト - クリーンアップ処理

## Drop トレイトとは？

**値がスコープを出る時の処理をカスタマイズするトレイト**

```rust
{
    let x = CustomSmartPointer::new();
}  // ← ここで drop メソッドが自動実行される
```

---

## Drop トレイトの定義

```rust
pub trait Drop {
    fn drop(&mut self);
}
```

**重要:** スコープを出る時に**自動的に**呼ばれる

---

## Drop の実装例

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("CustomSmartPointerをドロップ: `{}`", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointerを生成しました。");
}
```

### 出力
```
CustomSmartPointerを生成しました。
CustomSmartPointerをドロップ: `other stuff`
CustomSmartPointerをドロップ: `my stuff`
```

**注目:** 変数は**逆順**でドロップされる（スタック順）

---

## Drop の用途

### 1. リソース解放
```rust
impl Drop for File {
    fn drop(&mut self) {
        // ファイルハンドルを閉じる
        self.close();
    }
}
```

### 2. ネットワーク接続のクローズ
```rust
impl Drop for Connection {
    fn drop(&mut self) {
        // 接続を切断
        self.disconnect();
    }
}
```

### 3. メモリ解放
```rust
impl<T> Drop for Box<T> {
    fn drop(&mut self) {
        // ヒープのメモリを解放
        unsafe { /* ... */ }
    }
}
```

---

## 早期ドロップ: std::mem::drop

### 問題：dropを直接呼べない
```rust
c.drop();  // ❌ コンパイルエラー！
// エラー: 明示的なデストラクタ呼び出しは許可されていません
```

### 解決：std::mem::drop を使う
```rust
let c = CustomSmartPointer {
    data: String::from("some data"),
};
println!("CustomSmartPointerを生成しました。");
drop(c);  // ✅ 早期にドロップ
println!("mainの終わりの前にCustomSmartPointerをドロップしました。");
```

---

## drop vs std::mem::drop

| 項目 | `drop` メソッド | `std::mem::drop` 関数 |
|------|----------------|---------------------|
| **自動実行** | ✅ スコープを出る時 | ❌ 手動で呼ぶ |
| **直接呼び出し** | ❌ 不可 | ✅ 可能 |
| **所有権** | - | 所有権を奪う |
| **用途** | 自動クリーンアップ | 早期解放 |

---

## std::mem::drop の仕組み

### 定義
```rust
pub fn drop<T>(_x: T) {
    // 何もしない
}
```

**動作:**
1. 所有権を受け取る
2. スコープを出る
3. 自動的に `drop` メソッドが呼ばれる

---

## Drop の順序

### スタック順（LIFO）
```rust
{
    let a = CustomSmartPointer { data: String::from("a") };
    let b = CustomSmartPointer { data: String::from("b") };
    let c = CustomSmartPointer { data: String::from("c") };
}
// ドロップ順: c → b → a
```

### 構造体のフィールド
```rust
struct Foo {
    x: CustomSmartPointer,
    y: CustomSmartPointer,
}
// ドロップ順: y → x（フィールドの逆順）
```

---

## Copy と Drop の関係

### 重要な制約
**Copy と Drop は同時に実装できない**

```rust
#[derive(Copy, Clone)]
struct MyCopy {
    x: i32,
}

// ❌ Copy と Drop は同時に実装不可
// impl Drop for MyCopy { ... }
```

**理由:** Copyは単純なビットコピー、Dropはカスタム処理が必要

---

## Python との対応

| 概念 | Rust | Python |
|------|------|--------|
| **デストラクタ** | `Drop::drop` | `__del__` |
| **自動実行** | スコープを出る時 | GCのタイミング |
| **早期解放** | `std::mem::drop` | `del` |

**重要:** Rustは決定的、Pythonは非決定的

---

## まとめ

| 項目 | 説明 |
|------|------|
| **Drop トレイト** | スコープを出る時の処理 |
| **自動実行** | コンパイラが自動挿入 |
| **用途** | リソース解放、クリーンアップ |
| **早期ドロップ** | `std::mem::drop` 関数 |
| **順序** | 変数はスタック順（LIFO） |
| **制約** | Copy と同時実装不可 |

**重要:** RAII（Resource Acquisition Is Initialization）パターンの基盤！
