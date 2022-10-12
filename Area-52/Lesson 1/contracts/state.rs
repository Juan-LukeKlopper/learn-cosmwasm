use serde::{Deserialize, Serialize};
use universe::species::{SapienceScale, Sapient};
use cosmwasm_std::{Addr};

#[derive(Serialize, Deserialize)]
pub struct State {
    pub owner: Addr,
    pub planet_name: String,
    pub planet_sapients: Vec<Sapient>,
    pub minimum_sapience: SapienceScale,
}