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
pub struct Orderbook {
    #[serde(rename = "figi")]
    pub figi: String,
    #[serde(rename = "depth")]
    pub depth: i32,
    #[serde(rename = "bids")]
    pub bids: Vec<crate::tinkoff_api::models::OrderResponse>,
    #[serde(rename = "asks")]
    pub asks: Vec<crate::tinkoff_api::models::OrderResponse>,
    #[serde(rename = "tradeStatus")]
    pub trade_status: crate::tinkoff_api::models::TradeStatus,
    /// Шаг цены
    #[serde(rename = "minPriceIncrement")]
    pub min_price_increment: f64,
    /// Номинал для облигаций
    #[serde(rename = "faceValue", skip_serializing_if = "Option::is_none")]
    pub face_value: Option<f64>,
    #[serde(rename = "lastPrice", skip_serializing_if = "Option::is_none")]
    pub last_price: Option<f64>,
    #[serde(rename = "closePrice", skip_serializing_if = "Option::is_none")]
    pub close_price: Option<f64>,
    /// Верхняя граница цены
    #[serde(rename = "limitUp", skip_serializing_if = "Option::is_none")]
    pub limit_up: Option<f64>,
    /// Нижняя граница цены
    #[serde(rename = "limitDown", skip_serializing_if = "Option::is_none")]
    pub limit_down: Option<f64>,
}

impl Orderbook {
    pub fn new(figi: String, depth: i32, bids: Vec<crate::tinkoff_api::models::OrderResponse>, asks: Vec<crate::tinkoff_api::models::OrderResponse>, trade_status: crate::tinkoff_api::models::TradeStatus, min_price_increment: f64) -> Orderbook {
        Orderbook {
            figi,
            depth,
            bids,
            asks,
            trade_status,
            min_price_increment,
            face_value: None,
            last_price: None,
            close_price: None,
            limit_up: None,
            limit_down: None,
        }
    }
}


