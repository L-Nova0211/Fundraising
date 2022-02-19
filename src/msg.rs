use cosmwasm_std::{Uint128, Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::{UserInfo, VestingParameter};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: Option<String>,
    pub token_addr: Option<String>,
    pub start_time: Option<Uint128>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetConfig { 
        admin:Option<String>, 
        token_addr:Option<String>, 
        start_block: Option<Uint128> 
    },
    SetVestingParameters{
        params: Vec<VestingParameter>
    },
    SetSeedUsers {
        user_infos: Vec<UserInfo>
    },
    AddSeedUser {
        wallet: Addr,
        amount: Uint128
    },
    SetPresaleUsers {
        user_infos: Vec<UserInfo>
    },
    AddPresaleUser {
        wallet: Addr,
        amount: Uint128
    },
    SetIDOUsers {
        user_infos: Vec<UserInfo>
    },
    AddIDOUser {
        wallet: Addr,
        amount: Uint128
    },
    ClaimPendingTokens{
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig{},
    GetSeedUsers{},
    GetPresaleUsers{},
    GetIDOUsers{},
    GetPendingTokens{ wallet:String },
    GetBalance{ wallet:String },
}

