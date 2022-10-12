use cosmwasm_std::entry_point;
use cosmwasm_std::{Response};
use crate::error::ContractError;

#[entry_point]
pub fn instantiate() -> Result<Response, ContractError> {
    Ok(Response::default())
}