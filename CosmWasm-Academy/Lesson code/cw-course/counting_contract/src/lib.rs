use cosmwasm_std::{
    DepsMut, Env, MessageInfo, Empty, StdResult, Response, entry_point
};

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
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> StdResult<Response> {
    Ok(Response::new())
}

#[entry_point]
pub fn query(
    _deps: DepsMut,
    _env: Env,
    _msg: QueryMsg,
) -> StdResult<Binary> {
    Ok(Response::new())
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
