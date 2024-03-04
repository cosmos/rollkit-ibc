use ibc::core::channel::types::channel::ChannelEnd;
use ibc::core::channel::types::commitment::{AcknowledgementCommitment, PacketCommitment};
use ibc::core::channel::types::packet::Receipt;
use ibc::core::client::types::Height;
use ibc::core::client::context::ClientExecutionContext;
use ibc::core::connection::types::ConnectionEnd;
use ibc::core::handler::types::error::ContextError;
use ibc::core::handler::types::events::IbcEvent;
use ibc::core::host::types::identifiers::{ClientId, ConnectionId, Sequence};
use ibc::core::host::types::path::{
    iteration_key,
    AckPath, ChannelEndPath, ClientConnectionPath, ClientConsensusStatePath, ClientStatePath,
    CommitmentPath, ConnectionPath, ReceiptPath, SeqAckPath, SeqRecvPath, SeqSendPath,
};
use ibc::core::host::{ExecutionContext, ValidationContext};
use ibc::core::primitives::{Timestamp, proto::Any};
use ibc::clients::wasm_types::consensus_state::ConsensusState as WasmConsensusState;
use prost::Message;

use super::Context;

impl ExecutionContext for Context<'_> {
    fn get_client_execution_context(&mut self) -> &mut Self::E {
        todo!()
    }

    fn increase_client_counter(&mut self) -> Result<(), ContextError> {
        todo!()
    }

    fn store_connection(
        &mut self,
        _connection_path: &ConnectionPath,
        _connection_end: ConnectionEnd,
    ) -> Result<(), ContextError> {
        todo!()
    }

    fn store_connection_to_client(
        &mut self,
        _client_connection_path: &ClientConnectionPath,
        _conn_id: ConnectionId,
    ) -> Result<(), ContextError> {
        todo!()
    }

    fn increase_connection_counter(&mut self) -> Result<(), ContextError> {
        todo!()
    }

    fn store_packet_commitment(
        &mut self,
        _commitment_path: &CommitmentPath,
        _commitment: PacketCommitment,
    ) -> Result<(), ContextError> {
        todo!()
    }

    fn delete_packet_commitment(
        &mut self,
        _commitment_path: &CommitmentPath,
    ) -> Result<(), ContextError> {
        todo!()
    }

    fn store_packet_receipt(
        &mut self,
        _receipt_path: &ReceiptPath,
        _receipt: Receipt,
    ) -> Result<(), ContextError> {
        todo!()
    }

    fn store_packet_acknowledgement(
        &mut self,
        _ack_path: &AckPath,
        _ack_commitment: AcknowledgementCommitment,
    ) -> Result<(), ContextError> {
        todo!()
    }

    fn delete_packet_acknowledgement(&mut self, _ack_path: &AckPath) -> Result<(), ContextError> {
        todo!()
    }

    fn store_channel(
        &mut self,
        _channel_end_path: &ChannelEndPath,
        _channel_end: ChannelEnd,
    ) -> Result<(), ContextError> {
        todo!()
    }

    fn store_next_sequence_send(
        &mut self,
        _seq_send_path: &SeqSendPath,
        _seq: Sequence,
    ) -> Result<(), ContextError> {
        todo!()
    }

    fn store_next_sequence_recv(
        &mut self,
        _seq_recv_path: &SeqRecvPath,
        _seq: Sequence,
    ) -> Result<(), ContextError> {
        todo!()
    }

    fn store_next_sequence_ack(
        &mut self,
        _seq_ack_path: &SeqAckPath,
        _seq: Sequence,
    ) -> Result<(), ContextError> {
        todo!()
    }

    fn increase_channel_counter(&mut self) -> Result<(), ContextError> {
        todo!()
    }

    fn emit_ibc_event(&mut self, _event: IbcEvent) -> Result<(), ContextError> {
        todo!()
    }

    fn log_message(&mut self, _message: String) -> Result<(), ContextError> {
        todo!()
    }
}

impl ClientExecutionContext for Context<'_> {
    type V = <Self as ValidationContext>::V;
    type AnyClientState = <Self as ValidationContext>::AnyClientState;
    type AnyConsensusState = <Self as ValidationContext>::AnyConsensusState;
  
    fn store_client_state(
        &mut self,
        _client_state_path: ClientStatePath,
        client_state: Self::AnyClientState,
    ) -> Result<(), ContextError> {
        let key = ClientStatePath::leaf().into_bytes();
  
        let encoded_client_state = self.encode_client_state(client_state)?;
  
        self.insert(key, encoded_client_state);
  
        Ok(())
    }
  
    fn store_consensus_state(
        &mut self,
        consensus_state_path: ClientConsensusStatePath,
        consensus_state: Self::AnyConsensusState,
    ) -> Result<(), ContextError> {
        let key = consensus_state_path.leaf().into_bytes();
        
        let encoded_consensus_state = Any::from(consensus_state.clone()).encode_to_vec();

        let wasm_consensus_state = WasmConsensusState {
            data: encoded_consensus_state,
        };

        self.insert(key, Any::from(wasm_consensus_state).encode_to_vec());

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