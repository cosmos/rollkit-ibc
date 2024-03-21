use alloc::string::String;
use std::error::Error as StdError;

use cosmwasm_std::StdError as CwError;
use derive_more::{Display, From};
use ibc_core::client::types::error::ClientError;
use ibc_core::commitment_types::error::CommitmentError;
use ibc_core::handler::types::error::ContextError;
use ibc_core::host::types::error::IdentifierError;
use ibc_core::host::types::path::PathError;

use crate::types::Error;

#[derive(From, Display, Debug)]
pub enum ContractError {
    Std(CwError),
    #[display(fmt = "invalid message: {_0}")]
    InvalidMsg(String),
    #[display(fmt = "IBC validation/execution context error: {_0}")]
    Context(ContextError),
    #[display(fmt = "IBC 02-client error: {_0}")]
    Ics02ClientError(ClientError),
    #[display(fmt = "IBC commitment error: {_0}")]
    Commitment(CommitmentError),
    #[display(fmt = "IBC identifier error: {_0}")]
    Identifier(IdentifierError),
    #[display(fmt = "IBC path error: {_0}")]
    Path(PathError),
    #[display(fmt = "Proto decode error: {_0}")]
    ProtoDecode(prost::DecodeError),
}

impl StdError for ContractError {}

impl From<ContractError> for CwError {
    fn from(err: ContractError) -> CwError {
        CwError::generic_err(err.to_string())
    }
}

impl From<Error> for ContractError {
    fn from(err: Error) -> ContractError {
        ContractError::Std(CwError::generic_err(err.origin))
    }
}
