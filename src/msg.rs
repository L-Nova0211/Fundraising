use cosmwasm_std::{Uint128, Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::{UserInfo, VestingParameter};

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
        token_addr: Option<String>,
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

