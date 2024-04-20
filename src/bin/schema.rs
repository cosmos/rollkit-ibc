use cosmwasm_schema::write_api;

use ibc_client_cw::types::{InstantiateMsg, SudoMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        sudo: SudoMsg,
        // query: QueryMsg,
    }
}
