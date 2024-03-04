use ibc::core::client::types::error::ClientError;
use ibc::core::derive::ConsensusState as ConsensusStateDerive;
use ibc::core::primitives::proto::{Any, Protobuf};
use ibc::clients::tendermint::consensus_state::ConsensusState as TendermintConsensusState;
use ibc::clients::tendermint::types::TENDERMINT_CONSENSUS_STATE_TYPE_URL;

#[derive(Clone, Debug, derive_more::From, ConsensusStateDerive)]
pub enum AnyConsensusState {
    Rollkit(TendermintConsensusState),
}

impl TryFrom<AnyConsensusState> for TendermintConsensusState {
    type Error = ClientError;

    fn try_from(value: AnyConsensusState) -> Result<Self, Self::Error> {
        match value {
            AnyConsensusState::Rollkit(state) => Ok(state),
        }
    }
}

impl From<AnyConsensusState> for Any {
    fn from(value: AnyConsensusState) -> Self {
        match value {
            AnyConsensusState::Rollkit(cs) => Any {
                type_url: TENDERMINT_CONSENSUS_STATE_TYPE_URL.to_string(),
                value: Protobuf::<Any>::encode_vec(cs),
            },
        }
    }
}
