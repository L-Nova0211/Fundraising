use cosmwasm_std::{Uint128, Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    AddProject {
        project_id: Uint128,
        admin: String, 
        token_addr: String,
        vesting_addr: Option<String>,
        start_time: Option<Uint128>,
    },
    SetConfig { 
        project_id: Uint128,
        admin: Option<String>, 
        token_addr: Option<String>,
        vesting_addr: Option<String>,
        start_time: Option<Uint128> 
    },
    AddUser {
        project_id: Uint128,
        wallet: Addr,
        stage: String,
        amount: Uint128,
    },
    SetVestingParameters{
        project_id: Uint128,
        params: Vec<VestingParameter>
    },
    SetSeedUsers {
        project_id: Uint128,
        user_infos: Vec<UserInfo>
    },
    AddSeedUser {
        project_id: Uint128,
        wallet: Addr,
        amount: Uint128
    },
    SetPresaleUsers {
        project_id: Uint128,
        user_infos: Vec<UserInfo>
    },
    AddPresaleUser {
        project_id: Uint128,
        wallet: Addr,
        amount: Uint128
    },
    SetIDOUsers {
        project_id: Uint128,
        user_infos: Vec<UserInfo>
    },
    AddIDOUser {
        project_id: Uint128,
        wallet: Addr,
        amount: Uint128
    },
    StartVesting {
        project_id: Uint128
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig{ project_id: Uint128 },
    GetProjectInfo{ project_id: Uint128 },
    GetAllProjectInfo { },
    GetBalance{ project_id: Uint128, wallet: String },
}

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

//-------------Token holder-------------------------------------------
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserInfo{
	pub wallet_address: Addr, //investor wallet address
	pub total_amount: Uint128, //WFD token total amount that the investor buys.
	pub released_amount: Uint128, //released WFD token amount of totalAmount
	pub pending_amount: Uint128, //token amount that investor can claim 
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProjectInfo{
	pub project_id: Uint128,
	pub config: Config,
	pub vest_param: HashMap<String, VestingParameter>,
	pub seed_users: Vec<UserInfo>,
	pub presale_users: Vec<UserInfo>,
	pub ido_users: Vec<UserInfo>,
}
