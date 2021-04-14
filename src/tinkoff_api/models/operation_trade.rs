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
pub struct OperationTrade {
    #[serde(rename = "tradeId")]
    pub trade_id: String,
    /// ISO8601
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "price")]
    pub price: f64,
    #[serde(rename = "quantity")]
    pub quantity: i32,
}

impl OperationTrade {
    pub fn new(trade_id: String, date: String, price: f64, quantity: i32) -> OperationTrade {
        OperationTrade {
            trade_id,
            date,
            price,
            quantity,
        }
    }
}


