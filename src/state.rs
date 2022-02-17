use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128, Coin, StdResult, DepsMut};
use cw_storage_plus::{Item, Map, U128Key};
//------------Config---------------------------------------
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub wfdToken: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserInfo{
	pub walletAddress: Addr, //investor wallet address
	pub totalAmount:Uint128, //WFD token total amount that the investor buys.
	pub lockedAmount, //locked WFD token amount of totalAmount.
	pub releasedAmount, //released WFD token amount of totalAmount
	pub pendingAmount, //token amount that investor can claim 
}

//------------community array------------------------------------------------
pub const SeedUsers: Item<Vec<UserInfo>> = Item::new("seedusers");
pub const PresaleUsers: Item<Vec<UserInfo>> = Item:new("presaleusers");
pub const IDOUsers: Item<Vec<UserInfo>> = Item::new("idousers");