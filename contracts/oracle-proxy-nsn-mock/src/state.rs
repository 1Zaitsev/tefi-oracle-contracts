use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, StdResult, Storage};
use cw_storage_plus::{Item, Map};
use tefi_oracle::proxy::ProxyPriceResponse;

use crate::msg::ConfigResponse;

pub const CONFIG: Item<Config> = Item::new("config");
// key - symbol: &str
// value - ProxyPriceResponse
pub const KEY_PRICES: Map<&str, ProxyPriceResponse> = Map::new("prices");

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Config {
    pub source_addr: Addr,
}

pub fn upsert_price(storage: &mut dyn Storage, symbol: &str, price_info: ProxyPriceResponse) -> StdResult<ProxyPriceResponse> {
    KEY_PRICES.update(storage, symbol, |p: Option<ProxyPriceResponse>| -> StdResult<ProxyPriceResponse>{
        match p {
            Some(mut value) => {
                value = price_info;
                Ok(value)
            }
            None => Ok(price_info),
        }
    })
}

pub fn load_price(storage: &dyn Storage, symbol: &str) -> StdResult<ProxyPriceResponse> {
    KEY_PRICES.load(storage, symbol)
}

impl Config {
    pub fn as_res(&self) -> ConfigResponse {
        ConfigResponse {
            source_addr: self.source_addr.to_string(),
        }
    }
}
