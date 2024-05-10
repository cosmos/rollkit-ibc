use ibc_client_cw::api::ClientType;
use ibc_client_tendermint::consensus_state::ConsensusState as TendermintConsensusState;

use crate::client_state::ClientState as RollkitClientState;

pub struct RollkitClient;

impl<'a> ClientType<'a> for RollkitClient {
    type ClientState = RollkitClientState;
    type ConsensusState = TendermintConsensusState;
}
