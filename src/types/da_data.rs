use std::str::FromStr;
use core::fmt::{Debug, Display, Error as FmtError, Formatter};
use base64::engine::general_purpose;
use base64::Engine;

use ibc_proto::ibc::lightclients::rollkit::v1::DaData as RawDaData;
use ibc::core::host::types::identifiers::ClientId;
use ibc::core::primitives::proto::Protobuf;

use crate::types::Error;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Tendermint consensus header
#[derive(Clone, PartialEq, Eq)]
pub struct DaData {
    pub client_id: ClientId,
    pub shared_proof: Vec<u8>,
}

impl Debug for DaData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, " DaData {{...}}")
    }
}

impl Display for DaData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {

        write!(f, "DaData {{ client_id: {}, shared_proof: {} }}", &self.client_id, &general_purpose::STANDARD.encode(self.shared_proof))
    }
}

impl DaData {
    pub fn new(
        client_id: ClientId,
        shared_proof: Vec<u8>,
    ) -> Self {
        Self {
            client_id,
            shared_proof,
        }
    }
}

impl Protobuf<RawDaData> for DaData {}

impl TryFrom<RawDaData> for DaData {
    type Error = Error;

    fn try_from(raw: RawDaData) -> Result<Self, Self::Error> {
        let client_id = ClientId::from_str(&raw.client_id).map_err(Error::source)?;

        Ok(Self::new(
            client_id,
            raw.shared_proof,
        ))
    }
}

impl From<DaData> for RawDaData {
    fn from(value: DaData) -> Self {
        Self {
            client_id: value.client_id.to_string(),
            shared_proof: value.shared_proof,
        }
    }
}
