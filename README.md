# sandbox-office-library-system

社内図書管理システムのサンプル実装

## 🎯 プロジェクトの目的

このプロジェクトは、Rust Clean Architectureの実践的なパターンを実証するためのミニマル実装です。

**重点項目**:
- 静的ディスパッチによる型安全な設計
- レイヤー間の疎結合
- トランザクション境界の明確化
- ドメイン駆動設計の基礎

> ⚠️ ビジネス機能の最適化ではなく、**アーキテクチャパターンの再現**を目的としています

---

## 🚀 クイックスタート

### 🐳 Docker環境（推奨）

#### バックエンド起動

```bash
cd backend

# コンテナ起動（PostgreSQL + アプリケーション）
docker compose up -d

# ログ確認
docker compose logs -f app
```

バックエンドAPIは `http://localhost:8080` で起動します。

```bash
# コンテナ停止
docker compose down

# コンテナ停止 + ボリューム削除（データベースリセット）
docker compose down -v
```

#### フロントエンド起動

```bash
cd frontend

# 依存関係インストール
npm install

# 開発サーバー起動
npm run dev
```

フロントエンドは `http://localhost:5173` で起動します。

### 💻 ローカル環境

#### 前提条件
- Rust (edition 2021以降)
- PostgreSQL
- Diesel CLI

#### セットアップ

```bash
# Diesel CLIのインストール
cargo install diesel_cli --no-default-features --features postgres

cd backend

# データベースのセットアップ
diesel setup

# マイグレーション実行（スキーマファイルも自動生成されます）
diesel migration run

# アプリケーション起動（ログ出力有効化）
RUST_LOG=info cargo run
```

サーバーは `http://localhost:8080` で起動します。

#### Dieselスキーマ設定

このプロジェクトでは、Dieselの**サブスキーマ機能**を使用して、各ドメインのスキーマファイルを別々のディレクトリに生成します。

設定ファイル: [backend/diesel.toml](backend/diesel.toml)

スキーマファイルは `diesel migration run` や `diesel database reset` の実行時に自動生成されます。
- `db_domain/division/src/schema.rs` - divisionsテーブル
- `db_domain/user/src/schema.rs` - usersテーブル
- `db_domain/book/src/schema.rs` - booksテーブル

---

## 📚 ドメインとAPI

### ドメイン構成

| ドメイン | 説明 |
|---------|------|
| **Division（部署）** | 組織の部署を管理 |
| **User（ユーザー）** | システム利用者を管理 |
| **Book（書籍）** | 図書の登録・貸出・返却を管理 |

### REST API エンドポイント

```
POST   /divisions          # 部署作成
GET    /divisions          # 部署一覧取得
POST   /users              # ユーザー作成
GET    /users              # ユーザー一覧取得
POST   /books              # 書籍登録
GET    /books              # 書籍一覧取得
GET    /books/{id}         # 書籍詳細取得
POST   /books/{id}/borrow  # 書籍貸出
POST   /books/{id}/return  # 書籍返却
```

### データベーススキーマ

```sql
-- 部署テーブル
CREATE TABLE divisions (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

-- ユーザーテーブル
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    division_id INT NOT NULL REFERENCES divisions(id)
);

-- 書籍テーブル
CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    division_id INT NOT NULL REFERENCES divisions(id),
    borrowed_by_user_id INT REFERENCES users(id),
    borrowed_at TIMESTAMPTZ
);
```

---

## 🏗️ アーキテクチャ概要

### 設計原則

| 原則 | 説明 |
|------|------|
| **静的ディスパッチ** | Genericsと`where`句を使用、trait objectを使わない |
| **DB抽象化** | `PgConnection`とDieselは`repository_impl`レイヤーのみに閉じ込める |
| **トランザクション境界** | Controllerレイヤーでのみトランザクション管理 |
| **コンテキスト伝搬** | 統一された`TxContext`による単一コンテキストの伝搬 |
| **CRUD組織化** | リポジトリトレイトをCreate/Read/Update/Deleteの4つに整理 |

### レイヤー構成

```
┌─────────────────────────────────────────────────┐
│ Controller (HTTP/トランザクション境界)             │
├─────────────────────────────────────────────────┤
│ Usecase (ユースケースオーケストレーション)          │
├─────────────────────────────────────────────────┤
│ Feature (ビジネスロジック)                        │
├─────────────────────────────────────────────────┤
│ Repository (トレイト定義)                         │
├─────────────────────────────────────────────────┤
│ Repository Impl (Diesel実装)                     │
├─────────────────────────────────────────────────┤
│ DB Domain (Diesel schema + models)              │
└─────────────────────────────────────────────────┘

     ┌──────────────────────────────┐
     │ Domain (値オブジェクト/エンティティ) │
     │ Presenter (JSON DTO)           │
     └──────────────────────────────┘
```

### 技術スタック

#### バックエンド
- **言語**: Rust (edition 2021)
- **Webフレームワーク**: Actix-Web 4
- **ORM**: Diesel 2 (PostgreSQL)
- **DB**: PostgreSQL
- **その他**:
  - r2d2 (コネクションプール)
  - chrono (日時処理)
  - serde/serde_json (JSON シリアライゼーション)
  - env_logger (ログ出力)
  - dotenv (環境変数)

#### フロントエンド
- **言語**: TypeScript
- **フレームワーク**: React 18
- **ビルドツール**: Vite
- **スタイリング**: Tailwind CSS 3
- **HTTP クライアント**: axios
- **状態管理**: TanStack Query (React Query) v5
- **ルーティング**: React Router v6

---

## 📁 ディレクトリ構造

```
sandbox-office-library-system/
├── backend/                     # バックエンド
│   ├── domain/                  # ドメイン層
│   │   ├── context/            # コンテキスト定義（PgConn, TxContext等）
│   │   ├── value_object/<ctx>/ # 値オブジェクト（ID、検証済みプリミティブ）
│   │   ├── entity_object/<ctx>/ # エンティティ
│   │   └── collection_object/<ctx>/ # コレクション
│   │
│   ├── presenter/<ctx>/        # プレゼンター層（JSON DTO）
│   ├── db_domain/<ctx>/        # DB層（Diesel schema + models）
│   │
│   ├── repository/<ctx>/       # リポジトリトレイト定義
│   ├── repository_impl/<ctx>/  # リポジトリ実装（Diesel）
│   │
│   ├── feature/<ctx>/          # フィーチャー層（ビジネスロジック）
│   ├── usecase/<ctx>/          # ユースケース層
│   │
│   ├── controller/<ctx>/       # コントローラー層
│   ├── controller_utils/       # コントローラー共通ユーティリティ
│   │
│   ├── di_service/             # 依存性注入サービス
│   └── app/                    # アプリケーションエントリポイント
│
└── frontend/                    # フロントエンド（React + TypeScript）
    ├── src/
    │   ├── components/         # 共通コンポーネント
    │   │   └── PageHeader.tsx  # ページヘッダー
    │   ├── lib/
    │   │   └── api.ts          # API クライアント（axios）
    │   ├── pages/              # ページコンポーネント
    │   │   ├── Books.tsx       # 書籍管理（一覧・登録・貸出・返却）
    │   │   ├── Divisions.tsx   # 部署管理（一覧・登録）
    │   │   └── Users.tsx       # ユーザー管理（一覧・登録）
    │   ├── App.tsx             # メインアプリ（ルーティング）
    │   ├── main.tsx            # エントリポイント
    │   └── index.css           # グローバルスタイル（Tailwind CSS）
    ├── .env                    # 環境設定
    ├── package.json            # 依存関係
    ├── vite.config.ts          # Vite設定
    └── tailwind.config.js      # Tailwind CSS設定
```

`<ctx>` = `{division, user, book}` の各ドメインコンテキスト

---

## 💡 コア概念

### 1. コンテキスト伝搬

全レイヤーでジェネリック`Ctx`パラメータを使用し、具体的な型は実装時のみ指定:

```
Controller (TxContext)
    ↓ &mut Ctx
Usecase
    ↓ &mut Ctx
Feature
    ↓ &mut Ctx
Repository trait (generic Ctx)
    ↓ where Ctx: PgConn
Repository impl (ctx.conn() で PgConnection取得)
    ↓
Diesel
```

#### PgConnトレイト

データベース接続の抽象化:

```rust
pub trait PgConn {
    fn conn(&mut self) -> &mut PgConnection;
}
```

#### AppContext

アプリケーション全体で共有されるコンテキスト（DBプール保持）

#### TxContext

トランザクションスコープのコンテキスト（AppContext参照 + トランザクション接続）

### 2. トランザクション管理

`controller/utils`の`with_tx`ヘルパーで統一的に管理:

```rust
pub fn with_tx<F, R>(app_ctx: &AppContext, f: F) -> Result<R, anyhow::Error>
where
    F: for<'a> FnOnce(&mut TxContext<'a>) -> Result<R, anyhow::Error>,
{
    let mut conn = app_ctx.pool.get()?;
    conn.transaction(|tx_conn| {
        let mut tx_ctx = TxContext::new(app_ctx, tx_conn);
        f(&mut tx_ctx)
    })
}
```

**重要**: トランザクションを開始できるのはControllerレイヤーのみ

### 3. Executorパターン

統一された`executor`関数で様々なパラメータパターンをサポート:

```rust
// パラメータなし
executor(&ctx, (), |conn, _| { ... })

// 単一パラメータ
executor(&ctx, req, |conn, req| { ... })

// 複数パラメータ（タプル）
executor(&ctx, (*path, req), |conn, (path, req)| { ... })
```

### 4. CRUD Repository組織化

リポジトリトレイトを最大4つのCRUDカテゴリに整理:

```rust
// 例：Book Repository
pub trait BookRepoCreate<Ctx> {
    fn create_book(&self, ctx: &mut Ctx, ...) -> Result<Book>;
}

pub trait BookRepoRead<Ctx> {
    fn find_book(&self, ctx: &mut Ctx, id: BookId) -> Result<Option<Book>>;
    fn list_books(&self, ctx: &mut Ctx) -> Result<Books>;
}

pub trait BookRepoUpdate<Ctx> {
    fn borrow_book(&self, ctx: &mut Ctx, ...) -> Result<Book>;
    fn return_book(&self, ctx: &mut Ctx, ...) -> Result<Book>;
}

pub trait BookRepoDelete<Ctx> {
    // 削除操作は現在未実装
}
```

---

## 🔍 レイヤー詳細

### Domain層

#### domain/context
- **PgConnトレイト**: データベース接続の抽象化
- **AppContext**: アプリケーション全体コンテキスト
- **TxContext**: トランザクションコンテキスト

#### value_object
- ID型（DivisionId, UserId, BookId）と検証済みプリミティブ型を定義
- 他ドメインのID型のみ参照可能（エンティティは参照不可）
- 外部依存なし

#### entity_object
- ドメインエンティティを定義
- 自ドメインのvalue_objectと他ドメインのID型に依存

#### collection_object
- エンティティのコレクションを定義
- entity_objectとvalue_objectに依存

### Presenter層

各ドメインのJSON DTO（Jdto）を定義。Diesel/db_domainに依存しません。

**Division**:
- `CreateDivisionRequestJdto`
- `DivisionResponseJdto`

**User**:
- `CreateUserRequestJdto`
- `UserResponseJdto`

**Book**:
- `CreateBookRequestJdto`
- `BookResponseJdto`
- `BorrowBookRequestJdto`
- `ReturnBookRequestJdto`
- `ErrorJdto`

### DB Domain層

Dieselに関連するコードのみを配置。domain層に依存しません（完全分離）。

- **schema.rs**: `diesel print-schema`で生成されたテーブル定義
- **models.rs**: Queryable/Insertable DTO（Row型）

### Repository層

リポジトリトレイトをCRUDで整理して定義。Ctx型はジェネリックのまま（具体型への言及なし）。

```rust
pub trait BookRepoCreate<Ctx> {
    fn create_book(&self, ctx: &mut Ctx, ...) -> Result<Book>;
}
```

### Repository Implementation層

Dieselを使ったリポジトリの具体実装:

```rust
impl<Ctx> BookRepoCreate<Ctx> for BookRepoImpl
where
    Ctx: PgConn,  // ← ここで初めてPgConnトレイト境界を指定
{
    fn create_book(&self, ctx: &mut Ctx, ...) -> Result<Book> {
        // ctx.conn()でPgConnectionを取得
        let row = diesel::insert_into(books::table)
            .values(&new_row)
            .get_result(ctx.conn())?;

        Self::row_to_domain(row)
    }
}
```

**責務**:
- Dieselクエリの実行
- Domain型 ↔ DbDomain型（Row）の変換
- db_domain型を上位レイヤーに漏らさない

**重要**: Dieselとdb_domainのimportはこのレイヤーのみ

### Feature層

ビジネスロジックを実装。リポジトリをDIで受け取り、Genericsと`where`句のみ使用:

```rust
pub struct BookFeatureService<BookRepo> {
    book_repo: BookRepo,
}

impl<Ctx, BookRepo> BookFeatureCreateBook<Ctx> for BookFeatureService<BookRepo>
where
    BookRepo: BookRepoCreate<Ctx>,
{
    fn execute(&self, ctx: &mut Ctx, ...) -> Result<Book> {
        self.book_repo.create_book(ctx, ...)
    }
}
```

### Usecase層

複数のFeatureを組み合わせてユースケースを実装。Domain型とJdto型の変換を担当:

```rust
impl<Ctx, BookFeature> BookUsecaseCreateBook<Ctx> for BookUsecase<BookFeature>
where
    BookFeature: BookFeatureCreateBook<Ctx>,
{
    fn execute(&self, ctx: &mut Ctx, ...) -> Result<BookResponseJdto> {
        let book = self.book_feature.execute(ctx, ...)?;
        // Domain → Jdto変換
        Ok(book.into())
    }
}
```

### Controller層

HTTPリクエスト/レスポンスの処理とトランザクション管理:

```rust
pub async fn create_book(
    ctx: web::Data<AppContext>,
    req: web::Json<CreateBookRequestJdto>,
) -> Result<web::Json<BookResponseJdto>, BookControllerError>
{
    controller_utils::executor(
        &ctx,
        req.into_inner(),
        |conn, req| {
            di_service::build_book_usecase().execute(conn, &req)
        }
    )
}
```

**責務**:
- HTTPリクエストの受付
- **トランザクション境界の管理**（`executor`内の`with_tx`で実行）
- Jdtoの検証とドメイン型への変換
- JSON レスポンスの返却

### DI Service層

依存性注入のためのビルダー関数を提供:

```rust
pub fn build_book_usecase() -> BookUsecaseImpl {
    let book_repo = BookRepoImpl::new();
    let book_feature = BookFeatureService::new(book_repo);
    BookUsecase::new(book_feature)
}
```

### App層

アプリケーションのエントリポイント:
- Actix-webサーバーの起動
- ルーティング設定
- DBプールの初期化
- AppContextの作成と共有

---

## ⚠️ 依存関係のルール

### 1. Domain層からの依存禁止
- domain層は`presenter`, `db_domain`, `diesel`に依存しない
- value_objectは外部依存なし
- entity_objectは他ドメインのID型のみ参照可能

### 2. Diesel隔離
- `PgConnection`とDieselは`repository_impl`のみに出現
- `db_domain`型は`repository_impl`の外に出さない

### 3. トランザクション境界
- トランザクション開始はControllerレイヤーのみ
- 他のレイヤーではトランザクションを開始しない

### 4. 静的ディスパッチ
- trait objectを使用しない
- すべてGenericsと`where`句で実装

---

## 🛠️ 技術スタック

### バックエンド

| カテゴリ | 技術 |
|---------|------|
| **言語** | Rust (Edition 2021) |
| **Webフレームワーク** | Actix-web |
| **ORM** | Diesel |
| **データベース** | PostgreSQL |
| **接続プール** | r2d2 |
| **インフラ** | Docker, Docker Compose |

### フロントエンド

| カテゴリ | 技術 |
|---------|------|
| **言語** | TypeScript |
| **フレームワーク** | React 18 |
| **ビルドツール** | Vite |
| **スタイリング** | Tailwind CSS |
| **データフェッチ** | TanStack Query (React Query) |
| **ルーティング** | React Router v6 |
| **HTTPクライアント** | Axios |

---

## 🐳 Docker構成

### コンテナ構成

```
┌─────────────────────────────────────┐
│ app (Rust Application)              │
│ - Port: 8080                        │
│ - Backend REST API                  │
└─────────────────────────────────────┘
              ↓
┌─────────────────────────────────────┐
│ postgres (PostgreSQL 16)            │
│ - Port: 5432                        │
│ - Database: sandbox_office_library  │
└─────────────────────────────────────┘
```

### ファイル構成

- [backend/docker-compose.yml](backend/docker-compose.yml) - サービス定義
- [backend/Dockerfile](backend/Dockerfile) - マルチステージビルド
- [backend/.dockerignore](backend/.dockerignore) - ビルド除外設定

### マルチステージビルド

```dockerfile
# Build stage - Rust依存関係 + ビルド
FROM rust:1.83-slim

# Runtime stage - 最小限のバイナリ実行環境
FROM debian:bookworm-slim
```

最終イメージサイズを最小化するため、ビルドと実行環境を分離しています。

---

## 📖 参考情報

このアーキテクチャは以下の原則に基づいています:

- Clean Architecture (Robert C. Martin)
- Domain-Driven Design
- Dependency Injection
- Repository Pattern
- Use Case Driven Development

詳細な設計制約については [CLAUDE.md](CLAUDE.md) を参照してください。
