# Echo - 承認欲求ゼロの気軽SNS

投稿者が自分の投稿を確認できない、承認機能のない匿名SNSアプリケーション。

## コンセプト

- ✅ 投稿者は自分の投稿を確認できない
- ✅ 「いいね」やフォローなどの承認機能は一切なし
- ✅ 名前・プロフィール画像はランダム
- ✅ 投稿は10回表示されたら自動削除
- ✅ 他人の投稿はランダムに閲覧可能

## 技術スタック

### バックエンド
- **言語**: Rust
- **フレームワーク**: Axum
- **GraphQL**: async-graphql
- **ORM**: SeaORM
- **データベース**: PostgreSQL
- **アーキテクチャ**: クリーンアーキテクチャ + DDD

### フロントエンド (Web)
- **フレームワーク**: Next.js 14 (App Router)
- **言語**: TypeScript
- **スタイリング**: Tailwind CSS
- **UIコンポーネント**: shadcn/ui
- **GraphQL**: Apollo Client

### フロントエンド (Android)
- **言語**: Kotlin
- **UIフレームワーク**: Jetpack Compose
- **GraphQL**: Apollo Kotlin
- **アーキテクチャ**: MVVM

## プロジェクト構造

```
echo/
├── backend/           # Rust + GraphQL バックエンド
│   ├── src/
│   │   ├── domain/           # ドメイン層
│   │   ├── application/      # アプリケーション層（ユースケース）
│   │   ├── infrastructure/   # インフラ層（リポジトリ実装）
│   │   └── presentation/     # プレゼンテーション層（GraphQL API）
│   └── migration/     # データベースマイグレーション
├── frontend/          # Next.js Webフロントエンド
│   └── src/
│       ├── app/              # Next.js App Router
│       ├── components/       # Reactコンポーネント
│       └── lib/              # ユーティリティ
└── android/           # Android アプリ
    └── app/src/main/
        ├── java/             # Kotlinコード
        └── graphql/          # GraphQLスキーマとクエリ
```

## セットアップ

### 1. データベースのセットアップ

```bash
# PostgreSQLをインストール（macOSの場合）
brew install postgresql@14

# データベースを作成
createdb echo

# 環境変数を設定
cd backend
cp .env.example .env
# .env ファイルを編集してDATABASE_URLを設定
```

### 2. バックエンドのセットアップ

```bash
cd backend

# 依存関係のインストール（Rustが必要）
# Rustがない場合: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# マイグレーションの実行
cargo install sea-orm-cli
sea-orm-cli migrate up -d migration

# サーバーの起動
cargo run
```

バックエンドは http://localhost:8000 で起動します。
GraphQL Playgroundは http://localhost:8000 で確認できます。

### 3. Webフロントエンドのセットアップ

```bash
cd frontend

# 依存関係のインストール
npm install

# 環境変数の設定
cp .env.local.example .env.local

# 開発サーバーの起動
npm run dev
```

Webアプリは http://localhost:3000 で確認できます。

### 4. Androidアプリのセットアップ

1. Android Studioでandroidディレクトリを開く
2. Gradle同期を実行
3. エミュレータまたは実機でアプリを実行

## GraphQL API

### クエリ

```graphql
# タイムラインを取得（ランダム順、表示回数10未満の投稿のみ）
query {
  timeline(limit: 10) {
    id
    content
    imageUrl
  }
}
```

### ミューテーション

```graphql
# 投稿を作成（ランダムなユーザーとして）
mutation {
  createPost(content: "Hello, Echo!", imageUrl: null)
}

# 投稿の表示回数をインクリメント（10回で自動削除）
mutation {
  incrementDisplayCount(postId: 1)
}
```

## アーキテクチャ

### バックエンド（クリーンアーキテクチャ）

```
Presentation Layer (GraphQL API)
        ↓
Application Layer (Use Cases)
        ↓
Domain Layer (Entities, Repositories Interface)
        ↓
Infrastructure Layer (Repository Implementation)
```

**ドメイン層**: ビジネスロジックとエンティティ
- `Post`: 投稿エンティティ（表示回数、自動削除ロジック）
- `User`: ユーザーエンティティ
- `PostRepository`, `UserRepository`: リポジトリインターフェース

**アプリケーション層**: ユースケース
- `CreatePostUseCase`: 投稿作成
- `GetTimelineUseCase`: タイムライン取得（ランダム表示）
- `IncrementDisplayCountUseCase`: 表示回数更新と自動削除

**インフラ層**: データベース実装
- `PostRepositoryImpl`, `UserRepositoryImpl`: SeaORMを使ったリポジトリ実装

**プレゼンテーション層**: GraphQL API
- Query, Mutation定義

## 開発

### バックエンド

```bash
# 開発モード（自動リロード）
cargo install cargo-watch
cargo watch -x run

# テスト
cargo test

# ビルド
cargo build --release
```

### フロントエンド

```bash
# 開発
npm run dev

# ビルド
npm run build

# 本番起動
npm run start
```

トークンの仕組み

  1. トークンの種類と有効期限

  jwt.rs:6-7
  - アクセストークン: 15分間有効
  - リフレッシュトークン: 30日間有効

  両トークンとも TokenType で区別され、JWTのペイロードに含まれます。

  2. トークン発行フロー

  初回ユーザー登録時 (createUser mutation)

  1. ユーザーを作成 → create_user.rs:37-40
  2. アクセストークン + リフレッシュトークンを生成 → create_user.rs:43-50
  3. GraphQLレスポンスでアクセストークンのみ返却 → types.rs:28-31
  4. リフレッシュトークンはHTTPヘッダー経由でCookieに保存 → mutation.rs:25

  3. リフレッシュフロー

  refreshToken mutation 実行時

  1. HTTPレイヤーがCookieからリフレッシュトークンを抽出 → main.rs:30-38
  2. GraphQLコンテキストに注入 → main.rs:42-44
  3. RefreshTokenUseCaseがリフレッシュトークンを検証 → refresh_token.rs:24-27
  4. 新しいアクセストークン + 新しいリフレッシュトークンを両方発行 → refresh_token.rs:34-42
  5. 新リフレッシュトークンをHTTPヘッダー経由でCookie更新 → mutation.rs:41
  6. GraphQLレスポンスで新アクセストークンを返却 → types.rs:43-45

  4. Cookie管理

  main.rs:56-68
  refresh_token=<token>;
  HttpOnly;           // JavaScriptからアクセス不可
  Secure;             // HTTPS必須
  SameSite=Strict;    // CSRF対策
  Max-Age=2592000;    // 30日
  Path=/

  セキュリティ上の強み

  ✅ リフレッシュトークンはHTTPOnly Cookie → XSS攻撃で盗まれない✅ アクセストークンは短命(15分) → 漏洩しても影響範囲が小さい✅
  トークンタイプを厳密に検証 → verify_access_token / verify_refresh_token で分離✅ リフレッシュ時に両方のトークンを更新 →
  トークンローテーション実装済み

  フロントエンドでの利用方法

  // 1. ユーザー登録
  const { data } = await client.mutate({
    mutation: CREATE_USER,
    variables: { displayName: "太郎" }
  });
  // data.createUser.access_token を保存
  // refresh_token は自動でCookieに保存される

  // 2. API呼び出し時
  headers: {
    'Authorization': `Bearer ${accessToken}`
  }

  // 3. アクセストークン期限切れ時
  const { data } = await client.mutate({
    mutation: REFRESH_TOKEN
    // Cookieは自動送信される
  });
  // 新しい access_token を取得


