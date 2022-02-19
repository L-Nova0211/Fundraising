use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128, Coin, StdResult, DepsMut};
use cw_storage_plus::{Item, Map, U128Key};
//------------Config---------------------------------------
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub token_addr: Addr,
	pub token_name: String,
    pub token_decimal: Uint128,
	pub start_time: Uint128,
}

pub const CONFIG: Item<Config> = Item::new("config");

//------------Vesting parameter---------------------------------------
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VestingParameter{
	pub soon: Uint128,
	pub after: Uint128,
	pub period: Uint128
}

pub const VEST_PARAM: Map<String, VestingParameter> = Map::new("vestingparameter");

//-------------Token holder-------------------------------------------
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserInfo{
	pub wallet_address: Addr, //investor wallet address
	pub total_amount: Uint128, //WFD token total amount that the investor buys.
	pub released_amount: Uint128, //released WFD token amount of totalAmount
	pub pending_amount: Uint128, //token amount that investor can claim 
}

pub const SEED_USERS: Item<Vec<UserInfo>> = Item::new("seedusers");
pub const PRESALE_USERS: Item<Vec<UserInfo>> = Item::new("presaleusers");
pub const IDO_USERS: Item<Vec<UserInfo>> = Item::new("idousers");