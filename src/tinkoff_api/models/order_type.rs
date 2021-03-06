/*
 * OpenAPI
 *
 * tinkoff.ru/invest OpenAPI.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: al.a.volkov@tinkoff.ru
 * Generated by: https://openapi-generator.tech
 */

/// OrderType : Тип заявки

/// Тип заявки
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrderType {
    #[serde(rename = "Limit")]
    Limit,
    #[serde(rename = "Market")]
    Market,

}

impl ToString for OrderType {
    fn to_string(&self) -> String {
        match self {
            Self::Limit => String::from("Limit"),
            Self::Market => String::from("Market"),
        }
    }
}




