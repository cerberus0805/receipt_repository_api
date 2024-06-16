use diesel::prelude::*;
use serde::{
    Deserialize, 
    Serialize
};
use bigdecimal::BigDecimal;

use crate::models::v1::entities::entity_product::EntityProduct;
use crate::models::v1::entities::entity_receipt::EntityReceipt;

#[derive(Queryable, Selectable, Insertable, AsChangeset, Identifiable, Associations, Serialize, Deserialize, Debug, PartialEq)]
#[diesel(table_name = crate::schema::inventories)]
#[diesel(belongs_to(EntityProduct, foreign_key = product_id))]
#[diesel(belongs_to(EntityReceipt, foreign_key = receipt_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EntityInventory {
    pub id: i32,
    pub price: BigDecimal,
    pub quantity: i32,
    pub product_id: i32,
    pub receipt_id: i32
}