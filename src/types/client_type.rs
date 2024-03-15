use ibc::clients::tendermint::types::ClientState as TendermintClientState;
use ibc::core::client::context::client_state::ClientStateExecution;
use ibc::core::client::context::consensus_state::ConsensusState as ConsensusStateTrait;
use ibc::core::client::types::error::ClientError;
use ibc::core::primitives::proto::Any;

use crate::context::Context;
use crate::types::AnyConsensusState;

pub struct RollkitClient;

impl<'a> ClientType<'a> for RollkitClient {
    type ClientState = TendermintClientState;
    type ConsensusState = AnyConsensusState;
}

/// Enables the introduction of custom client and consensus state types tailored
/// for Rollkit light clients.
pub trait ClientType<'a>: Sized {
    type ClientState: ClientStateExecution<Context<'a, Self>> + Clone;
    type ConsensusState: ConsensusStateTrait + Into<Any> + TryFrom<Any, Error = ClientError>;
}
