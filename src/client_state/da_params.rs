use core::fmt::{Debug, Display, Error as FmtError, Formatter};
use core::str::FromStr;

use ibc_core::host::types::identifiers::ClientId;
use ibc_core::primitives::proto::Protobuf;
use ibc_proto::ibc::lightclients::rollkit::v1::DaParams as RawDaParams;

use crate::types::Error;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, PartialEq, Eq)]
pub struct DaParams {
    pub client_id: ClientId,
    pub fraud_period_window: u64,
}

impl Debug for DaParams {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, " DaParams {{...}}")
    }
}

impl Display for DaParams {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(
            f,
            "DaParams {{ client_id: {}, fraud_period_window: {} }}",
            &self.client_id, &self.fraud_period_window,
        )
    }
}

impl DaParams {
    pub fn new(client_id: ClientId, fraud_period_window: u64) -> Self {
        Self {
            client_id,
            fraud_period_window,
        }
    }
}

impl Protobuf<RawDaParams> for DaParams {}

impl TryFrom<RawDaParams> for DaParams {
    type Error = Error;

    fn try_from(raw: RawDaParams) -> Result<Self, Self::Error> {
        let client_id = ClientId::from_str(&raw.client_id).map_err(Error::source)?;

        Ok(Self::new(client_id, raw.fraud_period_window))
    }
}

impl From<DaParams> for RawDaParams {
    fn from(value: DaParams) -> Self {
        Self {
            client_id: value.client_id.to_string(),
            fraud_period_window: value.fraud_period_window,
        }
    }
}
