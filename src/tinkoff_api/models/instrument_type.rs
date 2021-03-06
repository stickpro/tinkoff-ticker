/*
 * OpenAPI
 *
 * tinkoff.ru/invest OpenAPI.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: al.a.volkov@tinkoff.ru
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InstrumentType {
    #[serde(rename = "Stock")]
    Stock,
    #[serde(rename = "Currency")]
    Currency,
    #[serde(rename = "Bond")]
    Bond,
    #[serde(rename = "Etf")]
    Etf,

}

impl ToString for InstrumentType {
    fn to_string(&self) -> String {
        match self {
            Self::Stock => String::from("Stock"),
            Self::Currency => String::from("Currency"),
            Self::Bond => String::from("Bond"),
            Self::Etf => String::from("Etf"),
        }
    }
}




