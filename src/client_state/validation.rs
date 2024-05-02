use ibc_client_tendermint::types::ConsensusState as TendermintConsensusStateType;
use ibc_client_tendermint::types::TENDERMINT_MISBEHAVIOUR_TYPE_URL;
use ibc_core::client::context::client_state::ClientStateValidation;
use ibc_core::client::context::{Convertible, ExtClientValidationContext};
use ibc_core::client::types::error::ClientError;
use ibc_core::client::types::Status;
use ibc_core::host::types::identifiers::ClientId;
use ibc_core::primitives::proto::Any;
use tendermint_light_client_verifier::{ProdVerifier, Verifier};

use crate::client_message::Header;
use crate::client_message::ROLLKIT_HEADER_TYPE_URL;
use crate::client_state::ClientState;

impl<V> ClientStateValidation<V> for ClientState
where
    V: ExtClientValidationContext,
    V::ConsensusStateRef: Convertible<TendermintConsensusStateType, ClientError>,
{
    fn verify_client_message(
        &self,
        ctx: &V,
        client_id: &ClientId,
        client_message: Any,
    ) -> Result<(), ClientError> {
        verify_client_message::<V>(
            self,
            ctx,
            client_id,
            client_message,
            &ProdVerifier::default(),
        )
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

/// Verify the client message as part of the client state validation process.
///
/// Note that this function is typically implemented as part of the
/// [`ClientStateValidation`] trait, but has been made a standalone function in
/// order to make the ClientState APIs more flexible. It mostly adheres to the
/// same signature as the `ClientStateValidation::verify_client_message`
/// function, except for an additional `verifier` parameter that allows users
/// who require custom verification logic to easily pass in their own verifier
/// implementation.
pub fn verify_client_message<V>(
    client_state: &ClientState,
    ctx: &V,
    client_id: &ClientId,
    client_message: Any,
    _verifier: &impl Verifier,
) -> Result<(), ClientError>
where
    V: ExtClientValidationContext,
    V::ConsensusStateRef: Convertible<TendermintConsensusStateType, ClientError>,
{
    match client_message.type_url.as_str() {
        ROLLKIT_HEADER_TYPE_URL => {
            let header = Header::try_from(client_message)?;
            client_state.tendermint_client_state.verify_client_message(
                ctx,
                client_id,
                Any::from(header.tendermint_header),
            )
        }
        TENDERMINT_MISBEHAVIOUR_TYPE_URL => client_state
            .tendermint_client_state
            .verify_client_message(ctx, client_id, client_message),
        _ => Err(ClientError::InvalidUpdateClientMessage),
    }
}
