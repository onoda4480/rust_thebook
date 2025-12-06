# Rust + DDD 学習ガイド

会社のコードが DDD（ドメイン駆動設計）で書かれている場合の学習パスをまとめたガイドです。

---

## 🎯 推奨する学習順序

### 結論: Rust を先に深めてから DDD を学ぶ

**理由**:
1. DDD のコードを読むには、まず **Rust のコード自体が読めることが前提**
2. axum + async/await が分からないと、DDD 以前に構文でつまずく
3. **2つの難しさを同時に学ぶのは非効率**
   - 言語の難しさ（Rust、async/await）
   - アーキテクチャの難しさ（DDD）

---

## 📅 6週間の学習スケジュール

### Week 1-2: Rust + axum の基礎固め

**目標**: シンプルな REST API を自分で作れるようになる

**やること**:
1. **The Async Book** を読む
   - URL: https://rust-lang.github.io/async-book/
   - async/await の理解

2. **tokio チュートリアル** を完了
   - URL: https://tokio.rs/tokio/tutorial
   - 非同期プログラミングの基礎

3. **axum examples** を写経（1日1-2個）
   - URL: https://github.com/tokio-rs/axum/tree/main/examples
   - 必須の examples:
     - `hello-world` - 基本
     - `rest-greet-with-query` - クエリパラメータ
     - `todos` - CRUD API
     - `error-handling-and-dependency-injection` - エラーハンドリング
     - `jwt` - 認証
     - `sqlx-postgres` - データベース

**平日**: axum examples を1-2個写経
**週末**: 簡単な CRUD API を自分で作る

---

### Week 3-4: DDD の概念を学ぶ + 実践

**目標**: DDD の基本概念を理解し、簡単なプロジェクトで実践

**やること**:
1. DDD の基本概念を学ぶ（下記の「DDD 基礎概念」を参照）
2. TODO アプリを DDD で設計して実装
3. Rust DDD の参考リポジトリを読む
   - https://github.com/reacherhq/backend
   - https://github.com/brooks-builds/full-stack-todo-rust-course

**平日**: DDD の概念を1日1概念学ぶ
**週末**: TODO アプリを DDD で設計・実装

---

### Week 5-6: 会社のコードを読む

**目標**: 会社のコードベースを理解する

**やること**:
1. 会社のコードを少しずつ読む
2. 分からない部分を個人プロジェクトで試す
3. チームメンバーに質問する

**平日**: 会社のコードを読む（1日30分-1時間）
**週末**: 分からなかった部分を整理、個人プロジェクトで実験

---

## 📚 Rust + axum の基礎

### 必須の学習リソース

1. **The Async Book** ⭐⭐⭐⭐⭐
   - https://rust-lang.github.io/async-book/
   - async/await の詳細

2. **tokio チュートリアル** ⭐⭐⭐⭐⭐
   - https://tokio.rs/tokio/tutorial
   - 非同期ランタイムの理解

3. **axum 公式ドキュメント** ⭐⭐⭐⭐⭐
   - https://docs.rs/axum/latest/axum/
   - すべての機能の詳細な説明

4. **axum examples** ⭐⭐⭐⭐⭐
   - https://github.com/tokio-rs/axum/tree/main/examples
   - 実践的なコード例

---

### axum の基本パターン

#### 最小構成の API

```rust
use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
}

// GET /users/:id
async fn get_user(
    Path(id): Path<u64>,
) -> Json<User> {
    Json(User {
        id,
        name: "Alice".to_string(),
    })
}

// POST /users
#[derive(Deserialize)]
struct CreateUser {
    name: String,
}

async fn create_user(
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    (
        StatusCode::CREATED,
        Json(User {
            id: 1,
            name: payload.name,
        }),
    )
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users/:id", get(get_user))
        .route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
```

---

#### 状態管理（State）

```rust
use std::sync::Arc;
use sqlx::PgPool;

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

async fn handler(
    State(state): State<Arc<AppState>>,
) -> String {
    // state.db を使ってデータベースにアクセス
    "OK".to_string()
}

#[tokio::main]
async fn main() {
    let pool = PgPool::connect("postgres://localhost/mydb")
        .await
        .unwrap();

    let state = Arc::new(AppState { db: pool });

    let app = Router::new()
        .route("/", get(handler))
        .with_state(state);

    // サーバー起動
}
```

---

#### エラーハンドリング

```rust
use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
};
use thiserror::Error;

#[derive(Error, Debug)]
enum ApiError {
    #[error("User not found")]
    UserNotFound,

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

// axum の Response に変換
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::UserNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            ApiError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        (status, message).into_response()
    }
}

async fn get_user(
    Path(id): Path<u64>,
) -> Result<Json<User>, ApiError> {
    if id == 0 {
        return Err(ApiError::UserNotFound);
    }

    Ok(Json(User { id, name: "Alice".to_string() }))
}
```

---

### 必須のクレート

```toml
[dependencies]
# コア
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["trace", "cors"] }

# シリアライゼーション
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# データベース
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres"] }

# エラーハンドリング
thiserror = "1.0"
anyhow = "1.0"

# バリデーション
validator = { version = "0.16", features = ["derive"] }

# 認証
jsonwebtoken = "9"

# ロギング
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# 環境変数
dotenvy = "0.15"
```

---

## 🏗️ DDD 基礎概念

### 1. Entity（エンティティ）

**定義**: 一意の識別子を持つオブジェクト

**特徴**:
- ID によって識別される
- 属性が変わっても同じエンティティ
- ライフサイクルを持つ

```rust
#[derive(Debug, Clone)]
struct User {
    id: UserId,
    name: String,
    email: Email,
}

impl User {
    fn new(name: String, email: Email) -> Self {
        Self {
            id: UserId::new(),
            name,
            email,
        }
    }

    fn change_email(&mut self, new_email: Email) {
        self.email = new_email;
    }
}
```

---

### 2. Value Object（値オブジェクト）

**定義**: 不変で、値そのものが重要なオブジェクト

**特徴**:
- 不変（immutable）
- 等価性は値で判定
- バリデーションロジックを持つ

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EmailError {
    #[error("Invalid email format")]
    Invalid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn new(email: impl Into<String>) -> Result<Self, EmailError> {
        let email = email.into();

        // バリデーション
        if !email.contains('@') || !email.contains('.') {
            return Err(EmailError::Invalid);
        }

        Ok(Email(email))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// データベースとの変換
impl From<Email> for String {
    fn from(email: Email) -> String {
        email.0
    }
}

impl TryFrom<String> for Email {
    type Error = EmailError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Email::new(s)
    }
}
```

---

### 3. Aggregate（集約）

**定義**: 関連するエンティティと値オブジェクトのまとまり

**特徴**:
- 集約ルート（Aggregate Root）を持つ
- トランザクション境界
- 不変条件を守る

```rust
#[derive(Debug, Clone)]
struct Order {
    id: OrderId,
    items: Vec<OrderItem>,
    status: OrderStatus,
    total: Money,
}

impl Order {
    fn new(id: OrderId) -> Self {
        Self {
            id,
            items: Vec::new(),
            status: OrderStatus::Draft,
            total: Money::zero(),
        }
    }

    fn add_item(&mut self, item: OrderItem) -> Result<(), OrderError> {
        // ビジネスルール: 確定済みの注文には追加できない
        if self.status != OrderStatus::Draft {
            return Err(OrderError::CannotModifyConfirmedOrder);
        }

        self.items.push(item.clone());
        self.total = self.calculate_total();
        Ok(())
    }

    fn confirm(&mut self) -> Result<(), OrderError> {
        // ビジネスルール: 空の注文は確定できない
        if self.items.is_empty() {
            return Err(OrderError::EmptyOrder);
        }

        self.status = OrderStatus::Confirmed;
        Ok(())
    }

    fn calculate_total(&self) -> Money {
        self.items.iter()
            .map(|item| item.price())
            .sum()
    }
}
```

---

### 4. Repository（リポジトリ）

**定義**: データアクセスの抽象化

**特徴**:
- コレクションのようなインターフェイス
- ドメイン層ではトレイト定義
- インフラ層で具体的な実装

#### ドメイン層（トレイト定義）

```rust
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, Error>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, Error>;
    async fn save(&self, user: &User) -> Result<(), Error>;
    async fn delete(&self, id: UserId) -> Result<(), Error>;
}
```

#### インフラ層（実装）

```rust
use sqlx::PgPool;

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, Error> {
        let row = sqlx::query_as::<_, UserRow>(
            "SELECT id, name, email FROM users WHERE id = $1"
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.try_into()).transpose()?)
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, Error> {
        let row = sqlx::query_as::<_, UserRow>(
            "SELECT id, name, email FROM users WHERE email = $1"
        )
        .bind(email.as_str())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.try_into()).transpose()?)
    }

    async fn save(&self, user: &User) -> Result<(), Error> {
        sqlx::query(
            "INSERT INTO users (id, name, email) VALUES ($1, $2, $3)
             ON CONFLICT (id) DO UPDATE SET name = $2, email = $3"
        )
        .bind(user.id().as_uuid())
        .bind(user.name())
        .bind(user.email().as_str())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: UserId) -> Result<(), Error> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

// データベース行との変換
#[derive(sqlx::FromRow)]
struct UserRow {
    id: sqlx::types::Uuid,
    name: String,
    email: String,
}

impl TryFrom<UserRow> for User {
    type Error = Error;

    fn try_from(row: UserRow) -> Result<Self, Self::Error> {
        Ok(User {
            id: UserId::from_uuid(row.id),
            name: row.name,
            email: Email::new(row.email)?,
        })
    }
}
```

---

### 5. Use Case / Application Service

**定義**: ビジネスロジックの実行を調整

**特徴**:
- ユースケースごとに1つ
- トランザクション境界
- ドメインオブジェクトを組み合わせる

```rust
use std::sync::Arc;

pub struct CreateUserUseCase<R: UserRepository> {
    user_repository: Arc<R>,
}

impl<R: UserRepository> CreateUserUseCase<R> {
    pub fn new(user_repository: Arc<R>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(
        &self,
        input: CreateUserInput,
    ) -> Result<UserDto, Error> {
        // バリデーション
        let email = Email::new(input.email)
            .map_err(|e| Error::InvalidInput(e.to_string()))?;

        // 重複チェック
        if self.user_repository.find_by_email(&email).await?.is_some() {
            return Err(Error::EmailAlreadyExists);
        }

        // ドメインオブジェクト生成
        let user = User::new(input.name, email);

        // 永続化
        self.user_repository.save(&user).await?;

        // DTO に変換して返す
        Ok(UserDto::from(user))
    }
}

// 入力データ
#[derive(Debug, Deserialize)]
pub struct CreateUserInput {
    pub name: String,
    pub email: String,
}

// 出力データ（DTO）
#[derive(Debug, Serialize)]
pub struct UserDto {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id().to_string(),
            name: user.name().to_string(),
            email: user.email().as_str().to_string(),
        }
    }
}
```

---

## 📐 DDD ディレクトリ構造

```
src/
├── main.rs                              # エントリーポイント
│
├── domain/                              # ドメイン層
│   ├── mod.rs
│   ├── entities/                        # エンティティ
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   └── order.rs
│   ├── value_objects/                   # 値オブジェクト
│   │   ├── mod.rs
│   │   ├── email.rs
│   │   ├── user_id.rs
│   │   └── money.rs
│   ├── repositories/                    # Repository トレイト
│   │   ├── mod.rs
│   │   ├── user_repository.rs
│   │   └── order_repository.rs
│   └── errors.rs                        # ドメインエラー
│
├── application/                         # アプリケーション層
│   ├── mod.rs
│   ├── use_cases/                       # ユースケース
│   │   ├── mod.rs
│   │   ├── create_user.rs
│   │   ├── get_user.rs
│   │   ├── update_user.rs
│   │   └── delete_user.rs
│   └── dto/                             # データ転送オブジェクト
│       ├── mod.rs
│       └── user_dto.rs
│
├── infrastructure/                      # インフラ層
│   ├── mod.rs
│   ├── database.rs                      # データベース接続
│   └── repositories/                    # Repository 実装
│       ├── mod.rs
│       ├── postgres_user_repository.rs
│       └── postgres_order_repository.rs
│
└── presentation/                        # プレゼンテーション層
    ├── mod.rs
    ├── routes.rs                        # ルート定義
    ├── handlers/                        # ハンドラ
    │   ├── mod.rs
    │   ├── user_handler.rs
    │   └── order_handler.rs
    └── middleware/                      # ミドルウェア
        ├── mod.rs
        └── auth.rs
```

---

## 🔄 レイヤー間の依存関係

```
┌─────────────────────────────────────────┐
│      Presentation Layer (API)           │  ← axum ハンドラ
├─────────────────────────────────────────┤
│      Application Layer (Use Cases)      │  ← ビジネスロジックの調整
├─────────────────────────────────────────┤
│      Domain Layer (Core Business Logic) │  ← エンティティ、値オブジェクト
├─────────────────────────────────────────┤
│      Infrastructure Layer (DB, etc.)    │  ← Repository 実装
└─────────────────────────────────────────┘
```

**依存の方向**: 外側 → 内側（ドメイン層）

- Presentation → Application → Domain
- Infrastructure → Domain（インターフェイスに依存）

**重要**: Domain 層は他のレイヤーに依存しない（依存性逆転の原則）

---

## 💻 axum でのハンドラ実装

```rust
use axum::{
    extract::State,
    Json,
    http::StatusCode,
};
use std::sync::Arc;

// ハンドラ
async fn create_user(
    State(use_case): State<Arc<CreateUserUseCase<PostgresUserRepository>>>,
    Json(input): Json<CreateUserInput>,
) -> Result<Json<UserDto>, ApiError> {
    let user = use_case.execute(input).await?;
    Ok(Json(user))
}

async fn get_user(
    State(use_case): State<Arc<GetUserUseCase<PostgresUserRepository>>>,
    Path(id): Path<String>,
) -> Result<Json<UserDto>, ApiError> {
    let user_id = UserId::parse(&id)
        .map_err(|_| ApiError::InvalidId)?;

    let user = use_case.execute(user_id).await?;
    Ok(Json(user))
}

// ルート設定
pub fn user_routes<R: UserRepository + 'static>(
    user_repository: Arc<R>,
) -> Router {
    let create_use_case = Arc::new(CreateUserUseCase::new(Arc::clone(&user_repository)));
    let get_use_case = Arc::new(GetUserUseCase::new(Arc::clone(&user_repository)));

    Router::new()
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user))
        .with_state(create_use_case)
        .with_state(get_use_case)
}
```

---

## 📖 推奨書籍・リソース

### 書籍

1. **Domain Modeling Made Functional** ⭐⭐⭐⭐⭐
   - 著者: Scott Wlaschin
   - 関数型プログラミング + DDD
   - Rust と相性が良い

2. **Zero To Production In Rust** ⭐⭐⭐⭐⭐
   - URL: https://www.zero2prod.com/
   - プロダクションレベルの Web API 構築
   - actix-web だが、axum でも応用可能

3. **実践ドメイン駆動設計**
   - 著者: Vaughn Vernon
   - DDD の実践的なパターン

### オンラインリソース

1. **axum examples**
   - https://github.com/tokio-rs/axum/tree/main/examples

2. **Rust DDD 参考リポジトリ**
   - https://github.com/reacherhq/backend
   - https://github.com/brooks-builds/full-stack-todo-rust-course

3. **Exercism - Rust Track**
   - https://exercism.org/tracks/rust
   - 演習問題でスキルアップ

---

## ✅ チェックリスト

### Rust + axum の基礎
- [ ] The Async Book を読み終えた
- [ ] tokio チュートリアルを完了した
- [ ] axum examples を5個以上写経した
- [ ] 簡単な CRUD API を自分で作れる
- [ ] エラーハンドリングを実装できる
- [ ] データベース連携ができる

### DDD の理解
- [ ] Entity の概念を理解した
- [ ] Value Object を実装できる
- [ ] Aggregate の境界を設計できる
- [ ] Repository パターンを実装できる
- [ ] Use Case を実装できる
- [ ] レイヤー構造を理解した

### 実践
- [ ] TODO アプリを DDD で実装した
- [ ] 会社のコードの構造を理解した
- [ ] 分からない部分を質問できる状態になった

---

## 💡 学習のコツ

1. **小さく始める**
   - 最初から完璧な DDD を目指さない
   - シンプルな API から始めて、徐々に複雑にする

2. **写経する**
   - axum examples を写経して、動かして、改造する
   - 会社のコードも小さな部分から写経する

3. **個人プロジェクトを作る**
   - TODO アプリ、ブログ、在庫管理など
   - 会社で学んだパターンを試す場所を持つ

4. **質問する**
   - 分からないことはチームメンバーに聞く
   - コードレビューでフィードバックをもらう

5. **継続する**
   - 毎日少しずつコードを書く
   - 1日30分でも続けることが重要

---

## 🎯 次のステップ

1. **今週中にやること**
   - [ ] The Async Book の最初の3章を読む
   - [ ] axum の `hello-world` example を動かす

2. **今月中にやること**
   - [ ] axum examples を10個写経する
   - [ ] 簡単な CRUD API を作る

3. **3ヶ月後の目標**
   - [ ] 会社のコードベースを理解している
   - [ ] 小さな機能を自分で実装できる
   - [ ] DDD のパターンを使いこなせる

---

頑張ってください！分からないことがあればいつでも質問してください。
