use ibc_client_tendermint::client_state::ClientState as TendermintClientState;
use ibc_core::client::types::error::ClientError;
use ibc_proto::ibc::lightclients::rollkit::v1::ClientState as RawClientState;

use ibc_core::host::types::identifiers::ClientId;
use ibc_core::primitives::proto::{Any, Protobuf};

use crate::client_state::DaParams;
use crate::types::Error;

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
