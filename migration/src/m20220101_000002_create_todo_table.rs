use sea_orm_migration::{prelude::*, schema::*};
use super::m20220101_000001_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Todo::Table)
                    .if_not_exists()
                    .col(pk_auto(Todo::Id))
                    .col(uuid_uniq(Todo::Pid))
                    .col(uuid_uniq(Todo::UserPid))
                    .col(string(Todo::Done))
                    .col(string(Todo::Content))
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(ForeignKey::create()
                .name("FK_todo_user")
                .from(Todo::Table, Todo::UserPid)
                .to(User::Table, User::Pid)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade)
                .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Todo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Todo {
    Table,
    Id,
    Pid,
    UserPid,
    Done,
    Content,
}
