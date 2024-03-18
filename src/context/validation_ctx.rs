use ibc::clients::tendermint::context::{ConsensusStateConverter, ValidationContext};
use ibc::clients::wasm_types::client_state::ClientState as WasmClientState;
use ibc::clients::wasm_types::consensus_state::ConsensusState as WasmConsensusState;
use ibc::core::client::context::ClientValidationContext;
use ibc::core::client::types::{error::ClientError, Height};
use ibc::core::handler::types::error::ContextError;
use ibc::core::host::types::identifiers::ClientId;
use ibc::core::host::types::path::{ClientConsensusStatePath, ClientStatePath};
use ibc::core::primitives::proto::{Any, Protobuf};
use ibc::core::primitives::Timestamp;

use super::Context;
use crate::helpers::HeightTravel;
use crate::types::AnyCodec;
use crate::types::ClientType;

impl<'a, C: ClientType<'a>> ClientValidationContext for Context<'a, C> {
    type ClientStateRef = C::ClientState;
    type ConsensusStateRef = C::ConsensusState;

    fn client_state(&self, _client_id: &ClientId) -> Result<Self::ClientStateRef, ContextError> {
        let client_state_value = self.retrieve(ClientStatePath::leaf())?;

        let wasm_client_state: WasmClientState =
            Protobuf::<Any>::decode(client_state_value.as_slice()).map_err(|e| {
                ClientError::Other {
                    description: e.to_string(),
                }
            })?;

        let tm_client_state = C::ClientState::decode_thru_any(wasm_client_state.data)?;

        Ok(tm_client_state)
    }

    fn consensus_state(
        &self,
        client_cons_state_path: &ClientConsensusStatePath,
    ) -> Result<Self::ConsensusStateRef, ContextError> {
        let consensus_state_value = self.retrieve(client_cons_state_path.leaf())?;

        let wasm_consensus_state: WasmConsensusState =
            Protobuf::<Any>::decode(consensus_state_value.as_slice()).map_err(|e| {
                ClientError::Other {
                    description: e.to_string(),
                }
            })?;

        let tm_consensus_state = C::ConsensusState::decode_thru_any(wasm_consensus_state.data)?;

        Ok(tm_consensus_state)
    }

    fn client_update_meta(
        &self,
        _client_id: &ClientId,
        height: &Height,
    ) -> Result<(Timestamp, Height), ContextError> {
        let time_key = self.client_update_time_key(height);

        let time_vec = self.retrieve(time_key)?;

        let time = u64::from_be_bytes(time_vec.try_into().expect("invalid timestamp"));

        let timestamp =
            Timestamp::from_nanoseconds(time).map_err(ClientError::InvalidPacketTimestamp)?;

        let height_key = self.client_update_height_key(height);

        let revision_height_vec = self.retrieve(height_key)?;

        let revision_height =
            u64::from_be_bytes(revision_height_vec.try_into().expect("invalid height"));

        let height = Height::new(0, revision_height)?;

        Ok((timestamp, height))
    }
}

impl<'a, C: ClientType<'a>> ValidationContext for Context<'a, C>
where
    <C as ClientType<'a>>::ConsensusState: ConsensusStateConverter,
{
    fn host_timestamp(&self) -> Result<Timestamp, ContextError> {
        let time = self.env().block.time;

        let host_timestamp = Timestamp::from_nanoseconds(time.nanos()).expect("invalid timestamp");

        Ok(host_timestamp)
    }

    fn host_height(&self) -> Result<Height, ContextError> {
        let host_height = Height::new(0, self.env().block.height)?;

        Ok(host_height)
    }

    fn consensus_state_heights(&self, _client_id: &ClientId) -> Result<Vec<Height>, ContextError> {
        let heights = self.get_heights()?;

        Ok(heights)
    }

    fn next_consensus_state(
        &self,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<Option<Self::ConsensusStateRef>, ContextError> {
        let next_height = self.get_adjacent_height(height, HeightTravel::Next)?;

        match next_height {
            Some(h) => {
                let cons_state_path = ClientConsensusStatePath::new(
                    client_id.clone(),
                    h.revision_number(),
                    h.revision_height(),
                );
                self.consensus_state(&cons_state_path).map(Some)
            }
            None => Ok(None),
        }
    }

    fn prev_consensus_state(
        &self,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<Option<Self::ConsensusStateRef>, ContextError> {
        let prev_height = self.get_adjacent_height(height, HeightTravel::Prev)?;

        match prev_height {
            Some(prev_height) => {
                let cons_state_path = ClientConsensusStatePath::new(
                    client_id.clone(),
                    prev_height.revision_number(),
                    prev_height.revision_height(),
                );
                self.consensus_state(&cons_state_path).map(Some)
            }
            None => Ok(None),
        }
    }
}
