//! Defines the header type for the Rollkit light client

use core::fmt::{Debug, Display, Error as FmtError, Formatter};

use ibc_clients::tendermint::types::Header as TendermintHeader;
use ibc_core::client::types::Height;
use ibc_core::primitives::proto::{Any, Protobuf};
use ibc_core::primitives::Timestamp;
use ibc_proto::ibc::lightclients::rollkit::v1::Header as RawRollkitHeader;

use crate::types::DaData;
use crate::types::Error;

pub const ROLLKIT_HEADER_TYPE_URL: &str = "/ibc.lightclients.rollkit.v1.Header";

#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Header {
    pub tendermint_header: TendermintHeader,
    pub da_data: DaData,
}

impl Debug for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "Header {{...}}")
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(
            f,
            "Header {{ tendermint_header: {}, da_data: {} }}",
            &self.tendermint_header, &self.da_data,
        )
    }
}

impl Header {
    pub fn timestamp(&self) -> Timestamp {
        self.tendermint_header.timestamp()
    }

    pub fn height(&self) -> Height {
        self.tendermint_header.height()
    }

    /// Checks if the fields of a given header are consistent with the trusted fields of this header.
    pub fn validate_basic(&self) -> Result<(), Error> {
        self.tendermint_header
            .validate_basic()
            .map_err(Error::source)
    }
}

impl Protobuf<RawRollkitHeader> for Header {}

impl TryFrom<RawRollkitHeader> for Header {
    type Error = Error;

    fn try_from(value: RawRollkitHeader) -> Result<Self, Self::Error> {
        let raw_tendermint_header = value
            .tendermint_header
            .ok_or(Error::missing("missing tendermint header"))?;

        let tendermint_header =
            TendermintHeader::try_from(raw_tendermint_header).map_err(Error::source)?;

        let da_data = value
            .da_data
            .ok_or(Error::missing("missing aggregated proof"))?
            .try_into()?;

        Ok(Header {
            tendermint_header,
            da_data,
        })
    }
}

impl From<Header> for RawRollkitHeader {
    fn from(value: Header) -> RawRollkitHeader {
        RawRollkitHeader {
            tendermint_header: Some(value.tendermint_header.into()),
            da_data: Some(value.da_data.into()),
        }
    }
}

impl Protobuf<Any> for Header {}

impl TryFrom<Any> for Header {
    type Error = Error;

    fn try_from(any: Any) -> Result<Self, Self::Error> {
        let msg = match any.type_url.as_str() {
            ROLLKIT_HEADER_TYPE_URL => {
                Protobuf::<RawRollkitHeader>::decode_vec(&any.value).map_err(Error::source)?
            }
            _ => Err(Error::invalid(any.type_url.clone()))?,
        };

        Ok(msg)
    }
}

impl From<Header> for Any {
    fn from(header: Header) -> Self {
        Any {
            type_url: ROLLKIT_HEADER_TYPE_URL.to_string(),
            value: Protobuf::<RawRollkitHeader>::encode_vec(header),
        }
    }
}
