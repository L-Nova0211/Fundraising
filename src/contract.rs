#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Addr, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128, CosmosMsg, BankMsg, QueryRequest, BankQuery, WasmMsg,
    Coin, AllBalanceResponse, BlockInfo, Storage
};
use cw2::set_contract_version;
use cw_storage_plus::{U128Key};
use cw20::{Cw20ExecuteMsg, Cw20QueryMsg, BalanceResponse as Cw20BalanceResponse, TokenInfoResponse};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, QueryMsg, InstantiateMsg};
use crate::state::{Config, CONFIG, UserInfo, SEED_USERS, PRESALE_USERS, IDO_USERS, 
    VestingParameter, VEST_PARAM};

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

    let token_addr = msg
        .token_addr
        .and_then(|s| deps.api.addr_validate(s.as_str()).ok()) 
        .unwrap_or(Addr::unchecked(
            "terra1nppndpgfusn7p8nd5d9fqy47xejg0x55jjxe2y".to_string()));//main net
            // "terra1ajt556dpzvjwl0kl5tzku3fc3p3knkg9mkv8jl".to_string()));//test net

    let start_time = msg
        .start_time
        .unwrap_or(Uint128::new(_env.block.time.seconds() as u128));

    let token_info: TokenInfoResponse = deps.querier.query_wasm_smart(
        token_addr.clone(),
        &Cw20QueryMsg::TokenInfo{}
    )?;

    let config = Config {
        owner, 
        token_addr,
        token_name: token_info.name,
        token_decimal: Uint128::new(token_info.decimals as u128),
        start_time,
    };

    CONFIG.save(deps.storage, &config)?;
    SEED_USERS.save(deps.storage, &Vec::new())?;
    PRESALE_USERS.save(deps.storage, &Vec::new())?;
    IDO_USERS.save(deps.storage, &Vec::new())?;

    let sec_per_month = 60 * 60 * 24 * 30;
    let seed_param = VestingParameter {
        soon: Uint128::new(15), //15% unlock at tge
        after: Uint128::new(sec_per_month), //after 1 month
        period: Uint128::new(sec_per_month * 6) //release over 6 month
    };
    let presale_param = VestingParameter {
        soon: Uint128::new(20), //20% unlock at tge
        after: Uint128::new(sec_per_month), //ater 1 month
        period: Uint128::new(sec_per_month * 5) //release over 5 month
    };
    let ido_param = VestingParameter {
        soon: Uint128::new(25), //25% unlock at tge
        after: Uint128::new(sec_per_month), //after 1 month
        period: Uint128::new(sec_per_month * 4) //release over 4 month
    };
    VEST_PARAM.save(deps.storage, "seed".to_string(), &seed_param)?;
    VEST_PARAM.save(deps.storage, "presale".to_string(), &presale_param)?;
    VEST_PARAM.save(deps.storage, "ido".to_string(), &ido_param)?;

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
        ExecuteMsg::SetConfig{ admin, token_addr , start_block} 
            => try_setconfig(deps, info, admin, token_addr, start_block),

        ExecuteMsg::SetVestingParameters{ params }
            => try_setvestingparameters(deps, info, params),

        ExecuteMsg::SetSeedUsers { user_infos } 
            =>  try_setseedusers(deps, info, user_infos),

        ExecuteMsg::AddSeedUser { wallet, amount } 
            =>  try_addseeduser(deps, info, wallet, amount),

        ExecuteMsg::SetPresaleUsers { user_infos } 
            =>  try_setpresaleusers(deps, info, user_infos),

        ExecuteMsg::AddPresaleUser { wallet, amount } 
            =>  try_addpresaleuser(deps, info, wallet, amount),

        ExecuteMsg::SetIDOUsers { user_infos } 
            =>  try_setidousers(deps, info, user_infos),

        ExecuteMsg::AddIDOUser { wallet, amount } 
            =>  try_addidouser(deps, info, wallet, amount),

        ExecuteMsg::ClaimPendingTokens { }
            =>  try_claimpendingtokens(deps, _env,info)
    }
}
pub fn try_setvestingparameters(deps: DepsMut, info: MessageInfo, params: Vec<VestingParameter>)
    ->Result<Response, ContractError>
{
    //-----------check owner--------------------------
    let config = CONFIG.load(deps.storage).unwrap();
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized{});
    }

    VEST_PARAM.save(deps.storage, "seed".to_string(), &params[0])?;
    VEST_PARAM.save(deps.storage, "presale".to_string(), &params[1])?;
    VEST_PARAM.save(deps.storage, "ido".to_string(), &params[2])?;

    Ok(Response::new()
    .add_attribute("action", "Set Vesting parameters"))
}

pub fn calc_pending(store: &dyn Storage, _env: Env, user: UserInfo, stage: String)
    -> Uint128
{
    let param: VestingParameter = VEST_PARAM.load(store, stage).unwrap();

    let config = CONFIG.load(store).unwrap();
    let past_time =Uint128::new(_env.block.time.seconds() as u128) - config.start_time;

    let mut unlocked = Uint128::zero();
    if past_time > Uint128::zero() {
        unlocked = user.total_amount * param.soon / Uint128::new(100);
    }
    let locked = user.total_amount - unlocked;
    if past_time > param.after {
        unlocked += (past_time - param.after) * locked / param.period;
        if unlocked >= user.total_amount{
            unlocked = user.total_amount;
        }
    }

    return unlocked - user.released_amount;
}

pub fn try_claimpendingtokens(deps: DepsMut, _env: Env, info: MessageInfo)
    ->Result<Response, ContractError>
{
    let mut users = SEED_USERS.load(deps.storage).unwrap();
    let mut index =users.iter().position(|x| x.wallet_address == info.sender);
    let mut amount = Uint128::zero();
    if index != None {
        let pending_amount = calc_pending(
            deps.storage, _env.clone(), users[index.unwrap()].clone(), "seed".to_string()
        );
        users[index.unwrap()].released_amount += pending_amount;
        amount += pending_amount;
        SEED_USERS.save(deps.storage, &users)?;
    }

    users = PRESALE_USERS.load(deps.storage).unwrap();
    index =users.iter().position(|x| x.wallet_address == info.sender);
    if index != None {
        let pending_amount = calc_pending(
            deps.storage, _env.clone(), users[index.unwrap()].clone(), "presale".to_string()
        );
        users[index.unwrap()].released_amount += pending_amount;
        amount += pending_amount;
        PRESALE_USERS.save(deps.storage, &users)?;
    }

    users = IDO_USERS.load(deps.storage).unwrap();
    index =users.iter().position(|x| x.wallet_address == info.sender);
    if index != None {
        let pending_amount = calc_pending(
            deps.storage, _env.clone(), users[index.unwrap()].clone(), "ido".to_string()
        );
        users[index.unwrap()].released_amount += pending_amount;
        amount += pending_amount;
        IDO_USERS.save(deps.storage, &users)?;    
    }
    if amount == Uint128::zero() {
        return Err(ContractError::NoPendingTokens{});
    }

    let config = CONFIG.load(deps.storage).unwrap();
    let token_info: TokenInfoResponse = deps.querier.query_wasm_smart(
        config.token_addr.clone(),
        &Cw20QueryMsg::TokenInfo{}
    )?;
    amount = amount * Uint128::new((10 as u128).pow(token_info.decimals as u32)); //for decimals

    let token_balance: Cw20BalanceResponse = deps.querier.query_wasm_smart(
        config.token_addr.clone(),
        &Cw20QueryMsg::Balance{
            address: info.sender.to_string(),
        }
    )?;
    if token_balance.balance < amount {
        return Err(ContractError::NotEnoughBalance{})
    }

    let bank_cw20 = WasmMsg::Execute {
        contract_addr: String::from(config.token_addr),
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: info.sender.to_string(),
            amount: amount,
        }).unwrap(),
        funds: Vec::new()
    };

    Ok(Response::new()
    .add_message(CosmosMsg::Wasm(bank_cw20))
    .add_attribute("action", "Claim pending tokens"))
}

pub fn check_add_userinfo( users: &mut Vec<UserInfo>, wallet:Addr, amount: Uint128)
{
    let index =users.iter().position(|x| x.wallet_address == wallet);
    if index == None {
        users.push(UserInfo { 
            wallet_address: wallet, 
            total_amount: amount, 
            released_amount: Uint128::zero(), 
            pending_amount: Uint128::zero() 
        });
    }
    else{
        users[index.unwrap()].total_amount += amount;
    }
}
pub fn try_addseeduser(deps: DepsMut, info: MessageInfo, wallet:Addr, amount: Uint128)
    ->Result<Response, ContractError>
{
    //-----------check owner--------------------------
    let config = CONFIG.load(deps.storage).unwrap();
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized{});
    }

    let mut users = SEED_USERS.load(deps.storage).unwrap();
    check_add_userinfo(&mut users, wallet, amount);
    SEED_USERS.save(deps.storage, &users)?;

    Ok(Response::new()
    .add_attribute("action", "Add  User info for Seed stage"))
}
pub fn try_addpresaleuser(deps: DepsMut, info: MessageInfo, wallet: Addr, amount:Uint128)
    ->Result<Response, ContractError>
{
    //-----------check owner--------------------------
    let config = CONFIG.load(deps.storage).unwrap();
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized{});
    }

    let mut users = SEED_USERS.load(deps.storage).unwrap();
    check_add_userinfo(&mut users, wallet, amount);
    PRESALE_USERS.save(deps.storage, &users)?;

    Ok(Response::new()
    .add_attribute("action", "Add  User info for Presale stage"))
}
pub fn try_addidouser(deps: DepsMut, info: MessageInfo, wallet:Addr, amount:Uint128)
    ->Result<Response, ContractError>
{
    //-----------check owner--------------------------
    let config = CONFIG.load(deps.storage).unwrap();
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized{});
    }

    let mut users = IDO_USERS.load(deps.storage).unwrap();
    check_add_userinfo(&mut users, wallet, amount);
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

    SEED_USERS.save(deps.storage, &user_infos)?;

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

    PRESALE_USERS.save(deps.storage, &user_infos)?;

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

    IDO_USERS.save(deps.storage, &user_infos)?;

    Ok(Response::new()
    .add_attribute("action", "Set User infos for IDO stage"))
}
pub fn try_setconfig(deps:DepsMut, info:MessageInfo,
    admin:Option<String>, 
    token_addr:Option<String>,
    start_block: Option<Uint128>
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

    config.token_addr = token_addr
        .and_then(|s| deps.api.addr_validate(s.as_str()).ok()) 
        .unwrap_or(config.token_addr);

    config.start_time = start_block
        .unwrap_or(config.start_time);

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
        QueryMsg::GetPendingTokens{ wallet } => to_binary(&query_pendingtokens(deps, _env, wallet)?),
        QueryMsg::GetIDOUsers{ } => to_binary(&query_idousers(deps)?),
    }
}
fn query_pendingtokens(deps:Deps, _env:Env, wallet: String) -> StdResult<Uint128> {
    let mut users = SEED_USERS.load(deps.storage).unwrap();
    let mut index =users.iter().position(|x| x.wallet_address == wallet);
    let mut amount = Uint128::zero();
    if index != None {
        let pending_amount = calc_pending(
            deps.storage, _env.clone(), users[index.unwrap()].clone(), "seed".to_string()
        );
        amount += pending_amount;
    }

    users = PRESALE_USERS.load(deps.storage).unwrap();
    index =users.iter().position(|x| x.wallet_address == wallet);
    if index != None {
        let pending_amount = calc_pending(
            deps.storage, _env.clone(), users[index.unwrap()].clone(), "presale".to_string()
        );
        amount += pending_amount;
    }

    users = IDO_USERS.load(deps.storage).unwrap();
    index =users.iter().position(|x| x.wallet_address == wallet);
    if index != None {
        let pending_amount = calc_pending(
            deps.storage, _env.clone(), users[index.unwrap()].clone(), "ido".to_string()
        );
        amount += pending_amount;
    }

    Ok(amount)
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

    let token_balance: Cw20BalanceResponse = deps.querier.query_wasm_smart(
        config.token_addr,
        &Cw20QueryMsg::Balance{
            address: wallet,
        }
    )?;
    balance.amount.push(Coin::new(token_balance.balance.u128(), config.token_name));

    Ok(balance)
}
fn query_getconfig(deps:Deps) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage).unwrap();
    Ok(config)
}
