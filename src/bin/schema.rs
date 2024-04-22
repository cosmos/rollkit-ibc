use cosmwasm_schema::write_api;

use ibc_client_cw::types::{InstantiateMsg, SudoMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        sudo: SudoMsg,
        //query: QueryMsg, // TODO: should be able to renable after https://github.com/cosmos/ibc-rs/pull/1187 is merged
    }
}
