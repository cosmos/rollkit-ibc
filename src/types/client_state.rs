use ibc_clients::tendermint::context::ConsensusStateConverter;
use ibc_clients::tendermint::types::ConsensusState;
use ibc_clients::tendermint::client_state::ClientState as TendermintClientState;
use ibc_proto::ibc::lightclients::rollkit::v1::ClientState as RawClientState;
use ibc_core::client::context::client_state::{ClientStateValidation, ClientStateExecution};
use ibc_core::client::context::client_state::ClientStateCommon;
use ibc_core::client::types::error::ClientError;

use ibc_core::client::types::Height;
use ibc_core::client::types::Status;
use ibc_core::commitment_types::commitment::{
    CommitmentPrefix, CommitmentProofBytes, CommitmentRoot,
};
use ibc_core::primitives::proto::{Any, Protobuf};
use ibc_core::host::types::path::Path;
use ibc_core::host::types::identifiers::{ClientType, ClientId};

use crate::types::Error;
use crate::types::da_params::DaParams;
use crate::context::{ValidationContext, ExecutionContext};

pub const ROLLKIT_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.rollkit.v1.ClientState";

/// Defines the `ClientState` type for the Rollkit rollups.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct ClientState {
    pub tendermint_client_state: TendermintClientState,
    pub da_params: DaParams,
}

impl ClientState {
    pub fn new(tendermint_client_state: TendermintClientState, da_params: DaParams) -> Self {
        Self {
            tendermint_client_state,
            da_params,
        }
    }

    pub fn da_client_id(&self) -> &ClientId {
        &self.da_params.client_id
    }

    pub fn da_fraud_period_window(&self) -> u64 {
        self.da_params.fraud_period_window
    }
}

impl Protobuf<RawClientState> for ClientState {}

impl TryFrom<RawClientState> for ClientState {
    type Error = ClientError;

    fn try_from(raw: RawClientState) -> Result<Self, Self::Error> {
        let tendermint_client_state = raw
            .tendermint_client_state
            .ok_or(Error::missing("tendermint_client_state"))?
            .try_into()?;

        let da_params = raw
            .da_params
            .ok_or(Error::missing("da_params"))?
            .try_into()?;

        Ok(Self::new(tendermint_client_state, da_params))
    }
}

impl From<ClientState> for RawClientState {
    fn from(value: ClientState) -> Self {
        Self {
            tendermint_client_state: Some(value.tendermint_client_state.into()),
            da_params: Some(value.da_params.into()),
        }
    }
}

impl Protobuf<Any> for ClientState {}

impl TryFrom<Any> for ClientState {
    type Error = ClientError;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        fn decode_client_state(value: &[u8]) -> Result<ClientState, ClientError> {
            let client_state =
                Protobuf::<RawClientState>::decode(value).map_err(|e| ClientError::Other {
                    description: e.to_string(),
                })?;

            Ok(client_state)
        }

        match raw.type_url.as_str() {
          ROLLKIT_CLIENT_STATE_TYPE_URL => decode_client_state(&raw.value),
            _ => Err(ClientError::UnknownClientStateType {
                client_state_type: raw.type_url,
            }),
        }
    }
}

impl From<ClientState> for Any {
    fn from(client_state: ClientState) -> Self {
        Any {
            type_url: ROLLKIT_CLIENT_STATE_TYPE_URL.to_string(),
            value: Protobuf::<RawClientState>::encode_vec(client_state),
        }
    }
}

impl ClientStateCommon for ClientState {
    fn verify_consensus_state(&self, consensus_state: Any) -> Result<(), ClientError> {
        let tm_consensus_state = ConsensusState::try_from(consensus_state)?;
        if tm_consensus_state.root().is_empty() {
            return Err(ClientError::Other {
                description: "empty commitment root".into(),
            });
        };

        Ok(())
    }

    fn client_type(&self) -> ClientType {
        unimplemented!("client_type")
    }

    fn latest_height(&self) -> Height {
        self.0.latest_height()
    }

    fn validate_proof_height(&self, proof_height: Height) -> Result<(), ClientError> {
        if self.latest_height() < proof_height {
            return Err(ClientError::InvalidProofHeight {
                latest_height: self.latest_height(),
                proof_height,
            });
        }
        unimplemented!("validate_proof_height")
    }

    /// Perform client-specific verifications and check all data in the new
    /// client state to be the same across all valid clients for the new chain.
    ///
    /// You can learn more about how to upgrade IBC-connected SDK chains in
    /// [this](https://ibc.cosmos.network/main/ibc/upgrades/quick-guide.html)
    /// guide
    fn verify_upgrade_client(
        &self,
        _upgraded_client_state: Any,
        _upgraded_consensus_state: Any,
        _proof_upgrade_client: CommitmentProofBytes,
        _proof_upgrade_consensus_state: CommitmentProofBytes,
        _root: &CommitmentRoot,
    ) -> Result<(), ClientError> {
        unimplemented!("verify_upgrade_client")
    }

    fn verify_membership(
        &self,
        _prefix: &CommitmentPrefix,
        _proof: &CommitmentProofBytes,
        _root: &CommitmentRoot,
        _path: Path,
        _value: Vec<u8>,
    ) -> Result<(), ClientError> {
        unimplemented!("verify_membership")
    }

    fn verify_non_membership(
        &self,
        _prefix: &CommitmentPrefix,
        _proof: &CommitmentProofBytes,
        _root: &CommitmentRoot,
        _path: Path,
    ) -> Result<(), ClientError> {
        unimplemented!("verify_non_membership")
    }
}

impl<V> ClientStateValidation<V> for ClientState
where
    V: ValidationContext,
    V::ConsensusStateRef: ConsensusStateConverter,
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
        unimplemented!("check_for_misbehaviour")
    }

    fn status(&self, ctx: &V, client_id: &ClientId) -> Result<Status, ClientError> {
        unimplemented!("status")
    }
}

impl<E> ClientStateExecution<E> for ClientState
where
    E: ExecutionContext,
    E::ClientStateRef: From<ClientState>,
    E::ConsensusStateRef: ConsensusStateConverter,
{
    fn initialise(
        &self,
        ctx: &mut E,
        client_id: &ClientId,
        consensus_state: Any,
    ) -> Result<(), ClientError> {
        unimplemented!("initialise")
    }

    fn update_state(
        &self,
        ctx: &mut E,
        client_id: &ClientId,
        header: Any,
    ) -> Result<Vec<Height>, ClientError> {
        unimplemented!("update_state")
    }

    fn update_state_on_misbehaviour(
        &self,
        ctx: &mut E,
        client_id: &ClientId,
        client_message: Any,
    ) -> Result<(), ClientError> {
        unimplemented!("update_state_on_misbehaviour")
    }

    fn update_state_on_upgrade(
        &self,
        ctx: &mut E,
        client_id: &ClientId,
        upgraded_client_state: Any,
        upgraded_consensus_state: Any,
    ) -> Result<Height, ClientError> {
        unimplemented!("update_state_on_upgrade")
    }

    // fn update_on_recovery(
    //     &self,
    //     ctx: &mut E,
    //     subject_client_id: &ClientId,
    //     substitute_client_state: Any,
    // ) -> Result<(), ClientError> {
    //     unimplemented!("update_on_recovery")
    // }
}