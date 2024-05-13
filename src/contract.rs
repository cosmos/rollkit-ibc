#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response};

use crate::types::RollkitClient;
use ibc_client_cw::context::Context;
use ibc_client_cw::types::ContractError;
use ibc_client_cw::types::{InstantiateMsg, QueryMsg, SudoMsg};

pub type RollkitContext<'a> = Context<'a, RollkitClient>;

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:rollkit-ibc";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<'_>,
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
    deps: DepsMut<'_>,
    env: Env,
    _info: MessageInfo,
    msg: SudoMsg,
) -> Result<Response, ContractError> {
    let mut ctx = RollkitContext::new_mut(deps, env)?;

    let data = ctx.sudo(msg)?;

    Ok(Response::default().set_data(data))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<'_>, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    let ctx = RollkitContext::new_ref(deps, env)?;

    ctx.query(msg)
}

#[cfg(test)]
mod tests {}
