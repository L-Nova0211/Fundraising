use super::*;
use cosmwasm_std::{from_binary, Addr, CosmosMsg, WasmMsg,
    BankQuery, BalanceResponse, AllBalanceResponse, Coin, Uint128};
use cosmwasm_std::testing::{mock_env, mock_info, MOCK_CONTRACT_ADDR, mock_dependencies};

use crate::contract::{execute, instantiate, query};
use crate::state::{VestingParameter, Config, UserInfo};
use crate::msg::{QueryMsg, ExecuteMsg, InstantiateMsg};

// use crate::mock_querier::mock_dependencies;
use cw20::Cw20ExecuteMsg;
// use terraswap::asset::{Asset, AssetInfo};
// use terraswap::pair::ExecuteMsg as TerraswapExecuteMsg;

#[test]
fn workflow(){
    let mut deps = mock_dependencies(&[]);
    
    let msg = InstantiateMsg{
        admin: Some(String::from("admin")),
        token_addr: Some(MOCK_CONTRACT_ADDR.to_string()),
        start_time: None,
    };
//instantiate
    let info = mock_info("admin", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
//add community member

    let msg = ExecuteMsg::AddSeedUser{
        wallet: Addr::unchecked("seed1".to_string()),
        amount: Uint128::new(100)
    };

    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    println!("Add seed user{:?}", res);

//-Remove Project-------------------------

    // let info = mock_info("admin", &[Coin::new(105000000, "uusd")]);
    // let msg = ExecuteMsg::RemoveProject{project_id:Uint128::new(1)};
    // let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

//-Get Project-----------------
    let msg = QueryMsg::GetSeedUsers{};
    let allproject = query(deps.as_ref(), mock_env(), msg).unwrap();

    let res:Vec<UserInfo> = from_binary(&allproject).unwrap();
    println!("all Seeders {:?}", res );
}

