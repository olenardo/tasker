use sea_orm_migration::prelude::*;
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

const FK_SESSIONS_USERID: &str = "fk-sessions-user_id";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        
        // CREATE TABLE "USERS"
        manager
            .create_table(Table::create().table(User::Table).if_not_exists()
            .col(ColumnDef::new(User::Id).uuid().unique_key().not_null().primary_key())
            .col(ColumnDef::new(User::Login).string_len(32).unique_key().not_null())
            .col(ColumnDef::new(User::Password).string().unique_key().not_null())
            .to_owned())
            .await?;

        // CREATE TABLE "SESSIONS"
        manager
            .create_table(Table::create().table(Session::Table).if_not_exists()
            .col(ColumnDef::new(Session::Id).uuid().not_null().unique_key().primary_key())
            .col(ColumnDef::new(Session::Token).string().not_null().unique_key())
            .col(ColumnDef::new(Session::UserId).uuid().not_null())
            .to_owned())
            .await?;
        
        // TABLE RELATIONS - USER 1:M SESSIONS
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name(FK_SESSIONS_USERID)
                    .from(Session::Table, Session::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned()
            )
                .await?;
        
        // INITIAL DATA SEED
        let insert_users = Query::insert()
            .into_table(User::Table)
            .columns([User::Id, User::Login, User::Password])

            .values_panic([
                Uuid::new_v4().into(),
                "test".into(),
                // pass: 1234
                "$2b$14$I3wYkxG7NBA0je7qQqo65eKd4q3xHF5xAH3p0AQpkhONElxAdCl5q".into()
            ])
                .to_owned();
        
        manager.exec_stmt(insert_users).await

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        // DELETE TABLE RELATIONS - USER 1:M SESSIONS
        manager
            .drop_foreign_key(ForeignKey::drop().table(Session::Table).name(FK_SESSIONS_USERID)
            .to_owned())
            .await?;
        
        // DELETE TABLES: [USERS, SESSIONS]
        manager
            .drop_table(Table::drop().table(Session::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Login,
    Password,
}

#[derive(DeriveIden)]
enum Session {
    Table,
    Id,
    UserId,
    Token,
}