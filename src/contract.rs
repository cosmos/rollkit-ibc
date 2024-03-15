#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::context::Context;
use crate::error::ContractError;
use crate::msg::{InstantiateMsg, QueryMsg, SudoMsg};
use crate::types::RollkitClient;

pub type RollkitContext<'a> = Context<'a, RollkitClient>;

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:rollkit-ibc";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let mut ctx = RollkitContext::new_mut(deps, env)?;

    let data = ctx.instantiate(msg)?;

    Ok(Response::default().set_data(data))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: SudoMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
