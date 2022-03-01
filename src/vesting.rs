use cosmwasm_std::{Uint128, Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::msg::{ProjectInfo, VestingParameter, UserInfo};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetConfig {
        admin: String,
    },
    AddProject {
        project_id: Uint128,
        admin: String, 
        token_addr: String, 
        start_time: Uint128 
    },
    SetProjectInfo{
        project_id: Uint128,
        project_info: ProjectInfo
    },
    SetProjectConfig { 
        project_id: Uint128,
        admin:String, 
        token_addr:String, 
        start_time: Uint128 
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
    ClaimPendingTokens{
        project_id: Uint128
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig { project_id: Uint128 },
    GetPendingTokens { project_id: Uint128, wallet: String },
    GetBalance { project_id: Uint128, wallet: String },
    GetProjectInfo { project_id: Uint128 },
    GetAllProjectInfo {},
    GetOwner{ }
}
