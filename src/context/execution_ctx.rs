use ibc::clients::wasm_types::consensus_state::ConsensusState as WasmConsensusState;
use ibc::core::client::context::ClientExecutionContext;
use ibc::core::client::types::Height;
use ibc::core::handler::types::error::ContextError;
use ibc::core::host::types::identifiers::ClientId;
use ibc::core::host::types::path::{iteration_key, ClientConsensusStatePath, ClientStatePath};
use ibc::core::primitives::Timestamp;

use super::Context;
use crate::types::AnyCodec;
use crate::types::ClientType;

impl<'a, C: ClientType<'a>> ClientExecutionContext for Context<'a, C> {
    type ClientStateMut = C::ClientState;

    fn store_client_state(
        &mut self,
        _client_state_path: ClientStatePath,
        client_state: Self::ClientStateMut,
    ) -> Result<(), ContextError> {
        let key = ClientStatePath::leaf().into_bytes();

        let encoded_client_state = self.encode_client_state(client_state)?;

        self.insert(key, encoded_client_state);

        Ok(())
    }

    fn store_consensus_state(
        &mut self,
        consensus_state_path: ClientConsensusStatePath,
        consensus_state: Self::ConsensusStateRef,
    ) -> Result<(), ContextError> {
        let key = consensus_state_path.leaf().into_bytes();

        let encoded_consensus_state = C::ConsensusState::encode_thru_any(consensus_state);

        let wasm_consensus_state = WasmConsensusState {
            data: encoded_consensus_state,
        };

        let encoded_wasm_consensus_state = C::ConsensusState::encode_thru_any(wasm_consensus_state);

        self.insert(key, encoded_wasm_consensus_state);

        Ok(())
    }

    fn delete_consensus_state(
        &mut self,
        consensus_state_path: ClientConsensusStatePath,
    ) -> Result<(), ContextError> {
        self.remove(consensus_state_path.leaf().into_bytes());

        Ok(())
    }

    fn store_update_meta(
        &mut self,
        _client_id: ClientId,
        height: Height,
        host_timestamp: Timestamp,
        host_height: Height,
    ) -> Result<(), ContextError> {
        let time_key = self.client_update_time_key(&height);

        let time_vec: [u8; 8] = host_timestamp.nanoseconds().to_be_bytes();

        self.insert(time_key, time_vec);

        let height_key = self.client_update_height_key(&height);

        let revision_height_vec: [u8; 8] = host_height.revision_height().to_be_bytes();

        self.insert(height_key, revision_height_vec);

        let iteration_key = iteration_key(height.revision_number(), height.revision_height());

        let height_vec = height.to_string().into_bytes();

        self.insert(iteration_key, height_vec);

        Ok(())
    }

    fn delete_update_meta(
        &mut self,
        _client_id: ClientId,
        height: Height,
    ) -> Result<(), ContextError> {
        let time_key = self.client_update_time_key(&height);

        self.remove(time_key);

        let height_key = self.client_update_height_key(&height);

        self.remove(height_key);

        let iteration_key = iteration_key(height.revision_number(), height.revision_height());

        self.remove(iteration_key);

        Ok(())
    }
}
