use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use cosmwasm_std::{Addr, Uint128, Coin, StdResult, DepsMut};
use cw_storage_plus::{Item, Map};

//------------Config---------------------------------------
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub token_addr: String,
	pub vesting_addr: String,
	pub start_time: Uint128,
}

//------------Vesting parameter---------------------------------------
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Copy)]
pub struct VestingParameter{
	pub soon: Uint128,
	pub after: Uint128,
	pub period: Uint128
}

// pub const VEST_PARAM: Map<String, VestingParameter> = Map::new("vestingparameter");

//-------------Token holder-------------------------------------------
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserInfo{
	pub wallet_address: Addr, //investor wallet address
	pub total_amount: Uint128, //WFD token total amount that the investor buys.
	pub released_amount: Uint128, //released WFD token amount of totalAmount
	pub pending_amount: Uint128, //token amount that investor can claim 
}

// pub const SEED_USERS: Item<Vec<UserInfo>> = Item::new("seedusers");
// pub const PRESALE_USERS: Item<Vec<UserInfo>> = Item::new("presaleusers");
// pub const IDO_USERS: Item<Vec<UserInfo>> = Item::new("idousers");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProjectInfo{
	pub project_id: u32,
	pub config: Config,
	pub vest_param: HashMap<String, VestingParameter>,
	pub seed_users: Vec<UserInfo>,
	pub presale_users: Vec<UserInfo>,
	pub ido_users: Vec<UserInfo>,
}

pub const OWNER: Item<Addr> = Item::new("owner");

pub const PROJECT_INFOS:Map<u32, ProjectInfo> = Map::new("project_infos");
