use sea_orm_migration::prelude::*;

use crate::m20231029_193540_users_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Groups::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Groups::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Groups::Name).string().not_null())
                    .col(ColumnDef::new(Groups::Admins).json_binary().null())
                    .col(ColumnDef::new(Groups::GroupOwner).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-group-owner_id")
                            .from(Groups::Table, Groups::GroupOwner)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Groups::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Groups {
    Table,
    Id,
    Name,
    GroupOwner,
    Admins,
}


