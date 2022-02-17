use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128, Coin, StdResult, DepsMut};
use cw_storage_plus::{Item, Map, U128Key};
//------------Config---------------------------------------
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub tokenAddr: Addr,
    pub tokenDecimal: Uint128,
}

pub const CONFIG: Item<Config> = Item::new("config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserInfo{
	pub walletAddress: Addr, //investor wallet address
	pub totalAmount: Uint128, //WFD token total amount that the investor buys.
	pub lockedAmount: Uint128, //locked WFD token amount of totalAmount.
	pub releasedAmount: Uint128, //released WFD token amount of totalAmount
	pub pendingAmount: Uint128, //token amount that investor can claim 
}

//------------community array------------------------------------------------
pub const SEED_USERS: Item<Vec<UserInfo>> = Item::new("seedusers");
pub const PRESALE_USERS: Item<Vec<UserInfo>> = Item::new("presaleusers");
pub const IDO_USERS: Item<Vec<UserInfo>> = Item::new("idousers");