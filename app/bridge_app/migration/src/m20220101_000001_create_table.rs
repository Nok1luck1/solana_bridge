use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Orders::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Orders::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Orders::Fromevmtosol).boolean().not_null())
                    .col(ColumnDef::new(Orders::Maker).string().not_null())
                    .col(ColumnDef::new(Orders::Receiver).string().not_null())
                    .col(ColumnDef::new(Orders::Token0).string().not_null())
                    .col(ColumnDef::new(Orders::Token1).string().not_null())
                    .col(
                        ColumnDef::new(Orders::Token0amount)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Orders::Token1amount)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Orders::Timestart).decimal().not_null())
                    .col(ColumnDef::new(Orders::Timeendl).decimal().not_null())
                    .col(ColumnDef::new(Orders::TxHashSolana).string().not_null())
                    .col(ColumnDef::new(Orders::TxHashEVM).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Orders::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Orders {
    Table,
    Id,
    Fromevmtosol,
    Maker,
    Receiver,
    Token0,
    Token1,
    Token0amount,
    Token1amount,
    Timestart,
    Timeendl,
    TxHashSolana,
    TxHashEVM,
}
