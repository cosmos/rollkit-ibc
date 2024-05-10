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

use crate::client_message::Header;
use crate::client_state::ClientState;

impl<E> ClientStateExecution<E> for ClientState
where
    E: ExtClientExecutionContext,
    E::ClientStateRef: From<ClientState>,
    E::ClientStateMut: From<ibc_client_tendermint::types::ClientState>,
    TendermintConsensusStateType: Convertible<E::ConsensusStateRef>,
    ClientError: From<<TendermintConsensusStateType as TryFrom<E::ConsensusStateRef>>::Error>,
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
        ctx: &mut E,
        client_id: &ClientId,
        header: Any,
    ) -> Result<Vec<Height>, ClientError> {
        update_state(self, ctx, client_id, header)
    }

    fn update_state_on_misbehaviour(
        &self,
        ctx: &mut E,
        client_id: &ClientId,
        client_message: Any,
    ) -> Result<(), ClientError> {
        update_on_misbehaviour(self, ctx, client_id, client_message)
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
        _substitute_consensus_state: Any,
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
    TendermintConsensusStateType: Convertible<E::ConsensusStateRef>,
    ClientError: From<<TendermintConsensusStateType as TryFrom<E::ConsensusStateRef>>::Error>,
{
    let host_timestamp = ExtClientValidationContext::host_timestamp(ctx)?;
    let host_height = ExtClientValidationContext::host_height(ctx)?;

    let tendermint_consensus_state: TendermintConsensusStateType = consensus_state
        .clone()
        .try_into()
        .map_err(|_: ClientError| ClientError::UnknownConsensusStateType {
            consensus_state_type: consensus_state.type_url,
        })?;

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
        tendermint_consensus_state.into(),
    )?;

    ctx.store_update_meta(
        client_id.clone(),
        client_state.tendermint_client_state.inner().latest_height,
        host_timestamp,
        host_height,
    )?;

    Ok(())
}

/// Update the host store with a new client state, pruning old states from the
/// store if need be.
///
/// Note that this function is typically implemented as part of the
/// [`ClientStateExecution`] trait, but has been made a standalone function
/// in order to make the ClientState APIs more flexible.
pub fn update_state<E>(
    client_state: &ClientState,
    ctx: &mut E,
    client_id: &ClientId,
    header: Any,
) -> Result<Vec<Height>, ClientError>
where
    E: ExtClientExecutionContext,
    E::ClientStateRef: From<ClientState>,
    E::ClientStateMut: From<ibc_client_tendermint::types::ClientState>,
    TendermintConsensusStateType: Convertible<E::ConsensusStateRef>,
    ClientError: From<<TendermintConsensusStateType as TryFrom<E::ConsensusStateRef>>::Error>,
{
    let header = Header::try_from(header)?;
    client_state.tendermint_client_state.update_state(
        ctx,
        client_id,
        Any::from(header.tendermint_header),
    )
}

/// Commit a frozen client state, which was frozen as a result of having exhibited
/// misbehaviour, to the store.
///
/// Note that this function is typically implemented as part of the
/// [`ClientStateExecution`] trait, but has been made a standalone function
/// in order to make the ClientState APIs more flexible.
pub fn update_on_misbehaviour<E>(
    client_state: &ClientState,
    ctx: &mut E,
    client_id: &ClientId,
    _client_message: Any,
) -> Result<(), ClientError>
where
    E: ExtClientExecutionContext,
    E::ClientStateRef: From<ClientState>,
{
    // NOTE: frozen height is  set to `Height {revision_height: 0,
    // revision_number: 1}` and it is the same for all misbehaviour. This
    // aligns with the
    // [`ibc-go`](https://github.com/cosmos/ibc-go/blob/0e3f428e66d6fc0fc6b10d2f3c658aaa5000daf7/modules/light-clients/07-tendermint/misbehaviour.go#L18-L19)
    // implementation.
    let frozen_client_state = client_state.clone().with_frozen_height(Height::min(0));

    ctx.store_client_state(
        ClientStatePath::new(client_id.clone()),
        frozen_client_state.into(),
    )?;

    Ok(())
}
