use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Reactions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Reactions::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Reactions::PostId).uuid().not_null())
                    .col(ColumnDef::new(Reactions::UserId).uuid().not_null())
                    .col(ColumnDef::new(Reactions::ReactionType).string().not_null())
                    .col(
                        ColumnDef::new(Reactions::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_reactions_post_id")
                            .from(Reactions::Table, Reactions::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_reactions_user_id")
                            .from(Reactions::Table, Reactions::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    // Unique constraint: one reaction type per user per post
                    .index(
                        Index::create()
                            .unique()
                            .name("idx_reactions_user_post_type")
                            .col(Reactions::PostId)
                            .col(Reactions::UserId)
                            .col(Reactions::ReactionType),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Reactions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Reactions {
    Table,
    Id,
    PostId,
    UserId,
    ReactionType,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Posts {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
