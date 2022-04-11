use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal, StdResult, Storage};
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

pub fn upsert_price(storage: &mut dyn Storage, symbol: &str, rate: Decimal) -> StdResult<ProxyPriceResponse> {
    match KEY_PRICES.may_load(storage, symbol)?{
        None => {
            //TODO: calculate_curent_time and safe as last_updated
            let last_updated = 0;
            let response = ProxyPriceResponse{
                rate,
                last_updated
            };
            KEY_PRICES.save(storage, symbol, &response)?;
            Ok(response)
        },
        Some(value) => {//TODO: update},
    }
}

impl Config {
    pub fn as_res(&self) -> ConfigResponse {
        ConfigResponse {
            source_addr: self.source_addr.to_string(),
        }
    }
}
