use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdResult};

use crate::msg::{QueryMsg, ValueResp};

pub fn query(
    _deps:Deps,
    _env:Env,
    msg:QueryMsg,
) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg{
        Value {  } => to_json_binary(&query::value()),
    }

}
pub mod query {
    use super::*;
    // use crate::msg::ValueResp;

    pub fn value() -> ValueResp{
        // ValueResp{value: 0}
        let resp: ValueResp  = ValueResp{value: 1};
    resp

    }
}