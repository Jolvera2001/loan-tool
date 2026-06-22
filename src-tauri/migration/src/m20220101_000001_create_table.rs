use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Loans::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Loans::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Loans::DateCreated).date_time().not_null())
                    .col(ColumnDef::new(Loans::DateUpdated).date_time().not_null())
                    .col(ColumnDef::new(Loans::Principal).decimal().not_null())
                    .col(ColumnDef::new(Loans::Rate).decimal().not_null())
                    .col(ColumnDef::new(Loans::NumberOfMonths).decimal().not_null())
                    .col(ColumnDef::new(Loans::MonthlyPayment).decimal().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Loans::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Loans {
    Table,
    Id,
    DateCreated,
    DateUpdated,
    Principal,
    Rate,
    NumberOfMonths,
    MonthlyPayment,
}
