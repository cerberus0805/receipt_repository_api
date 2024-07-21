use axum::{extract::{rejection::PathRejection, Path, Query, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{models::v1::{errors::api_error::ApiError, parameters::pagination::Pagination, responses::response_currency::{ResponseCurrenciesPayload, ResponseCurrencyPayload}}, repository::DbRepository, services::v1::{converters::api_error_converter_service::ApiErrorConventerService, currencies::currencies_service::CurrencyService}};

pub struct  CurrenciesHandlers {
}

impl CurrenciesHandlers {
    pub async fn get_currency(State(repository): State<DbRepository>, id: Result<Path<u32>, PathRejection>) -> impl IntoResponse {
        let service = CurrencyService::new(repository);
        let api_error_converter_service = ApiErrorConventerService::new();
        if let Ok(s_id) = id {
            let response_currency = service.get_currency(s_id.0 as i32).await;
            match response_currency {
                Ok(response) => {
                    let payload = ResponseCurrencyPayload {
                        data: Some(response),
                        error: None
                    };
                    (StatusCode::OK, Json(payload))
                },
                Err(e) => {
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

    pub async fn get_currencies(State(repository): State<DbRepository>, pagination: Option<Query<Pagination>>) -> impl IntoResponse {
        let service = CurrencyService::new(repository);
        let api_error_converter_service = ApiErrorConventerService::new();
        let currencies_collection = service.get_currencies(pagination.unwrap_or_default().0).await;
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