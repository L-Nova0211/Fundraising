use cosmwasm_std::{Uint128, Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::{UserInfo, VestingParameter, ProjectInfo};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    AddProject {
        admin:String, 
        token_addr:String, 
        start_time: Uint128 
    },
    SetProjectInfo{
        project_id: u32,
        project_info: ProjectInfo
    },
    SetConfig { 
        project_id: u32,
        admin:String, 
        token_addr:String, 
        start_time: Uint128 
    },
    SetVestingParameters{
        project_id: u32,
        params: Vec<VestingParameter>
    },
    SetSeedUsers {
        project_id: u32,
        user_infos: Vec<UserInfo>
    },
    AddSeedUser {
        project_id: u32,
        wallet: Addr,
        amount: Uint128
    },
    SetPresaleUsers {
        project_id: u32,
        user_infos: Vec<UserInfo>
    },
    AddPresaleUser {
        project_id: u32,
        wallet: Addr,
        amount: Uint128
    },
    SetIDOUsers {
        project_id: u32,
        user_infos: Vec<UserInfo>
    },
    AddIDOUser {
        project_id: u32,
        wallet: Addr,
        amount: Uint128
    },
    ClaimPendingTokens{
        project_id: u32
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig{ project_id: u32 },
    GetProjectInfo{ project_id: u32 },
    GetPendingTokens{ project_id:u32, wallet: String },
    GetBalance{ project_id:u32, wallet: String },
}

