#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    Addr, to_binary, DepsMut, Env, MessageInfo, Response,
    Uint128, CosmosMsg, WasmMsg
};
use cw2::set_contract_version;
use cw20::{Cw20QueryMsg, BalanceResponse as Cw20BalanceResponse, TokenInfoResponse};

use crate::error::ContractError;
use crate::msg::{ ExecuteMsg, InstantiateMsg, ProjectInfo, UserInfo, VestingParameter, Config};
use crate::state::{PROJECT_INFOS, OWNER, VESTING_ADDR };
use crate::vesting::{ ExecuteMsg as vestingExecuteMsg };

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
    OWNER.save(deps.storage, &owner)?;

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
        ExecuteMsg::SetConfig{ admin, vesting_addr }
            => try_setconfig(deps, _env, info, admin, vesting_addr ),

        ExecuteMsg::AddProject{ project_id, admin, token_addr, vesting_params, start_time }
            => try_addproject(deps, info, project_id, admin, token_addr, vesting_params, start_time ),

        ExecuteMsg::SetProjectConfig{ project_id, admin, token_addr, start_time} 
            => try_setprojectconfig(deps, info, project_id, admin, token_addr, start_time),

        ExecuteMsg::AddUser{ project_id, wallet, stage, amount} 
            => try_adduser(deps, info, project_id, wallet, stage, amount),

        ExecuteMsg::SetVestingParameters{ project_id, params }
            => try_setvestingparameters(deps, info, project_id, params),

        ExecuteMsg::SetSeedUsers { project_id, user_infos } 
            =>  try_setseedusers(deps, info, project_id, user_infos),

        ExecuteMsg::AddSeedUser { project_id, wallet, amount } 
            =>  try_addseeduser(deps, info, project_id, wallet, amount),

        ExecuteMsg::SetPresaleUsers { project_id, user_infos } 
            =>  try_setpresaleusers(deps, info, project_id, user_infos),

        ExecuteMsg::AddPresaleUser { project_id, wallet, amount } 
            =>  try_addpresaleuser(deps, info, project_id, wallet, amount),

        ExecuteMsg::SetIDOUsers { project_id, user_infos } 
            =>  try_setidousers(deps, info, project_id, user_infos),

        ExecuteMsg::AddIDOUser { project_id, wallet, amount } 
            =>  try_addidouser(deps, info, project_id, wallet, amount),
        
        ExecuteMsg::StartVesting { project_id }
            =>  try_startvesting(deps, _env, info, project_id),

    }
}

pub fn try_startvesting(deps: DepsMut, _env:Env, info: MessageInfo, project_id: Uint128)
    ->Result<Response, ContractError>
{
    let mut x: ProjectInfo = PROJECT_INFOS.load(deps.storage, project_id.u128().into())?;
    let y = x.clone();
    if x.config.token_addr == "".to_string() {
        return Err(ContractError::NotTokenAddr { });
    }
    let vesting_addr = VESTING_ADDR.load(deps.storage)?;
    if vesting_addr == "".to_string() {
        return Err(ContractError::NotSetVestAddr { });
    }
    if x.config.start_time == Uint128::zero() {
        x.config.start_time = Uint128::from(_env.block.time.seconds());
    }

    let mut amount = Uint128::zero();
    for user in x.seed_users{
        amount += user.total_amount;
    }
    for user in x.presale_users{
        amount += user.total_amount;
    }
    for user in x.ido_users{
        amount += user.total_amount;
    }
    // let token_addr = deps.api.addr_validate(x.config.token_addr.as_str())?;
    // let token_info: TokenInfoResponse = deps.querier.query_wasm_smart(
    //     token_addr.clone(),
    //     &Cw20QueryMsg::TokenInfo{ }
    // )?;

    // amount = amount * Uint128::from((10u32).pow(token_info.decimals as u32));
    // let token_balance: Cw20BalanceResponse = deps.querier.query_wasm_smart(
    //     token_addr,
    //     &Cw20QueryMsg::Balance{
    //         address: _env.contract.address.to_string(),
    //     }
    // )?;
    // if token_balance.balance < amount {
    //     return Err(ContractError::NotEnoughBalance { })
    // }

    let msg_vesting = WasmMsg::Execute {
            contract_addr: vesting_addr.to_string(),
            msg: to_binary(&vestingExecuteMsg::SetProjectInfo {
                project_id: project_id,
                project_info: y
            }).unwrap(),
        funds: Vec::new()
    };

    Ok(Response::new()
    .add_message(CosmosMsg::Wasm(msg_vesting))
    .add_attribute("action", "Start vesting"))
}
pub fn try_setvestingparameters(deps: DepsMut, info: MessageInfo, project_id: Uint128, params: Vec<VestingParameter>)
    ->Result<Response, ContractError>
{
    let mut x = PROJECT_INFOS.load(deps.storage, project_id.u128().into())?;
    if x.config.owner != info.sender {
        return Err(ContractError::Unauthorized{ });
    }

    x.vest_param = params;

    PROJECT_INFOS.save(deps.storage, project_id.u128().into(), &x)?;
    Ok(Response::new()
    .add_attribute("action", "Set Vesting parameters"))
}

pub fn check_add_userinfo( users: &mut Vec<UserInfo>, wallet: Addr, amount: Uint128)
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
pub fn try_addseeduser(deps: DepsMut, info: MessageInfo, project_id: Uint128, wallet:Addr, amount: Uint128)
    ->Result<Response, ContractError>
{
    let owner = OWNER.load(deps.storage).unwrap();
    let mut x = PROJECT_INFOS.load(deps.storage, project_id.u128().into())?;
    if info.sender != owner && info.sender != x.config.owner {
        return Err(ContractError::Unauthorized{ });
    }

    check_add_userinfo(&mut x.seed_users, wallet, amount);
    PROJECT_INFOS.save(deps.storage, project_id.u128().into(), &x)?;

    Ok(Response::new()
    .add_attribute("action", "Add  User info for Seed stage"))
}
pub fn try_addpresaleuser(deps: DepsMut, info: MessageInfo, project_id: Uint128, wallet: Addr, amount:Uint128)
    ->Result<Response, ContractError>
{
    let owner = OWNER.load(deps.storage).unwrap();
    let mut x = PROJECT_INFOS.load(deps.storage, project_id.u128().into())?;
    if info.sender != owner && info.sender != x.config.owner {
        return Err(ContractError::Unauthorized{ });
    }

    check_add_userinfo(&mut x.presale_users, wallet, amount);
    PROJECT_INFOS.save(deps.storage, project_id.u128().into(), &x)?;

    Ok(Response::new()
    .add_attribute("action", "Add  User info for Presale stage"))
}
pub fn try_addidouser(deps: DepsMut, info: MessageInfo, project_id: Uint128, wallet:Addr, amount:Uint128)
    ->Result<Response, ContractError>
{
    let owner = OWNER.load(deps.storage).unwrap();
    let mut x = PROJECT_INFOS.load(deps.storage, project_id.u128().into())?;
    if info.sender != owner && info.sender != x.config.owner {
        return Err(ContractError::Unauthorized{ });
    }

    check_add_userinfo(&mut x.ido_users, wallet, amount);
    PROJECT_INFOS.save(deps.storage, project_id.u128().into(), &x)?;

    Ok(Response::new()
    .add_attribute("action", "Add  User info for IDO stage"))
}
pub fn try_setseedusers(deps: DepsMut, info: MessageInfo, project_id: Uint128, user_infos: Vec<UserInfo>)
    ->Result<Response, ContractError>
{
    let owner = OWNER.load(deps.storage).unwrap();
    let mut x = PROJECT_INFOS.load(deps.storage, project_id.u128().into())?;
    if info.sender != owner && info.sender != x.config.owner {
        return Err(ContractError::Unauthorized{ });
    }

    x.seed_users = user_infos;

    PROJECT_INFOS.save(deps.storage, project_id.u128().into(), &x)?;

    Ok(Response::new()
    .add_attribute("action", "Set User infos for Seed stage"))
}
pub fn try_setpresaleusers(deps: DepsMut, info: MessageInfo, project_id: Uint128, user_infos: Vec<UserInfo>)
    ->Result<Response, ContractError>
{
    let owner = OWNER.load(deps.storage).unwrap();
    let mut x = PROJECT_INFOS.load(deps.storage, project_id.u128().into())?;
    if info.sender != owner && info.sender != x.config.owner {
        return Err(ContractError::Unauthorized{ });
    }

    x.presale_users = user_infos;

    PROJECT_INFOS.save(deps.storage, project_id.u128().into(), &x)?;

    Ok(Response::new()
    .add_attribute("action", "Set User infos for Presale stage"))
}
pub fn try_setidousers(deps: DepsMut, info: MessageInfo, project_id: Uint128, user_infos: Vec<UserInfo>)
    ->Result<Response, ContractError>
{
    let owner = OWNER.load(deps.storage).unwrap();
    let mut x = PROJECT_INFOS.load(deps.storage, project_id.u128().into())?;
    if info.sender != owner && info.sender != x.config.owner {
        return Err(ContractError::Unauthorized{ });
    }

    x.ido_users = user_infos;

    PROJECT_INFOS.save(deps.storage, project_id.u128().into(), &x)?;

    Ok(Response::new()
    .add_attribute("action", "Set User infos for IDO stage"))
}
pub fn try_adduser(deps: DepsMut, info: MessageInfo, project_id: Uint128, wallet: Addr, stage: String, amount: Uint128)
    ->Result<Response, ContractError>
{
    let owner = OWNER.load(deps.storage).unwrap();
    let x = PROJECT_INFOS.load(deps.storage, project_id.u128().into())?;
    if info.sender != owner && info.sender != x.config.owner {
        return Err(ContractError::Unauthorized{ });
    }
    
    if stage.to_lowercase() == "seed".to_string(){
        try_addseeduser(deps, info, project_id, wallet, amount)?;
    }
    else if stage.to_lowercase() == "presale".to_string(){
        try_addpresaleuser(deps, info, project_id, wallet, amount)?;
    }
    else if stage.to_lowercase() == "ido".to_string(){
        try_addidouser(deps, info, project_id, wallet, amount)?;
    }

    Ok(Response::new()
    .add_attribute("action", "Set User info"))
}

pub fn try_setprojectconfig(deps:DepsMut, info:MessageInfo,
    project_id: Uint128,
    admin: Option<String>, 
    token_addr: Option<String>,
    start_time: Option<Uint128>
) -> Result<Response, ContractError>
{
    //-----------check owner--------------------------
    let owner = OWNER.load(deps.storage).unwrap();
    let mut x = PROJECT_INFOS.load(deps.storage, project_id.u128().into())?;
    if info.sender != owner && info.sender != x.config.owner {
        return Err(ContractError::Unauthorized{});
    }

    x.config.owner = admin
        .and_then(|s| deps.api.addr_validate(s.as_str()).ok()).unwrap();

    x.config.token_addr = match token_addr{
            Some(v) => v,
            None => x.config.token_addr
        };

    x.config.start_time = match start_time {
            Some(v) => v,
            None => x.config.start_time
        };

    PROJECT_INFOS.save(deps.storage, project_id.u128().into(), &x)?;
    Ok(Response::new()
        .add_attribute("action", "SetConfig"))                                
}

pub fn try_addproject(deps:DepsMut, info:MessageInfo,
    project_id: Uint128,
    admin: String, 
    token_addr: String,
    vesting_params: Vec<VestingParameter>,
    start_time: Option<Uint128>
) -> Result<Response, ContractError>
{
    //-----------check owner--------------------------
    let owner = OWNER.load(deps.storage).unwrap();
    if info.sender != owner {
        return Err(ContractError::Unauthorized{});
    }

    let config: Config = Config{
        owner: deps.api.addr_validate(admin.as_str())?,
        token_addr: token_addr,
        start_time : match start_time{
            Some(v) => v,
            None => Uint128::zero()
        }
    };
    let _config = config.clone();

    let project_info: ProjectInfo = ProjectInfo{
        project_id: project_id,
        config: config,
        vest_param: vesting_params,
        seed_users: Vec::new(),
        presale_users: Vec::new(),
        ido_users: Vec::new()
    };

    PROJECT_INFOS.save(deps.storage, project_id.u128().into(), &project_info)?;

    let vesting_addr = VESTING_ADDR.load(deps.storage)?;
    if vesting_addr != "".to_string() {
        let msg_addproject = WasmMsg::Execute {
            contract_addr: vesting_addr.to_string(),
                msg: to_binary(&vestingExecuteMsg::AddProject {
                    project_id: project_id,
                    admin: _config.owner.to_string(), 
                    token_addr: _config.token_addr, 
                    start_time: _config.start_time 
                }).unwrap(),
            funds: Vec::new()
        };
        return Ok(Response::new()
        .add_message(CosmosMsg::Wasm(msg_addproject))
        .add_attribute("action", "Start vesting"));
    }

    Ok(Response::new()
        .add_attribute("action", "Add Project"))                                
}
pub fn try_setconfig(deps:DepsMut, env:Env, info:MessageInfo, admin: String, vesting_addr: String) 
    -> Result<Response, ContractError>
{
    //-----------check owner--------------------------
    // let owner = OWNER.load(deps.storage).unwrap();
    // if info.sender != owner {
    //     return Err(ContractError::Unauthorized{});
    // }

    let admin_addr = deps.api.addr_validate(&admin).unwrap();
    OWNER.save(deps.storage, &admin_addr)?;

    let vesting_contract_address = deps.api.addr_validate(&vesting_addr).unwrap();
    VESTING_ADDR.save(deps.storage, &vesting_contract_address)?;

    // let set_vesting_config = WasmMsg::Execute {
    //     contract_addr: vesting_contract_address.to_string(),
    //     msg: to_binary(
    //         &vestingExecuteMsg::SetConfig {
    //             admin: env.contract.address.to_string(),
    //         }
    //     ).unwrap(),
    //     funds: vec![]
    // };

    Ok(Response::new()
    // .add_messages(vec![
    //     CosmosMsg::Wasm(set_vesting_config)
    // ])
    .add_attribute("action", "SetConfig")) 
}