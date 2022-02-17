use cosmwasm_std::{Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::{UserInfo};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: Option<String>,
    pub token_addr: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetConfig { admin:Option<String>, token_addr:Option<String> },
    SetSeedUsers {
        user_infos: Vec<UserInfo>
    },
    AddSeedUser { 
        user_info: UserInfo
    },
    SetPresaleUsers {
        user_infos: Vec<UserInfo>
    },
    AddPresaleUser {
        user_info: UserInfo
    },
    SetIDOUsers {
        user_infos: Vec<UserInfo>
    },
    AddIDOUser {
        user_info: UserInfo
    },
    ClaimPendingTokens{
        wallet: String
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig{},
    GetSeedUsers{},
    GetPresaleUsers{},
    GetIDOUsers{},
    GetBalance{ wallet:String },
}

