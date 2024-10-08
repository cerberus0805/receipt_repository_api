use chrono::NaiveDateTime;
use uuid::Uuid;
use crate::models::v1::errors::api_error::ApiError;

use super::{
    response_currency::ResponseCurrency, 
    response_inventory::ResponseInventory, 
    response_store::ResponseStore
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseReceipt {
    pub id: i32,
    pub transaction_date: NaiveDateTime,
    pub is_inventory_taxed: bool,
    pub transaction_id: Option<Uuid>,
    pub currency: ResponseCurrency,
    pub store: ResponseStore,
    pub inventories: Vec<ResponseInventory>
}

#[derive(Serialize)]
pub struct ResponseReceiptPayload {
    pub data: Option<ResponseReceipt>,
    pub error: Option<ApiError>
}

#[derive(Serialize)]
pub struct ResponseReceiptsPayload {
    pub data: Option<Vec<ResponseReceipt>>,
    pub total: Option<i64>,
    pub error: Option<ApiError>
}

#[derive(Serialize)]
pub struct ResponseCreateReceiptPayload {
    pub data: Option<Uuid>,
    pub error: Option<ApiError>
}

#[derive(Serialize)]
pub struct ResponseCreateReceipt {
    pub id: i32,
}
