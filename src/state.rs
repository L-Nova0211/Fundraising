use cosmwasm_std::{Addr};
use cw_storage_plus::{Item, Map, U128Key};
use crate::msg::{ProjectInfo};

pub const OWNER: Item<Addr> = Item::new("owner");
pub const VESTING_ADDR: Item<Addr> = Item::new("vesting_address");
pub const PROJECT_INFOS:Map<U128Key, ProjectInfo> = Map::new("project_infos");
