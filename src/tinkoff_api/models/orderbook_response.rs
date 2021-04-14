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
pub struct OrderbookResponse {
    #[serde(rename = "trackingId")]
    pub tracking_id: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "payload")]
    pub payload: Box<crate::tinkoff_api::models::Orderbook>,
}

impl OrderbookResponse {
    pub fn new(tracking_id: String, status: String, payload: crate::tinkoff_api::models::Orderbook) -> OrderbookResponse {
        OrderbookResponse {
            tracking_id,
            status,
            payload: Box::new(payload),
        }
    }
}


