use ibc::clients::tendermint::consensus_state::ConsensusState as TendermintConsensusState;
use ibc::clients::tendermint::types::TENDERMINT_CONSENSUS_STATE_TYPE_URL;
use ibc::core::client::types::error::ClientError;
//use ibc::core::derive::ConsensusState as ConsensusStateDerive;
use ibc::core::primitives::proto::Any;

#[derive(Clone, Debug)]
pub enum AnyConsensusState {
    Rollkit(TendermintConsensusState),
}

impl From<TendermintConsensusState> for AnyConsensusState {
    fn from(value: TendermintConsensusState) -> Self {
        AnyConsensusState::Rollkit(value.into())
    }
}

impl TryFrom<AnyConsensusState> for TendermintConsensusState {
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
