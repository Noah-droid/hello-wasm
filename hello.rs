use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary};
use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;
use rand::thread_rng;

// Instantiate message, currently empty since no initialization logic is needed
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {}

// Execute messages, currently empty since we have no execution logic
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {}

// Query messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // A query variant that triggers returning a random word
    RandomWord {},
}

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    // Instantiation logic, currently does nothing
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> StdResult<Response> {
    // Execution logic, currently does nothing
    Ok(Response::new().add_attribute("method", "execute"))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // Handle RandomWord query by selecting a random word from a predefined list
        QueryMsg::RandomWord {} => {
            let words = vec![
                "Blockchain", "Decentralized", "Smart Contract", "CosmWasm", "Rust", "Xion", "Cryptography"
            ];
            let mut rng = thread_rng();
            let random_word = words.choose(&mut rng).unwrap_or(&"Default");
            to_binary(random_word)
        }
    }
}
