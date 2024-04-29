use ibc_client_tendermint::types::ConsensusState as TendermintConsensusStateType;
use ibc_core::client::context::client_state::ClientStateExecution;
use ibc_core::client::context::{
    Convertible, ExtClientExecutionContext, ExtClientValidationContext,
};
use ibc_core::client::types::error::ClientError;
use ibc_core::client::types::Height;
use ibc_core::host::types::identifiers::ClientId;
use ibc_core::primitives::proto::Any;
use ibc_core_host::types::path::{ClientConsensusStatePath, ClientStatePath};

use crate::client_state::ClientState;

impl<E> ClientStateExecution<E> for ClientState
where
    E: ExtClientExecutionContext,
    E::ClientStateRef: From<ClientState>,
    E::ConsensusStateRef: Convertible<TendermintConsensusStateType, ClientError>,
{
    fn initialise(
        &self,
        ctx: &mut E,
        client_id: &ClientId,
        consensus_state: Any,
    ) -> Result<(), ClientError> {
        initialise(self, ctx, client_id, consensus_state)
    }

    fn update_state(
        &self,
        _ctx: &mut E,
        _client_id: &ClientId,
        _header: Any,
    ) -> Result<Vec<Height>, ClientError> {
        unimplemented!()
    }

    fn update_state_on_misbehaviour(
        &self,
        _ctx: &mut E,
        _client_id: &ClientId,
        _client_message: Any,
    ) -> Result<(), ClientError> {
        unimplemented!()
    }

    // Commit the new client state and consensus state to the store
    fn update_state_on_upgrade(
        &self,
        _ctx: &mut E,
        _client_id: &ClientId,
        _upgraded_client_state: Any,
        _upgraded_consensus_state: Any,
    ) -> Result<Height, ClientError> {
        unimplemented!()
    }

    fn update_on_recovery(
        &self,
        _ctx: &mut E,
        _subject_client_id: &ClientId,
        _substitute_client_state: Any,
    ) -> Result<(), ClientError> {
        unimplemented!()
    }
}

/// Seed the host store with initial client and consensus states.
///
/// Note that this function is typically implemented as part of the
/// [`ClientStateExecution`] trait, but has been made a standalone function
/// in order to make the ClientState APIs more flexible.
pub fn initialise<E>(
    client_state: &ClientState,
    ctx: &mut E,
    client_id: &ClientId,
    consensus_state: Any,
) -> Result<(), ClientError>
where
    E: ExtClientExecutionContext,
    E::ClientStateRef: From<ClientState>,
    E::ConsensusStateRef: Convertible<TendermintConsensusStateType, ClientError>,
{
    let host_timestamp = ExtClientValidationContext::host_timestamp(ctx)?;
    let host_height = ExtClientValidationContext::host_height(ctx)?;

    let tm_consensus_state = TendermintConsensusStateType::try_from(consensus_state)?;

    ctx.store_client_state(
        ClientStatePath::new(client_id.clone()),
        client_state.clone().into(),
    )?;
    ctx.store_consensus_state(
        ClientConsensusStatePath::new(
            client_id.clone(),
            client_state
                .tendermint_client_state
                .inner()
                .latest_height
                .revision_number(),
            client_state
                .tendermint_client_state
                .inner()
                .latest_height
                .revision_height(),
        ),
        E::ConsensusStateRef::from(tm_consensus_state),
    )?;

    ctx.store_update_meta(
        client_id.clone(),
        client_state.tendermint_client_state.inner().latest_height,
        host_timestamp,
        host_height,
    )?;

    Ok(())
}
