use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(SysUser::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(SysUser::Id)
              .string()
              .not_null()
              .primary_key(),
          )
          .col(ColumnDef::new(SysUser::Name).string().not_null())
          .col(ColumnDef::new(SysUser::Gender).string().not_null())
          .col(ColumnDef::new(SysUser::Account).string().not_null())
          .col(ColumnDef::new(SysUser::Password).string().not_null())
          .col(ColumnDef::new(SysUser::MobilePhone).string().not_null())
          .col(ColumnDef::new(SysUser::Birthday).date().not_null())
          .col(ColumnDef::new(SysUser::Enabled).boolean().not_null())
          .col(
            ColumnDef::new(SysUser::CreatedAt)
              .date_time()
              .not_null()
              .default(Expr::cust("CURRENT_TIMESTAMP")), // 使用 custom 表达式
          )
          .col(
            ColumnDef::new(SysUser::UpdatedAt)
              .date_time()
              .not_null()
              .default(Expr::cust("CURRENT_TIMESTAMP")), // 使用 custom 表达式
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(SysUser::Table).to_owned())
      .await
  }
}

#[derive(Iden)]
pub enum SysUser {
  Table,
  Id,
  Name,
  Gender,
  Account,
  Password,
  MobilePhone,
  Birthday,
  Enabled,
  CreatedAt,
  UpdatedAt,
}
