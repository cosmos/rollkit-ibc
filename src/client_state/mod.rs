mod common;
mod da_params;
mod definition;
mod execution;
mod validation;

pub use da_params::*;
pub use definition::*;

use core::str::FromStr;
use ibc_core::host::types::identifiers::ClientType;

pub const ROLLKI_CLIENT_TYPE: &str = "07-rollkit";

/// Returns the `ClientType` for the `rollkit` light client.
pub fn rollkit_client_type() -> ClientType {
    ClientType::from_str(ROLLKI_CLIENT_TYPE).expect("Never fails because it's valid")
}
