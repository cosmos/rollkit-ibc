use ibc_client_tendermint::consensus_state::ConsensusState as TendermintConsensusState;
use ibc_core::client::context::client_state::ClientStateCommon;
use ibc_core::client::types::error::{ClientError, UpgradeClientError};

use ibc_core::client::types::Height;
use ibc_core::commitment_types::commitment::{
    CommitmentPrefix, CommitmentProofBytes, CommitmentRoot,
};
use ibc_core::host::types::identifiers::ClientType;
use ibc_core::host::types::path::{Path, UpgradeClientPath};
use ibc_core::primitives::proto::Any;
use ibc_core::primitives::ToVec;

use crate::client_state::{rollkit_client_type, ClientState};

impl ClientStateCommon for ClientState {
    fn verify_consensus_state(&self, consensus_state: Any) -> Result<(), ClientError> {
        self.tendermint_client_state.verify_consensus_state(consensus_state)
    }

    fn client_type(&self) -> ClientType {
        rollkit_client_type()
    }

    fn latest_height(&self) -> Height {
        self.tendermint_client_state.inner().latest_height
    }

    fn validate_proof_height(&self, proof_height: Height) -> Result<(), ClientError> {
        self.tendermint_client_state.validate_proof_height(proof_height)
    }

    fn verify_upgrade_client(
        &self,
        upgraded_client_state: Any,
        upgraded_consensus_state: Any,
        proof_upgrade_client: CommitmentProofBytes,
        proof_upgrade_consensus_state: CommitmentProofBytes,
        root: &CommitmentRoot,
    ) -> Result<(), ClientError> {
        verify_upgrade_client(
            self,
            upgraded_client_state,
            upgraded_consensus_state,
            proof_upgrade_client,
            proof_upgrade_consensus_state,
            root,
        )
    }

    fn verify_membership(
        &self,
        prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        root: &CommitmentRoot,
        path: Path,
        value: Vec<u8>,
    ) -> Result<(), ClientError> {
        verify_membership(self, prefix, proof, root, path, value)
    }

    fn verify_non_membership(
        &self,
        prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        root: &CommitmentRoot,
        path: Path,
    ) -> Result<(), ClientError> {
        verify_non_membership(self, prefix, proof, root, path)
    }
}

/// Perform client-specific verifications and check all data in the new
/// client state to be the same across all valid Rollkit clients for the
/// new chain.
///
/// You can learn more about how to upgrade IBC-connected SDK chains in
/// [this](https://ibc.cosmos.network/main/ibc/upgrades/quick-guide.html)
/// guide.
///
/// Note that this function is typically implemented as part of the
/// [`ClientStateCommon`] trait, but has been made a standalone function
/// in order to make the ClientState APIs more flexible.
pub fn verify_upgrade_client(
    client_state: &ClientState,
    upgraded_client_state: Any,
    upgraded_consensus_state: Any,
    proof_upgrade_client: CommitmentProofBytes,
    proof_upgrade_consensus_state: CommitmentProofBytes,
    root: &CommitmentRoot,
) -> Result<(), ClientError> {
    // TODO: is it ok that the chain commits a rollkit client state? Or is it better to let the chain commit a tendermint client state?

    // Make sure that the client type is of Rollkit type `ClientState`
    let upgraded_rollkit_client_state = ClientState::try_from(upgraded_client_state.clone())?;

    // Make sure that the consensus type is of Tendermint type `ConsensusState`
    TendermintConsensusState::try_from(upgraded_consensus_state.clone())?;

    let latest_height = client_state.latest_height();
    let upgraded_tendermint_client_state_height = upgraded_rollkit_client_state.latest_height();

    // Make sure the latest height of the current client is not greater then
    // the upgrade height This condition checks both the revision number and
    // the height
    if latest_height >= upgraded_tendermint_client_state_height {
        Err(UpgradeClientError::LowUpgradeHeight {
            upgraded_height: latest_height,
            client_height: upgraded_tendermint_client_state_height,
        })?
    }

    // TODO: is it ok if we use the upgrade path from the tendermint client state?
    // TODO: is it ok that in the tendermint upgrade path the chain commits to a rollkit client state?
    // Check to see if the upgrade path is set
    let mut upgrade_path = client_state.tendermint_client_state.inner().upgrade_path.clone();

    if upgrade_path.pop().is_none() {
        return Err(ClientError::ClientSpecific {
            description: "cannot upgrade client as no upgrade path has been set".to_string(),
        });
    };

    let upgrade_path_prefix = CommitmentPrefix::try_from(upgrade_path[0].clone().into_bytes())
        .map_err(ClientError::InvalidCommitmentProof)?;

    let last_height = latest_height.revision_height();

    // Verify the proof of the upgraded client state
    verify_membership(
        &client_state,
        &upgrade_path_prefix,
        &proof_upgrade_client,
        root,
        Path::UpgradeClient(UpgradeClientPath::UpgradedClientState(last_height)),
        upgraded_client_state.to_vec(),
    )?;

    // Verify the proof of the upgraded consensus state
    verify_membership(
        &client_state,
        &upgrade_path_prefix,
        &proof_upgrade_consensus_state,
        root,
        Path::UpgradeClient(UpgradeClientPath::UpgradedClientConsensusState(last_height)),
        upgraded_consensus_state.to_vec(),
    )?;

    Ok(())
}

/// Verify membership of the given value against the client's merkle proof.
///
/// Note that this function is typically implemented as part of the
/// [`ClientStateCommon`] trait, but has been made a standalone function
/// in order to make the ClientState APIs more flexible.
pub fn verify_membership(
    client_state: &ClientState,
    prefix: &CommitmentPrefix,
    proof: &CommitmentProofBytes,
    root: &CommitmentRoot,
    path: Path,
    value: Vec<u8>,
) -> Result<(), ClientError> {
    client_state
        .tendermint_client_state
        .verify_membership(prefix, proof, root, path, value)
}

/// Verify that the given value does not belong in the client's merkle proof.
///
/// Note that this function is typically implemented as part of the
/// [`ClientStateCommon`] trait, but has been made a standalone function
/// in order to make the ClientState APIs more flexible.
pub fn verify_non_membership(
    client_state: &ClientState,
    prefix: &CommitmentPrefix,
    proof: &CommitmentProofBytes,
    root: &CommitmentRoot,
    path: Path,
) -> Result<(), ClientError> {
    client_state
        .tendermint_client_state
        .verify_non_membership(prefix, proof, root, path)
}