use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};
use msg::QueryMsg;

mod contract;
mod msg;

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    // let resp
    contract::query(deps,env,msg)
    // Ok(Binary::default())
}

#[entry_point]
pub fn execute(_deps: DepsMut, _env: Env,_info:MessageInfo, _msg: Empty) -> StdResult<Response> {
    Ok(Response::new())
}
