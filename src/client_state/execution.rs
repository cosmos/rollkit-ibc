use ibc_client_tendermint::consensus_state::ConsensusState as TendermintConsensusState;
use ibc_core::client::context::client_state::ClientStateExecution;
use ibc_core::client::context::{Convertible, ExtClientExecutionContext};
use ibc_core::client::types::error::ClientError;
use ibc_core::client::types::Height;
use ibc_core::primitives::proto::Any;
use ibc_core::host::types::identifiers::ClientId;

use crate::client_state::ClientState;

impl<E> ClientStateExecution<E> for ClientState
where
    E: ExtClientExecutionContext,
    E::ClientStateRef: From<ClientState>,
    E::ConsensusStateRef: Convertible<TendermintConsensusState, ClientError>,
{
    fn initialise(
        &self,
        ctx: &mut E,
        client_id: &ClientId,
        consensus_state: Any,
    ) -> Result<(), ClientError> {
        unimplemented!()
    }

    fn update_state(
        &self,
        ctx: &mut E,
        client_id: &ClientId,
        header: Any,
    ) -> Result<Vec<Height>, ClientError> {
        unimplemented!()
    }

    fn update_state_on_misbehaviour(
        &self,
        ctx: &mut E,
        client_id: &ClientId,
        _client_message: Any,
    ) -> Result<(), ClientError> {
        unimplemented!()
    }

    // Commit the new client state and consensus state to the store
    fn update_state_on_upgrade(
        &self,
        ctx: &mut E,
        client_id: &ClientId,
        upgraded_client_state: Any,
        upgraded_consensus_state: Any,
    ) -> Result<Height, ClientError> {
        unimplemented!()
    }

    fn update_on_recovery(
        &self,
        ctx: &mut E,
        subject_client_id: &ClientId,
        substitute_client_state: Any,
    ) -> Result<(), ClientError> {
        unimplemented!()
    }
}