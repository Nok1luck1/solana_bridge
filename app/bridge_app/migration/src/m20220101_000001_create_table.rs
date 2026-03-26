use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(EvmTokens::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EvmTokens::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(EvmTokens::Address)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(EvmTokens::Name).string().not_null())
                    .col(ColumnDef::new(EvmTokens::Decimals).integer().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(SolanaTokens::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SolanaTokens::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SolanaTokens::Mint)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(SolanaTokens::Name).string().not_null())
                    .col(ColumnDef::new(SolanaTokens::Decimals).integer().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(TokenRelations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TokenRelations::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TokenRelations::EvmTokenId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TokenRelations::SolanaTokenId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_relation_evm")
                            .from(TokenRelations::Table, TokenRelations::EvmTokenId)
                            .to(EvmTokens::Table, EvmTokens::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_relation_solana")
                            .from(TokenRelations::Table, TokenRelations::SolanaTokenId)
                            .to(SolanaTokens::Table, SolanaTokens::Id),
                    )
                    .to_owned(),
            )
            .await?;
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
                    .col(ColumnDef::new(Orders::TokenRelationId).integer().not_null())
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
                    .col(ColumnDef::new(Orders::Timestart).big_integer().not_null())
                    .col(ColumnDef::new(Orders::Timeendl).big_integer().not_null())
                    .col(ColumnDef::new(Orders::TxHashSolana).string())
                    .col(ColumnDef::new(Orders::TxHashEVM).string())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_orders_token_relation")
                            .from(Orders::Table, Orders::TokenRelationId)
                            .to(TokenRelations::Table, TokenRelations::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Orders::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(TokenRelations::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(EvmTokens::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(SolanaTokens::Table).to_owned())
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
    TokenRelationId,
    Token0amount,
    Token1amount,
    Timestart,
    Timeendl,
    TxHashSolana,
    TxHashEVM,
}

#[derive(DeriveIden)]
enum EvmTokens {
    Table,
    Id,
    Address,
    Name,
    Decimals,
}

#[derive(DeriveIden)]
enum SolanaTokens {
    Table,
    Id,
    Mint,
    Name,
    Decimals,
}

#[derive(DeriveIden)]
enum TokenRelations {
    Table,
    Id,
    EvmTokenId,
    SolanaTokenId,
}
