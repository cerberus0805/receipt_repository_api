use axum::{extract::{rejection::{JsonRejection, PathRejection}, Path, Query, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{models::v1::{commands::writer_command::WriterCommand, errors::api_error::ApiError, forms::patch_payload::PatchCurrencyPayload, parameters::{pagination::Pagination, query_filters::KeywordFilters}, responses::response_currency::{ResponseCurrenciesPayload, ResponseCurrencyPayload}}, services::v1::{converters::api_error_converter_service::ApiErrorConventerService, currencies::currencies_service::CurrencyService}, share_state::HandlerState};

pub struct  CurrenciesHandlers {
}

impl CurrenciesHandlers {
    pub async fn get_currency(State(handler_state): State<HandlerState>, id: Result<Path<u32>, PathRejection>) -> impl IntoResponse {
        let service = CurrencyService::new(&handler_state.repository);
        if let Ok(c_id) = id {
            let response_currency = service.get_currency(c_id.0 as i32).await;
            match response_currency {
                Ok(response) => {
                    let payload = ResponseCurrencyPayload {
                        data: Some(response),
                        error: None
                    };
                    (StatusCode::OK, Json(payload))
                },
                Err(e) => {
                    let api_error_converter_service = ApiErrorConventerService::new();
                    let http_status_code = api_error_converter_service.get_http_status_from_api_error(&e);

                    let payload = ResponseCurrencyPayload {
                        data: None,
                        error: Some(e)
                    };
                    (http_status_code, Json(payload))
                }
            }
        }
        else {
            let payload = ResponseCurrencyPayload {
                data: None,
                error: Some(ApiError::InvalidParameter)
            };

            (StatusCode::BAD_REQUEST, Json(payload))
        }
    }

    pub async fn get_currencies(State(handler_state): State<HandlerState>, pagination: Option<Query<Pagination>>) -> impl IntoResponse {
        let service = CurrencyService::new(&handler_state.repository);
        let currencies_collection = service.get_currencies(&pagination.unwrap_or_default().0).await;
        match currencies_collection {
            Ok(responses) => {
                let payload = ResponseCurrenciesPayload {
                    data: Some(responses.partial_collection),
                    total: Some(responses.total_count),
                    error: None
                };
                (StatusCode::OK, Json(payload))
            },
            Err(e) => {
                let api_error_converter_service = ApiErrorConventerService::new();
                let http_return_code = api_error_converter_service.get_http_status_from_api_error(&e);
                let payload = ResponseCurrenciesPayload {
                    data: None,
                    total: None,
                    error: Some(e)
                };
                (http_return_code, Json(payload))
            }
        }
    }

    pub async fn patch_currency(State(handler_state): State<HandlerState>, id: Result<Path<u32>, PathRejection>,  payload: Result<Json<PatchCurrencyPayload>, JsonRejection>) -> impl IntoResponse {
        if id.is_ok() && payload.is_ok() {
            let c_id = id.expect("id should be ok after we have checked").0;
            let c_payload = payload.expect("payload should be ok after we have checked").0;
            let patch_command = WriterCommand::PatchCurrency(c_id as i32, c_payload);
            let _ = handler_state.sender.send(patch_command).await;
            let response = ResponseCurrencyPayload {
                data: None,
                error: None
            };
            (StatusCode::ACCEPTED, Json(response))
        }
        else {
            let payload = ResponseCurrencyPayload {
                data: None,
                error: Some(ApiError::InvalidParameter)
            };

            (StatusCode::BAD_REQUEST, Json(payload))
        }
    }

    pub async fn autocomplete_currencies(State(handler_state): State<HandlerState>, kw: Option<Query<KeywordFilters>>) -> impl IntoResponse {
        let service = CurrencyService::new(&handler_state.repository);
        let currencies_collection;
        if let Some(keyword) = kw {
            currencies_collection = service.autocomplete_currencies(&keyword.0.keyword).await;
        }
        else {
            currencies_collection = service.autocomplete_currencies(&None::<String>).await;
        }

        match currencies_collection {
            Ok(responses) => {
                let payload = ResponseCurrenciesPayload {
                    data: Some(responses.partial_collection),
                    total: Some(responses.total_count),
                    error: None
                };
                (StatusCode::OK, Json(payload))
            },
            Err(e) => {
                let api_error_converter_service = ApiErrorConventerService::new();
                let http_return_code = api_error_converter_service.get_http_status_from_api_error(&e);
                let payload = ResponseCurrenciesPayload {
                    data: None,
                    total: None,
                    error: Some(e)
                };
                (http_return_code, Json(payload))
            }
        }
    }
}