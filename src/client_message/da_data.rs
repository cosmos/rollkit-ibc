use base64::engine::general_purpose;
use base64::Engine;
use core::fmt::{Debug, Display, Error as FmtError, Formatter};

use ibc_core::primitives::proto::Protobuf;
use ibc_proto::ibc::lightclients::rollkit::v1::DaData as RawDaData;

use crate::types::Error;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, PartialEq, Eq)]
pub struct DaData {
    pub shared_proof: Vec<u8>,
}

impl Debug for DaData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, " DaData {{...}}")
    }
}

impl Display for DaData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(
            f,
            "DaData {{ shared_proof: {} }}",
            &general_purpose::STANDARD.encode(&self.shared_proof)
        )
    }
}

impl DaData {
    pub fn new(shared_proof: Vec<u8>) -> Self {
        Self { shared_proof }
    }
}

impl Protobuf<RawDaData> for DaData {}

impl TryFrom<RawDaData> for DaData {
    type Error = Error;

    fn try_from(raw: RawDaData) -> Result<Self, Self::Error> {
        Ok(Self::new(raw.shared_proof))
    }
}

impl From<DaData> for RawDaData {
    fn from(value: DaData) -> Self {
        Self {
            shared_proof: value.shared_proof,
        }
    }
}
