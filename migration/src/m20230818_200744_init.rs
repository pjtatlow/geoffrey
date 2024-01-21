use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Courses table
        manager
            .create_table(
                Table::create()
                    .table(Courses::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Courses::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Courses::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Courses::Start).date_time().not_null())
                    .col(ColumnDef::new(Courses::End).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        // Users table
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Users::Name).string().not_null().unique_key())
                    .col(ColumnDef::new(Users::NetID).string().not_null())
                    // .col(ColumnDef::new(Users::SemesterID).integer().not_null())
                    // .foreign_key(
                    //     ForeignKey::create()
                    //         .from_tbl(Users::Table)
                    //         .from_col(Users::SemesterID)
                    //         .to_tbl(Courses::Table)
                    //         .to_col(Courses::Id),
                    // )
                    .to_owned(),
            )
            .await?;

        // Forum channels table
        manager
            .create_table(
                Table::create()
                    .table(ForumChannels::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ForumChannels::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .to_owned(),
            )
            .await?;

        // Posts table
        manager
            .create_table(
                Table::create()
                    .table(Posts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Posts::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Posts::PostID).string().not_null())
                    .col(ColumnDef::new(Posts::UserID).string().not_null())
                    .col(ColumnDef::new(Posts::Date).string().not_null())
                    .col(ColumnDef::new(Posts::Kind).string().not_null())
                    .col(
                        ColumnDef::new(Posts::Deleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Name,
    NetID,
}

#[derive(DeriveIden)]
enum Courses {
    Table,
    Id,
    Name,
    Start,
    End,
}

#[derive(DeriveIden)]
enum ForumChannels {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Posts {
    Table,
    Id,
    PostID,
    UserID,
    Date,
    Kind,
    Deleted,
}
