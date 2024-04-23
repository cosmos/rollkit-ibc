use crate::types::AnyConsensusState;
use ibc_client_cw::api::ClientType;
use ibc_client_tendermint::client_state::ClientState as TendermintClientState;

pub struct RollkitClient;

impl<'a> ClientType<'a> for RollkitClient {
    type ClientState = TendermintClientState;
    type ConsensusState = AnyConsensusState;
}
