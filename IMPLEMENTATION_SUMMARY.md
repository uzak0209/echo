# Echo - 実装機能まとめ

## プロジェクト概要
**Echo** は承認欲求ゼロの気軽SNSです。「いいね」やフォローなどの承認機能を排除し、純粋に自分の思いを投稿できるプラットフォームを提供します。

## 実装済み機能

### 1. 100回表示で自動削除 ✅
**仕様:**
- 投稿は100回表示されたら自動的に削除される
- ユーザーに「あと何回で消える」などのプレッシャーを与えない設計

**実装詳細:**
- `display_count` カラムで表示回数を追跡（backend/src/infrastructure/persistence/models/post.rs）
- 投稿表示時に `incrementDisplayCount` mutation を自動呼び出し（frontend/src/components/PostCard.tsx:21-26）
- カウントが10に達したら自動削除（backend/src/application/usecases/increment_display_count.rs:20-23）
- タイムライン取得時は `display_count < 10` の投稿のみ取得（backend/src/infrastructure/persistence/repositories/post_repository_impl.rs:63-75）

**関連ファイル:**
- Backend:
  - `backend/src/domain/value_objects/display_count.rs:22-24` - is_expired() メソッド
  - `backend/src/application/usecases/increment_display_count.rs` - 削除ロジック
- Frontend:
  - `frontend/src/components/PostCard.tsx:21-26` - useEffect で自動インクリメント

---

### 2. ランダム表示タイムライン ✅
**仕様:**
- 投稿は時系列ではなくランダムな順序で表示される
- 新しい投稿も古い投稿も平等に表示される

**実装詳細:**
- Fisher-Yates シャッフルアルゴリズムで投稿をランダム化（backend/src/application/usecases/get_timeline.rs:28-30）
- クライアント側で5秒ごとに自動リフレッシュ（frontend/src/components/Timeline.tsx:17）

**関連ファイル:**
- Backend: `backend/src/application/usecases/get_timeline.rs:28-30`
- Frontend: `frontend/src/components/Timeline.tsx:17` - pollInterval

---

### 3. 匿名投稿システム ✅
**仕様:**
- ユーザーはランダムに割り当てられた名前とアバターで投稿
- 投稿者は自分の投稿を確認できない（フロントエンドでメッセージ表示）

**実装詳細:**
- ユーザー登録時にランダムな名前とアバターURLを設定
- 投稿にはユーザー情報（display_name, avatar_url）が含まれる
- GraphQL の Post 型に authorName と authorAvatar フィールドを追加

**関連ファイル:**
- Backend:
  - `backend/src/presentation/graphql/types.rs:11-12` - Post type 定義
  - `backend/src/application/dto/post_dto.rs:12-13` - DTO 定義
  - `backend/src/application/usecases/get_timeline.rs:34-48` - ユーザー情報取得
- Frontend:
  - `frontend/src/components/PostCard.tsx:33-40` - アバターと名前の表示
  - `frontend/src/components/CreatePost.tsx:59-61` - 投稿後に見えない旨の注記

---

### 4. JWT認証システム ✅
**仕様:**
- Access Token (5〜15分): API呼び出しに使用、メモリに保存
- Refresh Token (7〜30日): HTTP-only Cookie に保存、トークン再発行に使用

**実装詳細:**
- JWT生成・検証機能（backend/src/infrastructure/auth/jwt.rs）
- Refresh Token は HTTP-only, Secure, SameSite=Strict の Cookie として保存
- Access Token は GraphQL レスポンスの JSON で返却
- フロントエンドでメモリ（Context API）に保持

**セキュリティ:**
- XSS 対策: Refresh Token を HTTP-only Cookie に格納
- CSRF 対策: SameSite=Strict 設定
- Token 有効期限: Access Token は短時間、Refresh Token は長期間

**関連ファイル:**
- Backend:
  - `backend/src/infrastructure/auth/jwt.rs` - JWT サービス
  - `backend/src/main.rs:56-68` - Cookie 設定
  - `backend/src/presentation/graphql/mutation.rs:30-44` - refreshToken mutation
- Frontend:
  - `frontend/src/lib/auth-context.tsx` - 認証コンテキスト

---

### 5. CORS設定（修正済み） ✅
**問題:**
- `allow_credentials(true)` と `allow_origin(Any)` / `allow_methods(Any)` / `allow_headers(Any)` は同時使用不可

**修正内容:**
- Origin を `http://localhost:3000` に制限
- Methods を `GET, POST, OPTIONS` に制限
- Headers を `CONTENT_TYPE, AUTHORIZATION, ACCEPT` に制限

**関連ファイル:**
- `backend/src/main.rs:97-106`

---

### 6. データベーススキーマ（UUID対応） ✅
**修正内容:**
- `posts.user_id` を integer から uuid に変更
- 外部キー制約が正しく動作するように修正

**スキーマ:**

**users テーブル:**
```sql
id           | uuid                     | PRIMARY KEY
display_name | varchar                  | NOT NULL
avatar_url   | varchar                  | NOT NULL
valid        | boolean                  | NOT NULL, DEFAULT true
created_at   | timestamptz              | NOT NULL, DEFAULT CURRENT_TIMESTAMP
```

**posts テーブル:**
```sql
id            | uuid                     | PRIMARY KEY
user_id       | uuid                     | NOT NULL, FOREIGN KEY -> users.id
content       | varchar                  | NOT NULL
image_url     | varchar                  | NULL
valid         | boolean                  | NOT NULL, DEFAULT true
display_count | integer                  | NOT NULL, DEFAULT 0
created_at    | timestamptz              | NOT NULL, DEFAULT CURRENT_TIMESTAMP
```

**関連ファイル:**
- `backend/migration/src/create_posts_table.rs:15` - user_id を uuid に変更

---

## 技術スタック

### フロントエンド
- **Framework:** Next.js 14.2.0 (TypeScript)
- **UI:** Tailwind CSS + shadcn/ui
- **GraphQL Client:** Apollo Client 3.11.0
- **State Management:** React Context API

### バックエンド
- **Language:** Rust
- **Web Framework:** Axum
- **GraphQL:** async-graphql
- **ORM:** SeaORM
- **Database:** PostgreSQL 15
- **Authentication:** JWT (jsonwebtoken crate)

### インフラ
- **Container:** Docker (PostgreSQL)
- **Architecture:** Clean Architecture (Domain-Driven Design)

---

## アーキテクチャ

### バックエンド（Clean Architecture）
```
src/
├── domain/              # ビジネスロジックとルール
│   ├── entities/       # Post, User エンティティ
│   ├── repositories/   # Repository trait
│   ├── value_objects/  # DisplayCount, PostContent 等
│   └── error.rs        # Domain エラー
├── application/        # ユースケース層
│   ├── usecases/       # GetTimeline, CreatePost 等
│   └── dto/            # Data Transfer Object
├── infrastructure/     # 外部依存実装
│   ├── persistence/    # SeaORM 実装
│   └── auth/           # JWT 実装
└── presentation/       # API 層
    └── graphql/        # GraphQL Schema, Query, Mutation
```

### フロントエンド
```
src/
├── app/                # Next.js App Router
├── components/         # React コンポーネント
│   ├── ui/            # shadcn/ui コンポーネント
│   ├── Timeline.tsx   # タイムライン表示
│   ├── PostCard.tsx   # 投稿カード
│   ├── CreatePost.tsx # 投稿作成
│   └── Login.tsx      # ログイン画面
└── lib/
    ├── graphql/        # GraphQL queries/mutations
    └── auth-context.tsx # 認証コンテキスト
```

---

## API (GraphQL)

### Queries
```graphql
# タイムライン取得
timeline(limit: Int!): [Post!]!
```

### Mutations
```graphql
# ユーザー作成（ログイン）
createUser(displayName: String!, avatarUrl: String): AuthResponse!

# トークン更新
refreshToken: RefreshResponse!

# 投稿作成
createPost(content: String!, imageUrl: String, userId: String!): Boolean!

# 表示回数インクリメント
incrementDisplayCount(postId: String!): Boolean!
```

### Types
```graphql
type Post {
  id: String!
  content: String!
  imageUrl: String
  authorName: String!
  authorAvatar: String!
}

type AuthResponse {
  accessToken: String!
  userId: String!
}

type RefreshResponse {
  accessToken: String!
}
```

---

## セットアップ・起動方法

### 前提条件
- Docker & Docker Compose
- Rust (最新安定版)
- Node.js 20+

### 1. データベース起動
```bash
cd /Users/uzak/Projects/echo
docker-compose up -d
```

### 2. マイグレーション実行
```bash
cd backend/migration
cargo run
```

### 3. バックエンド起動
```bash
cd backend
cargo run
# http://localhost:8000 で起動
```

### 4. フロントエンド起動
```bash
cd frontend
npm install
npm run dev
# http://localhost:3000 で起動
```

---

## 改善実装内容まとめ

### 今回の改善点

1. **型の一致性**
   - Post.id を number から string (UUID) に修正
   - バックエンドとフロントエンドの型を統一

2. **ユーザー情報の表示**
   - GraphQL Post type に authorName と authorAvatar を追加
   - PostDto に author 情報を含めるよう拡張
   - GetTimelineUseCase でユーザー情報を取得
   - フロントエンドで投稿者のアバターと名前を表示

3. **CORS設定の修正**
   - allow_credentials と Any の組み合わせ問題を解決
   - 明示的な Origin, Methods, Headers 指定

4. **データベーススキーマの修正**
   - posts.user_id を integer から uuid に変更
   - 外部キー制約の整合性を確保

---

## テスト状況

### バックエンド
- ✅ ユニットテスト実装済み
  - `domain/entities/post.rs:37-98` - Post エンティティ
  - `domain/value_objects/display_count.rs:33-91` - DisplayCount

### フロントエンド
- 手動テスト実施済み
- 自動テスト未実装

---

## 今後の改善案

1. **投稿者情報の完全匿名化**
   - 現在: ユーザー登録時の名前・アバターを表示
   - 改善案: 表示ごとにランダムな名前・アバターを生成

2. **自分の投稿を見れない機能の強化**
   - 現在: フロントエンドでメッセージのみ
   - 改善案: バックエンドで自分の投稿を除外

3. **画像アップロード機能**
   - S3互換ストレージへの画像アップロード実装

4. **エラーハンドリングの改善**
   - より詳細なエラーメッセージ
   - ユーザーフレンドリーなエラー表示

5. **パフォーマンス最適化**
   - タイムライン取得のN+1問題解決（JOINクエリ）
   - キャッシング戦略の導入

6. **テストカバレッジの向上**
   - E2Eテストの追加
   - フロントエンドユニットテストの追加

---

## まとめ

Echo プロジェクトは、承認欲求から解放されたSNS体験を提供するための基本機能を実装済みです。

**主要機能:**
- ✅ 100回表示で自動削除
- ✅ ランダム表示タイムライン
- ✅ 匿名投稿（ユーザー名・アバター表示）
- ✅ JWT認証（Access Token + Refresh Token）
- ✅ Clean Architecture による保守性の高い設計

**技術的特徴:**
- Rust + GraphQL による高速バックエンド
- Next.js + Tailwind による洗練されたUI
- PostgreSQL による信頼性の高いデータ永続化
- Docker による簡単な環境構築

現在の実装により、MVPとしての基本機能は完成しており、今後の機能追加や改善の土台が整っています。
