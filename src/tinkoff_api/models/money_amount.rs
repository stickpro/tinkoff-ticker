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
pub struct MoneyAmount {
    #[serde(rename = "currency")]
    pub currency: crate::tinkoff_api::models::Currency,
    #[serde(rename = "value")]
    pub value: f64,
}

impl MoneyAmount {
    pub fn new(currency: crate::tinkoff_api::models::Currency, value: f64) -> MoneyAmount {
        MoneyAmount {
            currency,
            value,
        }
    }
}


