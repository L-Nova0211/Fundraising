#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, Env, StdResult,
    Uint128, QueryRequest, BankQuery,
    Coin, AllBalanceResponse,
};

use cw20::{ Cw20QueryMsg, BalanceResponse as Cw20BalanceResponse, TokenInfoResponse };

use crate::msg::{QueryMsg, Config, ProjectInfo};
use crate::state::{PROJECT_INFOS, OWNER};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBalance{ project_id, wallet } => 
            to_binary(&query_balance(deps, _env, project_id, wallet)?),
            
        QueryMsg::GetConfig{ project_id } => 
            to_binary(&query_getconfig(deps, project_id)?),

        QueryMsg::GetProjectInfo{ project_id } => 
            to_binary(&query_getprojectinfo(deps, project_id)?),

        QueryMsg::GetAllProjectInfo{ } =>
            to_binary(&query_getallprojectinfo(deps)?),

        QueryMsg::GetOwner{ } => {
            let owner = OWNER.load(deps.storage).unwrap();
            to_binary(&owner)
        }
            
    }
}

fn query_getallprojectinfo(deps: Deps) -> StdResult<Vec<ProjectInfo>>
{
    let all: StdResult<Vec<_>> = PROJECT_INFOS.range(deps.storage, None, None, 
        cosmwasm_std::Order::Ascending).collect();
    let all = all.unwrap();

    let mut all_project:Vec<ProjectInfo> = Vec::new();
    for x in all{
        all_project.push(x.1);
    }
    Ok(all_project)
}

fn query_getprojectinfo(deps:Deps, project_id: Uint128) -> StdResult<ProjectInfo>
{
    let x = PROJECT_INFOS.load(deps.storage, project_id.u128().into())?;
    Ok(x)
}

fn query_balance(deps:Deps, _env:Env, project_id: Uint128, wallet:String) -> StdResult<AllBalanceResponse>{

    // let uusd_denom = String::from("uusd");
    let mut balance: AllBalanceResponse = deps.querier.query(
        &QueryRequest::Bank(BankQuery::AllBalances {
            address: wallet.clone(),
        }
    ))?;

    let x = PROJECT_INFOS.load(deps.storage, project_id.u128().into())?;

    let token_balance: Cw20BalanceResponse = deps.querier.query_wasm_smart(
        x.clone().config.token_addr,
        &Cw20QueryMsg::Balance{
            address: wallet,
        }
    )?;
    let token_info: TokenInfoResponse = deps.querier.query_wasm_smart(
        x.config.token_addr.clone(),
        &Cw20QueryMsg::TokenInfo{}
    )?;
    balance.amount.push(Coin::new(token_balance.balance.u128(), token_info.name));

    Ok(balance)
}
fn query_getconfig(deps:Deps, project_id: Uint128) -> StdResult<Config> {
    let x = PROJECT_INFOS.load(deps.storage, project_id.u128().into())?;
    Ok(x.config)
}
