use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::models::v1::entities::entity_currency::EntityCurrency;
use crate::models::v1::entities::entity_store::EntityStore;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Clone)]
#[diesel(table_name = crate::schema::receipts)]
#[diesel(belongs_to(EntityCurrency, foreign_key = currency_id))]
#[diesel(belongs_to(EntityStore, foreign_key = store_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EntityReceipt {
    pub id: i32,
    pub transaction_date: NaiveDateTime,
    pub is_inventory_taxed: bool,
    pub currency_id: i32,
    pub store_id: i32,
    pub transaction_id: Option<uuid::Uuid>
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::receipts)]
#[diesel(belongs_to(EntityCurrency, foreign_key = currency_id))]
#[diesel(belongs_to(EntityStore, foreign_key = store_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewEntityReceipt {
    pub transaction_date: NaiveDateTime,
    pub is_inventory_taxed: bool,
    pub currency_id: i32,
    pub store_id: i32,
    pub transaction_id: Option<uuid::Uuid>
}