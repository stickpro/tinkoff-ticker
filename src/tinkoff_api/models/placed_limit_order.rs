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
pub struct PlacedLimitOrder {
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "operation")]
    pub operation: crate::tinkoff_api::models::OperationType,
    #[serde(rename = "status")]
    pub status: crate::tinkoff_api::models::OrderStatus,
    #[serde(rename = "rejectReason", skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<String>,
    /// Сообщение об ошибке
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "requestedLots")]
    pub requested_lots: i32,
    #[serde(rename = "executedLots")]
    pub executed_lots: i32,
    #[serde(rename = "commission", skip_serializing_if = "Option::is_none")]
    pub commission: Option<Box<crate::tinkoff_api::models::MoneyAmount>>,
}

impl PlacedLimitOrder {
    pub fn new(order_id: String, operation: crate::tinkoff_api::models::OperationType, status: crate::tinkoff_api::models::OrderStatus, requested_lots: i32, executed_lots: i32) -> PlacedLimitOrder {
        PlacedLimitOrder {
            order_id,
            operation,
            status,
            reject_reason: None,
            message: None,
            requested_lots,
            executed_lots,
            commission: None,
        }
    }
}


