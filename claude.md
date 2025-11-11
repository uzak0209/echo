Claude ハッカソン向け設計案（更新版）
プロジェクト名

Claude – 承認欲求ゼロの気軽SNS

コンセプト

投稿者は自分の投稿を確認できない

「いいね」やフォローなどの承認機能は一切なし

ユーザー名は自由に設定可能、プロフィール画像はランダム

投稿は100回表示されたら自動削除

他人の投稿はランダムに閲覧可能

技術スタック
層	技術
フロント Web	Next.js (TypeScript) + Tailwind CSS + shadcn/ui
フロント Android	Kotlin
バックエンド	Rust + GraphQL (async-graphql) + SeaORM
データベース	PostgreSQL
画像ストレージ	S3互換 / ローカルストレージ
フロント設計ポイント

Tailwind CSS

ユーティリティファーストで高速にスタイリング

ハッカソンでのプロトタイプ作成に最適

shadcn/ui

UI コンポーネントライブラリで再利用性が高い

フォーム、カード、ボタン、モーダルなどをすぐ組み込める

デザイン作業を最小化して開発スピードを上げられる

タイムライン表示

投稿カードに画像・テキストを表示

投稿はランダム順で表示

投稿ボタンは即時反映、確認画面なし

SeaORM + GraphQL 設計ポイント

投稿テーブル

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub image_url: Option<String>,
    pub display_count: i32,
    pub valid: bool,
    pub created_at: DateTimeUtc,
}


ユーザーテーブル

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub display_name: String,  // ユーザー名（ユニーク、ログインIDとして使用）
    pub avatar_url: String,     // ランダム生成
    pub password_hash: Option<String>,
    pub valid: bool,
    pub created_at: DateTimeUtc,
}


GraphQL スキーマ例

type User {
    id: ID!
    displayName: String!  # ユーザー名（ユニーク）
    avatarUrl: String!    # ランダム生成
}

type Post {
    id: ID!
    content: String!
    imageUrl: String
    displayCount: Int!
}

type Query {
    timeline(limit: Int!): [Post!]!
}

type Mutation {
    # 認証
    register(username: String!, password: String!): String!  # JWT token
    login(username: String!, password: String!): String!     # JWT token

    # 投稿
    createPost(content: String!, imageUrl: String): Boolean!
}


timeline はランダム表示

投稿は 100回表示 で自動削除

💡 この構成なら Web と Android 両方のクライアントから 同じ GraphQL API を通じて投稿・閲覧が可能

認証

- ユーザー名（display_name）とパスワードによるシンプルな認証
- パスワードはbcryptでハッシュ化
- JWTトークンによるセッション管理

承認欲求ゼロの匿名 SNS 体験をハッカソンで実装できます
