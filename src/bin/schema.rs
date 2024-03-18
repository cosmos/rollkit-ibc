use cosmwasm_schema::write_api;

use rollkit_ibc::msg::{InstantiateMsg, SudoMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        sudo: SudoMsg,
        // query: QueryMsg,
    }
}
