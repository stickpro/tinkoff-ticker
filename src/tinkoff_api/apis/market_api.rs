/*
 * OpenAPI
 *
 * tinkoff.ru/invest OpenAPI.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: al.a.volkov@tinkoff.ru
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::tinkoff_api::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `market_bonds_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarketBondsGetError {
    Status500(crate::tinkoff_api::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `market_candles_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarketCandlesGetError {
    Status500(crate::tinkoff_api::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `market_currencies_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarketCurrenciesGetError {
    Status500(crate::tinkoff_api::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `market_etfs_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarketEtfsGetError {
    Status500(crate::tinkoff_api::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `market_orderbook_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarketOrderbookGetError {
    Status500(crate::tinkoff_api::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `market_search_by_figi_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarketSearchByFigiGetError {
    Status500(crate::tinkoff_api::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `market_search_by_ticker_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarketSearchByTickerGetError {
    Status500(crate::tinkoff_api::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `market_stocks_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarketStocksGetError {
    Status500(crate::tinkoff_api::models::Error),
    UnknownValue(serde_json::Value),
}


pub async fn market_bonds_get(configuration: &configuration::Configuration, ) -> Result<crate::tinkoff_api::models::MarketInstrumentListResponse, Error<MarketBondsGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/market/bonds", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MarketBondsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn market_candles_get(configuration: &configuration::Configuration, figi: &str, from: String, to: String, interval: crate::tinkoff_api::models::CandleResolution) -> Result<crate::tinkoff_api::models::CandlesResponse, Error<MarketCandlesGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/market/candles", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("figi", &figi.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("from", &from.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("to", &to.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("interval", &interval.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MarketCandlesGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn market_currencies_get(configuration: &configuration::Configuration, ) -> Result<crate::tinkoff_api::models::MarketInstrumentListResponse, Error<MarketCurrenciesGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/market/currencies", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MarketCurrenciesGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn market_etfs_get(configuration: &configuration::Configuration, ) -> Result<crate::tinkoff_api::models::MarketInstrumentListResponse, Error<MarketEtfsGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/market/etfs", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MarketEtfsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn market_orderbook_get(configuration: &configuration::Configuration, figi: &str, depth: i32) -> Result<crate::tinkoff_api::models::OrderbookResponse, Error<MarketOrderbookGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/market/orderbook", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("figi", &figi.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("depth", &depth.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MarketOrderbookGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn market_search_by_figi_get(configuration: &configuration::Configuration, figi: &str) -> Result<crate::tinkoff_api::models::SearchMarketInstrumentResponse, Error<MarketSearchByFigiGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/market/search/by-figi", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("figi", &figi.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MarketSearchByFigiGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn market_search_by_ticker_get(configuration: &configuration::Configuration, ticker: &str) -> Result<crate::tinkoff_api::models::MarketInstrumentListResponse, Error<MarketSearchByTickerGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/market/search/by-ticker", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("ticker", &ticker.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MarketSearchByTickerGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn market_stocks_get(configuration: &configuration::Configuration, ) -> Result<crate::tinkoff_api::models::MarketInstrumentListResponse, Error<MarketStocksGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/market/stocks", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MarketStocksGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

