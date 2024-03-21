use ibc_clients::tendermint::consensus_state::ConsensusState as TendermintConsensusState;
use ibc_clients::tendermint::types::{ConsensusState, TENDERMINT_CONSENSUS_STATE_TYPE_URL};
use ibc_core::client::types::error::ClientError;
use ibc_core::primitives::proto::Any;
use ibc_derive::IbcCoreConsensusState as ConsensusStateDerive;

#[derive(Clone, Debug, ConsensusStateDerive)]
pub enum AnyConsensusState {
    Rollkit(TendermintConsensusState),
}

impl From<ConsensusState> for AnyConsensusState {
    fn from(value: ConsensusState) -> Self {
        AnyConsensusState::Rollkit(value.into())
    }
}

impl TryFrom<AnyConsensusState> for ConsensusState {
    type Error = ClientError;

    fn try_from(value: AnyConsensusState) -> Result<Self, Self::Error> {
        match value {
            AnyConsensusState::Rollkit(state) => Ok(state.inner().clone()),
        }
    }
}

impl From<AnyConsensusState> for Any {
    fn from(value: AnyConsensusState) -> Self {
        match value {
            AnyConsensusState::Rollkit(cs) => cs.into(),
        }
    }
}

impl TryFrom<Any> for AnyConsensusState {
    type Error = ClientError;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        match raw.type_url.as_str() {
            TENDERMINT_CONSENSUS_STATE_TYPE_URL => {
                let cs = TendermintConsensusState::try_from(raw)?;
                Ok(AnyConsensusState::Rollkit(cs))
            }
            _ => Err(ClientError::UnknownConsensusStateType {
                consensus_state_type: raw.type_url,
            }),
        }
    }
}
