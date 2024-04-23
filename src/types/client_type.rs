use ibc_client_cw::api::ClientType;

use crate::client_state::ClientState as RollkitClientState;
use crate::consensus_state::AnyConsensusState;

pub struct RollkitClient;

impl<'a> ClientType<'a> for RollkitClient {
    type ClientState = RollkitClientState;
    type ConsensusState = AnyConsensusState;
}
