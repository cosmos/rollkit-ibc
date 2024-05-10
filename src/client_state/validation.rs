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
    TendermintConsensusStateType: Convertible<V::ConsensusStateRef>,
    ClientError: From<<TendermintConsensusStateType as TryFrom<V::ConsensusStateRef>>::Error>
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
        ctx: &V,
        client_id: &ClientId,
        client_message: Any,
    ) -> Result<bool, ClientError> {
        check_for_misbehaviour(self, ctx, client_id, client_message)
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
    TendermintConsensusStateType: Convertible<V::ConsensusStateRef>,
    ClientError: From<<TendermintConsensusStateType as TryFrom<V::ConsensusStateRef>>::Error>
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


/// Check for misbehaviour on the client state as part of the client state
/// validation process.
///
/// Note that this function is typically implemented as part of the
/// [`ClientStateValidation`] trait, but has been made a standalone function
/// in order to make the ClientState APIs more flexible.
///
/// This method covers the following cases:
///
/// 1 - fork:
/// Assumes at least one consensus state before the fork point exists. Let
/// existing consensus states on chain B be: [Sn,.., Sf, Sf-1, S0] with
/// `Sf-1` being the most recent state before fork. Chain A is queried for a
/// header `Hf'` at `Sf.height` and if it is different than the `Hf` in the
/// event for the client update (the one that has generated `Sf` on chain),
/// then the two headers are included in the evidence and submitted. Note
/// that in this case the headers are different but have the same height.
///
/// 2 - BFT time violation for unavailable header (a.k.a. Future Lunatic
/// Attack or FLA):
/// Some header with a height that is higher than the latest height on A has
/// been accepted and a consensus state was created on B. Note that this
/// implies that the timestamp of this header must be within the
/// `clock_drift` of the client. Assume the client on B has been updated
/// with `h2`(not present on/ produced by chain A) and it has a timestamp of
/// `t2` that is at most `clock_drift` in the future. Then the latest header
/// from A is fetched, let it be `h1`, with a timestamp of `t1`. If `t1 >=
/// t2` then evidence of misbehavior is submitted to A.
///
/// 3 - BFT time violation for existing headers:
/// Ensure that consensus state times are monotonically increasing with
/// height.
pub fn check_for_misbehaviour<V>(
    client_state: &ClientState,
    ctx: &V,
    client_id: &ClientId,
    client_message: Any,
) -> Result<bool, ClientError>
where
    V: ExtClientValidationContext,
    TendermintConsensusStateType: Convertible<V::ConsensusStateRef>,
    ClientError: From<<TendermintConsensusStateType as TryFrom<V::ConsensusStateRef>>::Error>
{
    return client_state.tendermint_client_state.check_for_misbehaviour(ctx, client_id, client_message)
}