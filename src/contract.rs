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
use crate::state::{Config, CONFIG, PROJECTSTATES, ProjectState, BackerState,
        PROJECT_SEQ, COMMUNITY, Milestone, Vote, save_projectstate, TeamMember};

use crate::market::{ExecuteMsg as AnchorMarket, Cw20HookMsg,
    QueryMsg as AnchorQuery, EpochStateResponse};                    

// version info for migration info
const CONTRACT_NAME: &str = "WEFUND";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const UST: u128 = 1000000; //ust unit

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

    let wefund = msg
        .wefund
        .and_then(|s| deps.api.addr_validate(s.as_str()).ok()) 
        .unwrap_or(info.sender.clone());

    let anchor_market = msg
        .anchor_market
        .and_then(|s| deps.api.addr_validate(s.as_str()).ok()) 
        .unwrap_or(Addr::unchecked(
            String::from("terra1sepfj7s0aeg5967uxnfk4thzlerrsktkpelm5s")));//main net
            // String::from("terra15dwd5mj8v59wpj0wvt233mf5efdff808c5tkal")));//test net
    let aust_token = msg
        .aust_token
        .and_then(|s| deps.api.addr_validate(s.as_str()).ok()) 
        .unwrap_or(Addr::unchecked(
            String::from("terra1hzh9vpxhsk8253se0vv5jj6etdvxu3nv8z07zu")));//main net
            // String::from("terra1ajt556dpzvjwl0kl5tzku3fc3p3knkg9mkv8jl")));//test net
    let config = Config {
        owner, wefund, anchor_market, aust_token,
    };

    CONFIG.save(deps.storage, &config)?;
    PROJECT_SEQ.save(deps.storage, &Uint128::new(0))?;
    COMMUNITY.save(deps.storage, &Vec::new())?;

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
        ExecuteMsg::SetConfig{ admin, wefund, anchor_market, aust_token } 
            => try_setconfig(deps, info, admin, wefund, anchor_market, aust_token),
        ExecuteMsg::AddProject { 
            project_company,
            project_title,
            project_description,
            project_ecosystem,
            project_createddate,
            project_saft,
            project_logo,
            project_whitepaper,
            project_website,
            project_email,
            creator_wallet,
            project_collected,
            project_milestones,
            project_teammembers,
        } => 
            try_addproject(deps, info, 
                project_company,
                project_title,
                project_description,
                project_ecosystem,
                project_createddate,
                project_saft,
                project_logo,
                project_whitepaper,
                project_website,
                project_email,
                creator_wallet,
                project_collected,
                project_milestones,
                project_teammembers,
            ),

        ExecuteMsg::Back2Project { project_id, backer_wallet , otherchain, otherchain_wallet} => 
            try_back2project(deps, info, project_id, backer_wallet, otherchain, otherchain_wallet),

        ExecuteMsg::CompleteProject{ project_id } =>
            try_completeproject(deps, _env, project_id ),

        ExecuteMsg::FailProject{ project_id } =>
            try_failproject(deps, _env, project_id),
        
        ExecuteMsg::RemoveProject{ project_id } =>
            try_removeproject(deps, info, project_id),
        
        ExecuteMsg::TransferAllCoins{wallet} =>
            try_transferallcoins(deps, _env, info, wallet),

        ExecuteMsg::AddCommunitymember{wallet} =>
            try_addcommunitymember(deps, wallet),

        ExecuteMsg::RemoveCommunitymember{wallet} =>
            try_removecommunitymember(deps, wallet),

        ExecuteMsg::WefundApprove{project_id, deadline} =>
            try_wefundapprove(deps, info, project_id, deadline),

        ExecuteMsg::SetCommunityVote{project_id, wallet, voted} =>
            try_setcommunityvote(deps, project_id, wallet, voted),

        ExecuteMsg::SetMilestoneVote{project_id, wallet, voted} =>
            try_setmilestonevote(deps, _env, info, project_id, wallet, voted),

        ExecuteMsg::ReleaseMilestone{project_id} =>
            try_releasemilestone(deps, _env, project_id),

        ExecuteMsg::SetProjectStatus{project_id, status} =>
            try_setprojectstatus(deps, info, project_id, status),
    }
}
pub fn try_setprojectstatus(deps: DepsMut, info: MessageInfo, project_id: Uint128, status: Uint128)
    ->Result<Response, ContractError>
{
    //-----------check owner--------------------------
    let config = CONFIG.load(deps.storage).unwrap();
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized{});
    }
//    let x:ProjectState = PROJECTSTATES.load(deps.storage, _project_id.u128().into())?;
    //-------update-------------------------
    PROJECTSTATES.update(deps.storage, project_id.u128().into(), |op| match op {
        None => Err(ContractError::NotRegisteredProject {}),
        Some(mut project) => {
            project.project_status = status;
            Ok(project)
        }
    })?;
    Ok(Response::new()
    .add_attribute("action", "Set project status"))
}
pub fn convert_str_int(str: String)
    ->u128
{
    let bytes = str.into_bytes();
    let mut res: u128 = 0;
    let mut dot = false;
    let mut dotbelow = 0;

    for i in 0..bytes.len(){
        if bytes[i] < 48{
            dot = true;
        }
        else if dotbelow < 6 {
            res = res * 10 + (bytes[i] - 48) as u128;
            if dot {
                dotbelow += 1;
            }
        }
    }
    return res;
}
pub fn try_releasemilestone(deps: DepsMut, _env: Env, _project_id: Uint128 ) 
    -> Result<Response, ContractError>
{
    //--------Get project info----------------------------
    let x:ProjectState = PROJECTSTATES.load(deps.storage, _project_id.u128().into())?;

    //--------Checking project status-------------------------
    if x.project_status != Uint128::new(3){//only releasing status
        return Err(ContractError::NotCorrectStatus{status: x.project_status});
    }

    //---------get hope to release amount---------------------------
    let step = x.project_milestonestep.u128() as usize;
    let release_amount = 
        x.milestone_states[step].milestone_amount.u128() * UST;

    //---------calc total deposited to anchor----------------
    //----------map to vec-----------------------
    let all: StdResult<Vec<_>> = PROJECTSTATES.range(deps.storage, None, None, 
        cosmwasm_std::Order::Ascending).collect();
    let all = all.unwrap();

    let mut total_deposited = 0;
    for x in all{
        let prj = x.1;
        total_deposited += prj.communitybacked_amount.u128() + prj.backerbacked_amount.u128();

        for i in 0..(prj.project_milestonestep.u128() as usize){
            total_deposited -= prj.milestone_states[i].milestone_amount.u128() * UST;
        }
    }
    //----------load config and read aust token address-----------------
    let config = CONFIG.load(deps.storage).unwrap();
    
    //--------get aust balance---------------------
    let aust_balance: Cw20BalanceResponse = deps.querier.query_wasm_smart(
        config.aust_token.clone(),
        &Cw20QueryMsg::Balance{
            address: _env.contract.address.to_string(),
        }
    )?;

    //----------calc declaim aust amount---aust*(release/total)-----------
    let mut estimate_exchange_rate = total_deposited * UST/aust_balance.balance.u128();

    //--------get exchange rate between ust and aust ---------------------
    let epoch: EpochStateResponse = deps.querier.query_wasm_smart(
        config.anchor_market.to_string(),
        &AnchorQuery::EpochState{
            block_height: None,
            distributed_interest: None,
        }
    )?;
    let epoch_exchange_rate = convert_str_int(epoch.exchange_rate.to_string());
    
    if estimate_exchange_rate < epoch_exchange_rate{
        estimate_exchange_rate = epoch_exchange_rate;
    }

    let withdraw_amount = release_amount * UST / estimate_exchange_rate;
    let release_amount = withdraw_amount * epoch_exchange_rate / UST;

    //----ask aust_token for transfer to anchor martket and execute redeem_stable ----------
    let withdraw = WasmMsg::Execute {
        contract_addr: String::from(config.aust_token),
        msg: to_binary(&Cw20ExecuteMsg::Send {
            contract: config.anchor_market.to_string(),
            msg: to_binary(&Cw20HookMsg::RedeemStable{}).unwrap(), //redeem_stable{}
            amount: Uint128::new(withdraw_amount)
        }).unwrap(),
        funds: Vec::new()
    };

    // return Err(ContractError::Testing{
    //     aust_balance: aust_balance.balance.to_string(),
    //     estimate_exchange_rate: estimate_exchange_rate.to_string(),
    //     epoch_exchange_rate: epoch_exchange_rate.to_string(),
    //     withdraw_amount: withdraw_amount.to_string(),
    //     release_amount: release_amount.to_string()
    // });

    //---------send to creator wallet-------------
    let ust_release = Coin::new(release_amount, "uusd");
    let send2_creator = BankMsg::Send { 
        to_address: x.creator_wallet.to_string(),
        amount: vec![ust_release] 
    };

    Ok(Response::new()
    .add_messages(vec![
        CosmosMsg::Wasm(withdraw),
        CosmosMsg::Bank(send2_creator)])
    .add_attribute("action", "release milestone")
    .add_attribute("epoch_exchange_rate", epoch.exchange_rate.to_string())
    )
}
pub fn try_setmilestonevote(deps: DepsMut, _env:Env, info:MessageInfo, project_id: Uint128, wallet: String, voted: bool)
    -> Result<Response, ContractError>
{
    let mut x:ProjectState = PROJECTSTATES.load(deps.storage, project_id.u128().into())?;
    
    //-------check project status-------------------
    if x.project_status != Uint128::new(3) { //only releasing status
        return Err(ContractError::NotCorrectStatus{status:x.project_status});
    }

    let wallet = deps.api.addr_validate(&wallet).unwrap();
    let step = x.project_milestonestep.u128() as usize;

    if x.milestone_states[step].milestone_status != Uint128::zero(){//only voting status
        return Err(ContractError::NotCorrectMilestoneStatus{
            step:step, status:x.milestone_states[step].milestone_status 
        })
    }

    //------set vot status and check all voted for same backer--------------------
    // let index = x.milestone_states[step].milestone_votes.iter().position(|x|x.wallet == wallet).unwrap();
    // x.milestone_states[step].milestone_votes[index].voted = voted;

    let mut all_voted = true;
    for vote in x.milestone_states[step].milestone_votes.iter_mut() {
        if vote.wallet == wallet{
            vote.voted = voted;
        }
        all_voted = all_voted & vote.voted;
    }

    let mut deps = deps;
    if all_voted{
        x.milestone_states[step].milestone_status = Uint128::new(1); //switch to releasing status
        //-----------------release function---------------
        let res = execute(deps.branch(), _env, info, 
                    ExecuteMsg::ReleaseMilestone{project_id});

        x.milestone_states[step].milestone_status = Uint128::new(2); //switch to released status
        x.project_milestonestep += Uint128::new(1); //switch to next milestone step
        
        //-----------check milestone done---------------------
        if x.project_milestonestep >= Uint128::new(x.milestone_states.len() as u128){
            x.project_status = Uint128::new(4); //switch to project done status
        }

        //-------update-------------------------
        PROJECTSTATES.update(deps.storage, project_id.u128().into(), |op| match op {
            None => Err(ContractError::NotRegisteredProject {}),
            Some(mut project) => {
                project.milestone_states = x.milestone_states;
                project.project_milestonestep = x.project_milestonestep;
                project.project_status = x.project_status;
                Ok(project)
            }
        })?;
        return res;
    }
    //-------update-------------------------
    PROJECTSTATES.update(deps.storage, project_id.u128().into(), |op| match op {
        None => Err(ContractError::NotRegisteredProject {}),
        Some(mut project) => {
            project.milestone_states = x.milestone_states;
            project.project_milestonestep = x.project_milestonestep;
            project.project_status = x.project_status;
            Ok(project)
        }
    })?;

    Ok(Response::new()
    .add_attribute("action", "Set milestone vote")
    )
}

pub fn try_setcommunityvote(deps: DepsMut, project_id: Uint128, wallet: String, voted: bool)
    -> Result<Response, ContractError>
{
    let mut x:ProjectState = PROJECTSTATES.load(deps.storage, project_id.u128().into())?;
    
    //-------check project status-------------------
    if x.project_status != Uint128::new(1) { //only community voting status
        return Err(ContractError::NotCorrectStatus{status:x.project_status});
    }
    let wallet = deps.api.addr_validate(&wallet).unwrap();
    let index = x.community_votes.iter().position(|x|x.wallet == wallet).unwrap();

    //------set vot status--------------------
    x.community_votes[index].voted = voted;

    //------check all voted-----------------
    let mut voted_count: u128 = 0;
    for vote in x.community_votes.clone(){
        if vote.voted{
            voted_count += 1;
        }
    }
    if voted_count >= (x.community_votes.len()/2) as u128 {
        x.project_status = Uint128::new(2); //switch to fundrasing status
    }

    //-------update-------------------------
    PROJECTSTATES.update(deps.storage, project_id.u128().into(), |op| match op {
        None => Err(ContractError::NotRegisteredProject {}),
        Some(mut project) => {
            project.project_status = x.project_status;
            project.community_votes = x.community_votes;
            Ok(project)
        }
    })?;

    Ok(Response::new()
    .add_attribute("action", "Set community vote")
    )
}

pub fn try_wefundapprove(deps: DepsMut, info:MessageInfo, project_id: Uint128, deadline:Uint128)
    ->Result<Response, ContractError>
{
    //-----------check owner--------------------------
    let config = CONFIG.load(deps.storage).unwrap();
    if info.sender != config.owner{
        return Err(ContractError::Unauthorized{});
    }

    let mut x:ProjectState = PROJECTSTATES.load(deps.storage, project_id.u128().into())?;
    
    //-------check project status-------------------
    if x.project_status != Uint128::zero() { //only wefund approve status
        return Err(ContractError::NotCorrectStatus{status:x.project_status});
    }
    x.project_status = Uint128::new(1); //switch to community voting status

    PROJECTSTATES.update(deps.storage, project_id.u128().into(), |op| match op {
        None => Err(ContractError::NotRegisteredProject {}),
        Some(mut project) => {
            project.project_status = x.project_status;
            project.community_vote_deadline = deadline;
            Ok(project)
        }
    })?;

    Ok(Response::new()
    .add_attribute("action", "Wefund Approve")
    )
}

pub fn try_removecommunitymember(deps:DepsMut, wallet: String)
    -> Result<Response, ContractError>
{
    let wallet = deps.api.addr_validate(&wallet).unwrap();

    let mut community = COMMUNITY.load(deps.storage).unwrap();
    let res = community.iter().find(|&x| x == &wallet);
    if res == None {
        return Err(ContractError::NotRegisteredCommunity{});
    }

    community.retain(|x|x != &wallet);
    COMMUNITY.save(deps.storage, &community)?;

    Ok(Response::new()
    .add_attribute("action", "remove community member")
    )
}

pub fn try_addcommunitymember(deps:DepsMut, wallet: String)
    -> Result<Response, ContractError>
{
    let wallet = deps.api.addr_validate(&wallet).unwrap();

    let mut community = COMMUNITY.load(deps.storage).unwrap();
    let res = community.iter().find(|&x| x == &wallet);
    if res != None {
        return Err(ContractError::AlreadyRegisteredCommunity{});
    }

    community.push(wallet);
    COMMUNITY.save(deps.storage, &community)?;

    Ok(Response::new()
    .add_attribute("action", "add community member")
    )
}
pub fn try_transferallcoins(deps:DepsMut, _env:Env, info:MessageInfo, wallet:String)
    -> Result<Response, ContractError>
{
    //-----------check owner--------------------------
    let config = CONFIG.load(deps.storage).unwrap();
    if info.sender != config.owner{
        return Err(ContractError::Unauthorized{});
    }
    //--------get all native coins and ust - 4 ----------------------
    let balance: AllBalanceResponse = deps.querier.query(
        &QueryRequest::Bank(BankQuery::AllBalances {
            address: _env.contract.address.to_string(),
        }
    ))?;

    let mut nativecoins:Vec<Coin> = Vec::new();
    for mut x in balance.amount
    {
        if x.denom == "uusd" {
            if x.amount.u128() < 4000000 {
                return Err(ContractError::NeedCoin{});
            }
            x.amount = Uint128::new(x.amount.u128() - 4000000);
        }
        nativecoins.push(x);
    }

    let bank_native = BankMsg::Send { 
        to_address: wallet.clone(),
        amount: nativecoins,
    };

    //--------transfer all aust--------------------------------
    let config = CONFIG.load(deps.storage).unwrap();

    let aust_balance: Cw20BalanceResponse = deps.querier.query_wasm_smart(
        config.aust_token.clone(),
        &Cw20QueryMsg::Balance{
            address: _env.contract.address.to_string(),
        }
    )?;
    let bank_aust = WasmMsg::Execute {
        contract_addr: String::from(config.aust_token),
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: wallet,
            amount: aust_balance.balance,
        }).unwrap(),
        funds: Vec::new()
    };

    Ok(Response::new()
    .add_messages(vec![
        CosmosMsg::Bank(bank_native),
        CosmosMsg::Wasm(bank_aust)])
    .add_attribute("action", "trasnfer all coins")
    )
}
pub fn try_removeproject(deps:DepsMut, info:MessageInfo, project_id:Uint128)
    -> Result<Response, ContractError>
{
    //-----------check owner--------------------------
    let config = CONFIG.load(deps.storage).unwrap();
    if info.sender != config.owner
    {
        return Err(ContractError::Unauthorized{});
    }

    return remove_project(deps, project_id);
}
pub fn remove_project(deps:DepsMut, _project_id:Uint128)
    ->Result<Response, ContractError>
{
    let res = PROJECTSTATES.may_load(deps.storage, _project_id.u128().into());
    if res == Ok(None) {
        return Err(ContractError::NotRegisteredProject {});
    }
    PROJECTSTATES.remove(deps.storage, U128Key::new(_project_id.u128()));
    Ok(Response::new())
}
pub fn try_setconfig(deps:DepsMut, info:MessageInfo,
    admin:Option<String>, 
    wefund:Option<String>, 
    anchor_market:Option<String>, 
    aust_token:Option<String>
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

    config.wefund = wefund
        .and_then(|s| deps.api.addr_validate(s.as_str()).ok()) 
        .unwrap_or(config.wefund);

    config.anchor_market = anchor_market
        .and_then(|s| deps.api.addr_validate(s.as_str()).ok()) 
        .unwrap_or(config.anchor_market);

    config.aust_token = aust_token
        .and_then(|s| deps.api.addr_validate(s.as_str()).ok()) 
        .unwrap_or(config.aust_token);

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "SetConfig"))                                
}
pub fn try_completeproject(
    deps: DepsMut,
    _env: Env,
    _project_id: Uint128
) -> Result<Response, ContractError>
{
    //--------Get project info----------------------------
    let x:ProjectState = PROJECTSTATES.load(deps.storage, _project_id.u128().into())?;

    //--------Checking project status-------------------------
    if x.project_status != Uint128::new(3){//only releasing status
        return Err(ContractError::NotCorrectStatus{status: x.project_status});
    }

    //---------calc hope to release amount---------------------------
    let mut release_amount: u128 = x.communitybacked_amount.u128() + x.backerbacked_amount.u128();
    
    for i in 0..(x.project_milestonestep.u128() as usize){
        release_amount -= x.milestone_states[i].milestone_amount.u128() * UST;
    }

    //---------calc total deposited to anchor----------------
    //----------map to vec-----------------------
    let all: StdResult<Vec<_>> = PROJECTSTATES.range(deps.storage, None, None, 
        cosmwasm_std::Order::Ascending).collect();
    let all = all.unwrap();

    let mut total_deposited = 0;
    for x in all{
        let prj = x.1;
        total_deposited += prj.communitybacked_amount.u128() + prj.backerbacked_amount.u128();

        for i in 0..(prj.project_milestonestep.u128() as usize){
            total_deposited -= prj.milestone_states[i].milestone_amount.u128() * UST;
        }
    }
    //----------load config and read aust token address-----------------
    let config = CONFIG.load(deps.storage).unwrap();
    
    //--------get aust balance---------------------
    let aust_balance: Cw20BalanceResponse = deps.querier.query_wasm_smart(
        config.aust_token.clone(),
        &Cw20QueryMsg::Balance{
            address: _env.contract.address.to_string(),
        }
    )?;

    //----------calc declaim aust amount---aust*(release/total)-----------
    let mut estimate_exchange_rate = total_deposited * UST/aust_balance.balance.u128();

    //--------get exchange rate between ust and aust ---------------------
    let epoch: EpochStateResponse = deps.querier.query_wasm_smart(
        config.anchor_market.to_string(),
        &AnchorQuery::EpochState{
            block_height: None,
            distributed_interest: None,
        }
    )?;
    let epoch_exchange_rate = convert_str_int(epoch.exchange_rate.to_string());
    
    if estimate_exchange_rate < epoch_exchange_rate{
        estimate_exchange_rate = epoch_exchange_rate;
    }

    let withdraw_amount = release_amount * UST / estimate_exchange_rate;
    let release_amount = withdraw_amount * epoch_exchange_rate / UST;

    //----ask aust_token for transfer to anchor martket and execute redeem_stable ----------
    let withdraw = WasmMsg::Execute {
        contract_addr: String::from(config.aust_token),
        msg: to_binary(&Cw20ExecuteMsg::Send {
            contract: config.anchor_market.to_string(),
            msg: to_binary(&Cw20HookMsg::RedeemStable{}).unwrap(), //redeem_stable{}
            amount: Uint128::new(withdraw_amount)
        }).unwrap(),
        funds: Vec::new()
    };

    //---------send to creator wallet-------------
    let ust_release = Coin::new(release_amount, "uusd");
    let send2_creator = BankMsg::Send { 
        to_address: x.creator_wallet.to_string(),
        amount: vec![ust_release] 
    };

    Ok(Response::new()
    .add_messages(vec![
        CosmosMsg::Wasm(withdraw),
        CosmosMsg::Bank(send2_creator)])
    .add_attribute("action", "complete project")
    .add_attribute("epoch_exchange_rate", epoch.exchange_rate.to_string())
    )
}
pub fn try_failproject(
    deps: DepsMut,
    _env: Env,
    _project_id: Uint128
) -> Result<Response, ContractError>
{
    //--------Get project info----------------------------
    let x:ProjectState = PROJECTSTATES.load(deps.storage, _project_id.u128().into())?;

    //--------Checking project status-------------------------
    if x.project_status != Uint128::new(3){//only releasing status
        return Err(ContractError::NotCorrectStatus{status: x.project_status});
    }

    //---------calc hope to release amount---------------------------
    let mut release_amount: u128 = x.communitybacked_amount.u128() + x.backerbacked_amount.u128();
    
    for i in 0..(x.project_milestonestep.u128() as usize){
        release_amount -= x.milestone_states[i].milestone_amount.u128() * UST;
    }

    //---------calc total deposited to anchor----------------
    //----------map to vec-----------------------
    let all: StdResult<Vec<_>> = PROJECTSTATES.range(deps.storage, None, None, 
        cosmwasm_std::Order::Ascending).collect();
    let all = all.unwrap();

    let mut total_deposited = 0;
    for x in all{
        let prj = x.1;
        total_deposited += prj.communitybacked_amount.u128() + prj.backerbacked_amount.u128();

        for i in 0..(prj.project_milestonestep.u128() as usize){
            total_deposited -= prj.milestone_states[i].milestone_amount.u128() * UST;
        }
    }
    //----------load config and read aust token address-----------------
    let config = CONFIG.load(deps.storage).unwrap();
    
    //--------get aust balance---------------------
    let aust_balance: Cw20BalanceResponse = deps.querier.query_wasm_smart(
        config.aust_token.clone(),
        &Cw20QueryMsg::Balance{
            address: _env.contract.address.to_string(),
        }
    )?;

    //----------calc declaim aust amount---aust*(release/total)-----------
    let mut estimate_exchange_rate = total_deposited * UST/aust_balance.balance.u128();

    //--------get exchange rate between ust and aust ---------------------
    let epoch: EpochStateResponse = deps.querier.query_wasm_smart(
        config.anchor_market.to_string(),
        &AnchorQuery::EpochState{
            block_height: None,
            distributed_interest: None,
        }
    )?;
    let epoch_exchange_rate = convert_str_int(epoch.exchange_rate.to_string());
    
    if estimate_exchange_rate < epoch_exchange_rate{
        estimate_exchange_rate = epoch_exchange_rate;
    }

    let withdraw_amount = release_amount * UST / estimate_exchange_rate;
    let release_amount = withdraw_amount * epoch_exchange_rate / UST;

    let mut msg= Vec::new();

    //----ask aust_token for transfer to anchor martket and execute redeem_stable ----------
    let withdraw = WasmMsg::Execute {
        contract_addr: String::from(config.aust_token),
        msg: to_binary(&Cw20ExecuteMsg::Send {
            contract: config.anchor_market.to_string(),
            msg: to_binary(&Cw20HookMsg::RedeemStable{}).unwrap(), //redeem_stable{}
            amount: Uint128::new(withdraw_amount)
        }).unwrap(),
        funds: Vec::new()
    };

    msg.push(CosmosMsg::Wasm(withdraw));

    //---------send to backer wallet-------------
    for backer in x.backer_states{
        let mut backed_ust = backer.ust_amount.clone(); 

        //---while mistone releasing, suddenly failed, distribute with %
        backed_ust.amount = Uint128::new(backer.ust_amount.amount.u128() * release_amount
            /(x.communitybacked_amount.u128() + x.backerbacked_amount.u128()));

        let send2_backer = BankMsg::Send { 
            to_address: backer.backer_wallet.to_string(),
            amount: vec![backed_ust] 
        };
        msg.push(CosmosMsg::Bank(send2_backer));
    }
    
    //-----update project state to FAIL----------------------------
    PROJECTSTATES.update(deps.storage, _project_id.u128().into(), |op| match op {
        None => Err(ContractError::NotRegisteredProject {}),
        Some(mut project) => {
            project.project_status = Uint128::new(5); //fail
            Ok(project)
        }
    })?;

    Ok(Response::new()
    .add_messages(msg)
    .add_attribute("action", "project failed")
    )
}

pub fn try_addproject(
    deps:DepsMut,
    _info: MessageInfo,
    _project_company: String,
    _project_title: String,
    _project_description: String,
    _project_ecosystem: String,
    _project_createddate: String,
    _project_saft: String,
    _project_logo: String,
    _project_whitepaper: String,
    _project_website: String,
    _project_email: String,
    _creator_wallet: String,
    _project_collected: Uint128,
    _project_milestones: Vec<Milestone>,
    _project_teammembers: Vec<TeamMember>,
) -> Result<Response, ContractError> 
{
    let community = COMMUNITY.load(deps.storage).unwrap();
    let mut community_votes = Vec::new();
    for x in community{
        community_votes.push(
            Vote{wallet:x, voted: false}
        );
    }

    let new_project:ProjectState = ProjectState{
        project_company: _project_company,
        project_title: _project_title,
        project_description: _project_description,
        project_ecosystem: _project_ecosystem,
        project_createddate: _project_createddate,
        project_saft: _project_saft,
        project_logo: _project_logo,
        project_whitepaper: _project_whitepaper,
        project_website: _project_website,
        project_email: _project_email,

        project_id: Uint128::zero(), //auto increment
        creator_wallet: deps.api.addr_validate(&_creator_wallet).unwrap(),
        project_collected: _project_collected,
        project_status: Uint128::zero(), //wefund voting status

        backerbacked_amount: Uint128::zero(),
        communitybacked_amount: Uint128::zero(),

        backer_states: Vec::new(),
        communitybacker_states: Vec::new(),

        milestone_states: _project_milestones,
        project_milestonestep: Uint128::zero(), //first milestonestep

        community_votes: community_votes,
        community_vote_deadline: Uint128::zero(),

        teammember_states: _project_teammembers,
    };

    save_projectstate(deps, &new_project)?;
    Ok(Response::new()
        .add_attribute("action", "add project"))
}

pub fn try_back2project(
    deps: DepsMut, 
    info: MessageInfo,
    project_id: Uint128, 
    backer_wallet: String,
    otherchain: String,
    otherchain_wallet: String,
) -> Result<Response, ContractError> 
{
    //-------check project exist-----------------------------------
    let res = PROJECTSTATES.may_load(deps.storage, project_id.u128().into());
    if res == Ok(None) { //not exist
        return Err(ContractError::NotRegisteredProject {});
    }
    //--------Get project info------------------------------------
    let mut x = PROJECTSTATES.load(deps.storage, project_id.u128().into())?;
    if x.project_status != Uint128::new(2){//only fundrasing status
        return Err(ContractError::NotCorrectStatus{status: x.project_status});
    }

    //--------check sufficient back--------------------
    let fee:u128 = 4 * UST;
    if info.funds.is_empty() || info.funds[0].amount.u128() < 6 * UST{
        return Err(ContractError::NeedCoin{});
    }
 
    let fund = info.funds[0].clone();
    let mut fund_real_back = fund.clone();
    let mut fund_wefund = fund.clone();
    //--------calc amount to desposit and to wefund
    if fund.amount.u128() >= 100 * UST{
        fund_real_back.amount = Uint128::new(fund.amount.u128() * 100 / 105);
        fund_wefund.amount = Uint128::new((fund.amount.u128() * 5 / 105) - fee);
    } else {
        fund_real_back.amount = Uint128::new(fund.amount.u128() - 5 * UST);
        fund_wefund.amount = Uint128::new(1 * UST);
    }

    let backer_wallet = deps.api.addr_validate(&backer_wallet).unwrap();

    //--------check community and calc backed amount----------------
    let community = COMMUNITY.load(deps.storage)?;
    let is_community = community.iter().find(|&x| x == &backer_wallet);
    let collected = Uint128::new(x.project_collected.u128() / 2 * UST);

    if is_community != None { //community backer
        if x.communitybacked_amount >= collected{
            return Err(ContractError::AlreadyCollected{});
        }
        x.communitybacked_amount += fund_real_back.amount;
    } else { //only backer
        if x.backerbacked_amount >= collected{
            return Err(ContractError::AlreadyCollected{});
        }
        x.backerbacked_amount += fund_real_back.amount;
    }
    //------push to new backer------------------
    let new_baker:BackerState = BackerState{
        backer_wallet: backer_wallet,
        otherchain: otherchain,
        otherchain_wallet: otherchain_wallet,
        ust_amount: fund_real_back.clone(),
        aust_amount: Coin::new(0, "aust")
    };
    if is_community != None {//community backer
        x.communitybacker_states.push(new_baker);
    } else {
        x.backer_states.push(new_baker);
    }

    //------check needback-----------------
    let mut communitybacker_needback = true;
    let mut backer_needback = true;

    if x.communitybacked_amount  >= collected{
        communitybacker_needback = false;
    }
    if x.backerbacked_amount  >= collected{
        backer_needback = false;
    }

    //---------check collection and switch to releasing status---------
    if communitybacker_needback == false && backer_needback == false{
        x.project_status = Uint128::new(3); //releasing

        //------add milestone votes in every milestone---------------
        let mut milestone_votes = Vec::new();
        for backer in x.backer_states.clone(){
            milestone_votes.push(
                Vote{ wallet: backer.backer_wallet, voted: false }
            );
        }
        //-----add wefund vote------------------
        let config = CONFIG.load(deps.storage)?;
        milestone_votes.push(
            Vote{ wallet: config.owner, voted: true}
        );

        for i in 0..(x.milestone_states.len() as usize){
            x.milestone_states[i].milestone_votes = milestone_votes.clone();
        }
    }

    PROJECTSTATES.update(deps.storage, project_id.u128().into(), |op| match op {
        None => Err(ContractError::NotRegisteredProject {}),
        Some(mut project) => {
            project.project_status = x.project_status;
            project.communitybacked_amount = x.communitybacked_amount;
            project.backerbacked_amount = x.backerbacked_amount;
            project.backer_states = x.backer_states;
            project.communitybacker_states = x.communitybacker_states;
            
            if x.project_status == Uint128::new(3){//only on switching releasing status
                project.milestone_states = x.milestone_states;
            }
            Ok(project)
        }
    })?;

    //----------load config and read anchor market address-----------------
    let config = CONFIG.load(deps.storage).unwrap();
    let anchormarket = config.anchor_market;

    //----------deposite to anchor market------------------------
    let deposite_project = WasmMsg::Execute {
            contract_addr: String::from(anchormarket),
            msg: to_binary(&AnchorMarket::DepositStable {}).unwrap(),
            funds: vec![fund_real_back]
    };

    //---------send to Wefund with 5/105--------------------
    let bank_wefund = BankMsg::Send { 
        to_address: config.wefund.to_string(),
        amount: vec![fund_wefund] 
    };

    Ok(Response::new()
    .add_messages(vec![
        CosmosMsg::Wasm(deposite_project),
        CosmosMsg::Bank(bank_wefund)])
    .add_attribute("action", "back to project")
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBalance{ wallet } => to_binary(&query_balance(deps, _env, wallet)?),
        QueryMsg::GetConfig{ } => to_binary(&query_getconfig(deps)?),
        QueryMsg::GetAllProject{ } => to_binary(&query_allproject(deps)?),
        QueryMsg::GetProject{ project_id } => to_binary(&query_project(deps, project_id)?),
        QueryMsg::GetBacker{ project_id } => to_binary(&query_backer(deps, project_id)?),
        QueryMsg::GetCommunitymembers{ } => to_binary(&query_communitymembers(deps)?),
    }
}

fn query_communitymembers(deps:Deps) -> StdResult<Vec<Addr>>{
    let community = COMMUNITY.load(deps.storage).unwrap();
    Ok(community)
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
        config.aust_token,
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
fn query_allproject(deps:Deps) -> StdResult<Vec<ProjectState>> {
    let all: StdResult<Vec<_>> = PROJECTSTATES.range(deps.storage, None, None, 
        cosmwasm_std::Order::Ascending).collect();
    let all = all.unwrap();

    let mut all_project:Vec<ProjectState> = Vec::new();
    for x in all{
        all_project.push(x.1);
    }
    Ok(all_project)
}
fn query_backer(deps:Deps, id:Uint128) -> StdResult<Vec<BackerState>>{
    let x = PROJECTSTATES.load(deps.storage, id.u128().into())?;
    Ok(x.backer_states)
}
fn query_project(deps:Deps, id:Uint128) -> StdResult<ProjectState>{
    let x = PROJECTSTATES.load(deps.storage, id.u128().into())?;
    
    Ok(x)
}
