use cosmwasm_std::{Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::{UserInfo};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: Option<String>,
    pub wfdToken: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetConfig { admin:Option<String>, wfd_token:Option<String> },
    SetSeedUsers {
        seed_users: Vec<UserInfo>
    },
    AddSeedUsers { 
        user_info: UserInfo
    },
    SetPresaleUsers {
        presale_users: Vec<UserInfo>
    },
    AddPresaleUser {
        user_info: UserInfo
    },
    SetIDOUsers {
        ido_users: Vec<UserInfo>
    },
    AddIDOUser {
        user_info: UserInfo
    }
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

