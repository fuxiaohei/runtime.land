//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_token")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub owner_id: i32,
    pub token: String,
    pub name: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub origin: String,
    pub expired_at: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
