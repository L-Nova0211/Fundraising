#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Addr, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128, CosmosMsg, BankMsg, QueryRequest, BankQuery, WasmMsg,
    Coin, AllBalanceResponse
};
use cw2::set_contract_version;
use cw_storage_plus::{U128Key};
use cw20::{Cw20ExecuteMsg, Cw20QueryMsg, BalanceResponse as Cw20BalanceResponse};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, QueryMsg, InstantiateMsg};
use crate::state::{Config, CONFIG, UserInfo, SEED_USERS, PRESALE_USERS, IDO_USERS};

// version info for migration info
const CONTRACT_NAME: &str = "Vesting";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = msg
        .admin
        .and_then(|s| deps.api.addr_validate(s.as_str()).ok()) 
        .unwrap_or(info.sender.clone());

    let tokenAddr = msg
        .token_addr
        .and_then(|s| deps.api.addr_validate(s.as_str()).ok()) 
        .unwrap_or(Addr::unchecked(
            String::from("terra1hzh9vpxhsk8253se0vv5jj6etdvxu3nv8z07zu")));//main net
            // String::from("terra1ajt556dpzvjwl0kl5tzku3fc3p3knkg9mkv8jl")));//test net

    let tokenDecimal = Uint128::new(3);

    let config = Config {
        owner, tokenAddr, tokenDecimal
    };

    CONFIG.save(deps.storage, &config)?;
    SEED_USERS.save(deps.storage, &Vec::new())?;
    PRESALE_USERS.save(deps.storage, &Vec::new())?;
    IDO_USERS.save(deps.storage, &Vec::new())?;

    Ok(Response::new()
        .add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetConfig{ admin, token_addr } 
            => try_setconfig(deps, info, admin, token_addr),

        ExecuteMsg::SetSeedUsers { user_infos } 
            =>  try_setseedusers(deps, info, user_infos),

        ExecuteMsg::AddSeedUser { user_info } 
            =>  try_addseeduser(deps, info, user_info),

        ExecuteMsg::SetPresaleUsers { user_infos } 
            =>  try_setpresaleusers(deps, info, user_infos),

        ExecuteMsg::AddPresaleUser { user_info } 
            =>  try_addpresaleuser(deps, info, user_info),

        ExecuteMsg::SetIDOUsers { user_infos } 
            =>  try_setidousers(deps, info, user_infos),

        ExecuteMsg::AddIDOUser { user_info } 
            =>  try_addidouser(deps, info, user_info),

        ExecuteMsg::ClaimPendingTokens { wallet }
            =>  try_claimpendingtokens(deps, info, wallet)
    }
}
pub fn try_claimpendingtokens(deps: DepsMut, info: MessageInfo, wallet: String)
    ->Result<Response, ContractError>
{

    Ok(Response::new()
    .add_attribute("action", "Claim pending tokens"))
}

pub fn try_addseeduser(deps: DepsMut, info: MessageInfo, user_info: UserInfo)
    ->Result<Response, ContractError>
{
    //-----------check owner--------------------------
    let config = CONFIG.load(deps.storage).unwrap();
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized{});
    }

    let mut users = SEED_USERS.load(deps.storage).unwrap();
    users.push(user_info);
    SEED_USERS.save(deps.storage, &users)?;

    Ok(Response::new()
    .add_attribute("action", "Add  User info for Seed stage"))
}
pub fn try_addpresaleuser(deps: DepsMut, info: MessageInfo, user_info: UserInfo)
    ->Result<Response, ContractError>
{
    //-----------check owner--------------------------
    let config = CONFIG.load(deps.storage).unwrap();
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized{});
    }

    let mut users = SEED_USERS.load(deps.storage).unwrap();
    users.push(user_info);
    PRESALE_USERS.save(deps.storage, &users)?;

    Ok(Response::new()
    .add_attribute("action", "Add  User info for Presale stage"))
}
pub fn try_addidouser(deps: DepsMut, info: MessageInfo, user_info: UserInfo)
    ->Result<Response, ContractError>
{
    //-----------check owner--------------------------
    let config = CONFIG.load(deps.storage).unwrap();
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized{});
    }

    let mut users = IDO_USERS.load(deps.storage).unwrap();
    users.push(user_info);
    IDO_USERS.save(deps.storage, &users)?;

    Ok(Response::new()
    .add_attribute("action", "Add  User info for IDO stage"))
}
pub fn try_setseedusers(deps: DepsMut, info: MessageInfo, user_infos: Vec<UserInfo>)
    ->Result<Response, ContractError>
{
    //-----------check owner--------------------------
    let config = CONFIG.load(deps.storage).unwrap();
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized{});
    }

    SEED_USERS.save(deps.storage, &user_infos);

    Ok(Response::new()
    .add_attribute("action", "Set User infos for Seed stage"))
}
pub fn try_setpresaleusers(deps: DepsMut, info: MessageInfo, user_infos: Vec<UserInfo>)
    ->Result<Response, ContractError>
{
    //-----------check owner--------------------------
    let config = CONFIG.load(deps.storage).unwrap();
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized{});
    }

    PRESALE_USERS.save(deps.storage, &user_infos);

    Ok(Response::new()
    .add_attribute("action", "Set User infos for Presale stage"))
}
pub fn try_setidousers(deps: DepsMut, info: MessageInfo, user_infos: Vec<UserInfo>)
    ->Result<Response, ContractError>
{
    //-----------check owner--------------------------
    let config = CONFIG.load(deps.storage).unwrap();
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized{});
    }

    IDO_USERS.save(deps.storage, &user_infos);

    Ok(Response::new()
    .add_attribute("action", "Set User infos for IDO stage"))
}
pub fn try_setconfig(deps:DepsMut, info:MessageInfo,
    admin:Option<String>, 
    token_addr:Option<String>
) -> Result<Response, ContractError>
{
    //-----------check owner--------------------------
    let config = CONFIG.load(deps.storage).unwrap();
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized{});
    }
    
    let mut config = CONFIG.load(deps.storage).unwrap();

    config.owner =  admin
    .and_then(|s| deps.api.addr_validate(s.as_str()).ok()) 
    .unwrap_or(config.owner);

    config.tokenAddr = token_addr
        .and_then(|s| deps.api.addr_validate(s.as_str()).ok()) 
        .unwrap_or(config.tokenAddr);

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "SetConfig"))                                
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBalance{ wallet } => to_binary(&query_balance(deps, _env, wallet)?),
        QueryMsg::GetConfig{ } => to_binary(&query_getconfig(deps)?),
        QueryMsg::GetSeedUsers{ } => to_binary(&query_seedusers(deps)?),
        QueryMsg::GetPresaleUsers{ } => to_binary(&query_presaleusers(deps)?),
        QueryMsg::GetIDOUsers{ } => to_binary(&query_idousers(deps)?),
    }
}

fn query_seedusers(deps:Deps) -> StdResult<Vec<UserInfo>>{
    let users = SEED_USERS.load(deps.storage).unwrap();
    Ok(users)
}
fn query_presaleusers(deps:Deps) -> StdResult<Vec<UserInfo>>{
    let users = PRESALE_USERS.load(deps.storage).unwrap();
    Ok(users)
}
fn query_idousers(deps:Deps) -> StdResult<Vec<UserInfo>>{
    let users = IDO_USERS.load(deps.storage).unwrap();
    Ok(users)
}
fn query_balance(deps:Deps, _env:Env, wallet:String) -> StdResult<AllBalanceResponse>{

    // let uusd_denom = String::from("uusd");
    let mut balance: AllBalanceResponse = deps.querier.query(
        &QueryRequest::Bank(BankQuery::AllBalances {
            address: wallet.clone(),
        }
    ))?;

    let config = CONFIG.load(deps.storage).unwrap();

    let aust_balance: Cw20BalanceResponse = deps.querier.query_wasm_smart(
        config.tokenAddr,
        &Cw20QueryMsg::Balance{
            address: wallet,
        }
    )?;
    balance.amount.push(Coin::new(aust_balance.balance.u128(), "aust"));

    Ok(balance)
}
fn query_getconfig(deps:Deps) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage).unwrap();
    Ok(config)
}
