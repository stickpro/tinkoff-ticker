/*
 * OpenAPI
 *
 * tinkoff.ru/invest OpenAPI.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: al.a.volkov@tinkoff.ru
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LimitOrderRequest {
    #[serde(rename = "lots")]
    pub lots: i32,
    #[serde(rename = "operation")]
    pub operation: crate::tinkoff_api::models::OperationType,
    #[serde(rename = "price")]
    pub price: f64,
}

impl LimitOrderRequest {
    pub fn new(lots: i32, operation: crate::tinkoff_api::models::OperationType, price: f64) -> LimitOrderRequest {
        LimitOrderRequest {
            lots,
            operation,
            price,
        }
    }
}


