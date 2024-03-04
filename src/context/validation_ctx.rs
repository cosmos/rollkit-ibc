use std::time::Duration;

use ibc::core::host::ValidationContext as CoreValidationContext;
use ibc::core::host::types::identifiers::{ClientId, ConnectionId, Sequence};
use ibc::core::channel::types::{channel::ChannelEnd, packet::Receipt};
use ibc::core::channel::types::commitment::{AcknowledgementCommitment, PacketCommitment};
use ibc::core::commitment_types::commitment::CommitmentPrefix;
use ibc::core::primitives::{Signer, Timestamp};
use ibc::core::primitives::proto::{Any, Protobuf};
use ibc::core::client::types::{Height, error::ClientError};
use ibc::core::client::context::ClientValidationContext;
use ibc::core::handler::types::error::ContextError;
use ibc::core::connection::types::ConnectionEnd;
use ibc::core::host::types::path::{
    AckPath, ChannelEndPath, ClientConsensusStatePath, ClientStatePath,
    CommitmentPath, ReceiptPath, SeqAckPath, SeqRecvPath, SeqSendPath,
};
use ibc::clients::tendermint::types::TENDERMINT_CLIENT_STATE_TYPE_URL;
use ibc::clients::tendermint::context::{CommonContext, ValidationContext as TendermintValidationContext};
use ibc::clients::tendermint::client_state::ClientState as TendermintClientState;
use ibc::clients::wasm_types::client_state::ClientState as WasmClientState;
use ibc::clients::tendermint::consensus_state::ConsensusState as TendermintConsensusState;
use ibc::clients::wasm_types::consensus_state::ConsensusState as WasmConsensusState;
use ibc_proto::ibc::lightclients::tendermint::v1::ClientState as RawTendermintClientState;

use super::Context;
use crate::helpers::HeightTravel;
use crate::types::AnyConsensusState;

impl CommonContext for Context<'_> {
    type ConversionError = ClientError;
    type AnyConsensusState = AnyConsensusState;

    fn host_timestamp(&self) -> Result<Timestamp, ContextError> {
        CoreValidationContext::host_timestamp(self)
    }

    fn host_height(&self) -> Result<Height, ContextError> {
        CoreValidationContext::host_height(self)
    }

    fn consensus_state(
        &self,
        client_cons_state_path: &ClientConsensusStatePath,
    ) -> Result<Self::AnyConsensusState, ContextError> {
        CoreValidationContext::consensus_state(self, client_cons_state_path)
    }

    fn consensus_state_heights(&self, _client_id: &ClientId) -> Result<Vec<Height>, ContextError> {
        let heights = self.get_heights()?;

        Ok(heights)
    }
}

impl CoreValidationContext for Context<'_> {
    type V = Self;
    type E = Self;
    type AnyConsensusState = AnyConsensusState;
    type AnyClientState = TendermintClientState;

    fn get_client_validation_context(&self) -> &Self::V {
        self
    }

    fn client_state(&self, _client_id: &ClientId) -> Result<Self::AnyClientState, ContextError> {
        let client_state_value = self.retrieve(ClientStatePath::leaf())?;
        
        let wasm_client_state: WasmClientState =
            Protobuf::<Any>::decode(client_state_value.as_slice()).map_err(
                |e| ClientError::Other {
                    description: e.to_string(),
                }
            )?;

        let tm_client_state: Self::AnyClientState = 
            Protobuf::<Any>::decode(&mut wasm_client_state.data.as_slice()).map_err(
                |e| ClientError::Other {
                    description: e.to_string(),
                }
            )?;

        Ok(tm_client_state.into())
    }

    fn decode_client_state(&self, client_state: Any) -> Result<Self::AnyClientState, ContextError> {
        match client_state.type_url.as_str() {
            TENDERMINT_CLIENT_STATE_TYPE_URL => {
                let tendermint_client_state: TendermintClientState = Protobuf::<RawTendermintClientState>::decode(
                    client_state.value.as_slice(),
                )
                .map_err(|e| ClientError::Other {
                    description: e.to_string(),
                })?;

                Ok(tendermint_client_state)
            }
            _ => Err(ClientError::Other {
                description: "Client state type not supported".to_string(),
            }
            .into()),
        }
    }

    fn consensus_state(&self, client_cons_state_path: &ClientConsensusStatePath) -> Result<Self::AnyConsensusState, ContextError> {
        let consensus_state_value = self.retrieve(client_cons_state_path.leaf())?;

        let any_wasm: WasmConsensusState =
            Protobuf::<Any>::decode(consensus_state_value.as_slice()).map_err(|e| {
                ClientError::Other {
                    description: e.to_string(),
                }
            })?;

        let tm_consensus_state: TendermintConsensusState = 
            Protobuf::<Any>::decode(&mut any_wasm.data.as_slice()).map_err(|e| {
                ClientError::Other {
                    description: e.to_string(),
                }
            })?;
        
        Ok(AnyConsensusState::Rollkit(tm_consensus_state.into()))
    }

    fn host_height(&self) -> Result<Height, ContextError> {
        let host_height = Height::new(0, self.env().block.height)?;

        Ok(host_height)
    }

    fn host_timestamp(&self) -> Result<Timestamp, ContextError> {
        let time = self.env().block.time;

        let host_timestamp = Timestamp::from_nanoseconds(time.nanos()).expect("invalid timestamp");

        Ok(host_timestamp)
    }

    fn host_consensus_state(
        &self,
        _height: &Height,
    ) -> Result<Self::AnyConsensusState, ContextError> {
        unimplemented!()
    }

    fn client_counter(&self) -> Result<u64, ContextError> {
        unimplemented!()
    }

    fn connection_end(&self, _conn_id: &ConnectionId) -> Result<ConnectionEnd, ContextError> {
        unimplemented!()
    }

    fn validate_self_client(
        &self,
        _client_state_of_host_on_counterparty: Any,
    ) -> Result<(), ContextError> {
        Ok(())
    }

    fn commitment_prefix(&self) -> CommitmentPrefix {
        unimplemented!()
    }

    fn connection_counter(&self) -> Result<u64, ContextError> {
        unimplemented!()
    }

    fn channel_end(&self, _channel_end_path: &ChannelEndPath) -> Result<ChannelEnd, ContextError> {
        unimplemented!()
    }

    fn get_next_sequence_send(
        &self,
        _seq_send_path: &SeqSendPath,
    ) -> Result<Sequence, ContextError> {
        unimplemented!()
    }

    fn get_next_sequence_recv(
        &self,
        _seq_recv_path: &SeqRecvPath,
    ) -> Result<Sequence, ContextError> {
        unimplemented!()
    }

    fn get_next_sequence_ack(&self, _seq_ack_path: &SeqAckPath) -> Result<Sequence, ContextError> {
        unimplemented!()
    }

    fn get_packet_commitment(
        &self,
        _commitment_path: &CommitmentPath,
    ) -> Result<PacketCommitment, ContextError> {
        unimplemented!()
    }

    fn get_packet_receipt(&self, _receipt_path: &ReceiptPath) -> Result<Receipt, ContextError> {
        unimplemented!()
    }

    fn get_packet_acknowledgement(
        &self,
        _ack_path: &AckPath,
    ) -> Result<AcknowledgementCommitment, ContextError> {
        unimplemented!()
    }

    fn channel_counter(&self) -> Result<u64, ContextError> {
        unimplemented!()
    }

    fn max_expected_time_per_block(&self) -> Duration {
        // This effectively cancels the check on connection block delays.
        Duration::ZERO
    }

    fn validate_message_signer(&self, _signer: &Signer) -> Result<(), ContextError> {
        Ok(())
    }
}

impl TendermintValidationContext for Context<'_> {
    fn next_consensus_state(
        &self,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<Option<Self::AnyConsensusState>, ContextError> {
        let next_height = self.get_adjacent_height(height, HeightTravel::Next)?;

        match next_height {
            Some(h) => {
                let cons_state_path = ClientConsensusStatePath::new(
                    client_id.clone(),
                    h.revision_number(),
                    h.revision_height(),
                );
                CoreValidationContext::consensus_state(self, &cons_state_path).map(Some)
            }
            None => Ok(None),
        }
    }

    fn prev_consensus_state(
        &self,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<Option<Self::AnyConsensusState>, ContextError> {
        let prev_height = self.get_adjacent_height(height, HeightTravel::Prev)?;

        match prev_height {
            Some(prev_height) => {
                let cons_state_path = ClientConsensusStatePath::new(
                    client_id.clone(),
                    prev_height.revision_number(),
                    prev_height.revision_height(),
                );
                CoreValidationContext::consensus_state(self, &cons_state_path).map(Some)
            }
            None => Ok(None),
        }
    }
}

impl ClientValidationContext for Context<'_> {
    fn update_meta(
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