use ibc_client_tendermint::consensus_state::ConsensusState as TendermintConsensusState;
use ibc_core::client::context::client_state::ClientStateValidation;
use ibc_core::client::context::{Convertible, ExtClientValidationContext};
use ibc_core::client::types::Status;
use ibc_core::client::types::error::ClientError;
use ibc_core::primitives::proto::Any;
use ibc_core::host::types::identifiers::ClientId;

use crate::client_state::ClientState;

impl<V> ClientStateValidation<V> for ClientState
where
    V: ExtClientValidationContext,
    V::ConsensusStateRef: Convertible<TendermintConsensusState, ClientError>,
{
    fn verify_client_message(
        &self,
        _ctx: &V,
        _client_id: &ClientId,
        _client_message: Any,
    ) -> Result<(), ClientError> {
        unimplemented!("verify_client_message")
    }

    fn check_for_misbehaviour(
        &self,
        _ctx: &V,
        _client_id: &ClientId,
        _client_message: Any,
    ) -> Result<bool, ClientError> {
        unimplemented!("verify_client_message")
    }

    fn status(&self, _ctx: &V, _client_id: &ClientId) -> Result<Status, ClientError> {
        unimplemented!("verify_client_message")
    }

    fn check_substitute(&self, _ctx: &V, _substitute_client_state: Any) -> Result<(), ClientError> {
        unimplemented!("verify_client_message")
    }
}