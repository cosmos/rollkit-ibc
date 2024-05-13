use ibc_client_tendermint::types::Misbehaviour;
use ibc_client_tendermint::types::TENDERMINT_MISBEHAVIOUR_TYPE_URL;
use ibc_core::primitives::proto::{Any, Protobuf};
use prost::Message;

use crate::client_message::header::{Header, ROLLKIT_HEADER_TYPE_URL};
use crate::types::Error;

/// Defines the union ClientMessage type allowing to submit all possible
/// messages for updating clients or reporting misbehaviour.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClientMessage {
    Header(Box<Header>),
    Misbehaviour(Box<Misbehaviour>),
}

impl ClientMessage {
    /// Decodes a `ClientMessage` from a byte array using the `Any` type.
    pub fn decode(value: Vec<u8>) -> Result<ClientMessage, Error> {
        let any = Any::decode(&mut value.as_slice()).map_err(Error::source)?;
        ClientMessage::try_from(any)
    }
}

impl Protobuf<Any> for ClientMessage {}

impl TryFrom<Any> for ClientMessage {
    type Error = Error;

    fn try_from(any: Any) -> Result<Self, Self::Error> {
        let msg = match &*any.type_url {
            ROLLKIT_HEADER_TYPE_URL => Self::Header(Box::new(Header::try_from(any)?)),
            TENDERMINT_MISBEHAVIOUR_TYPE_URL => {
                Self::Misbehaviour(Box::new(Misbehaviour::try_from(any)?))
            }
            _ => Err(Error::invalid(format!("Unknown type: {}", any.type_url)))?,
        };

        Ok(msg)
    }
}

impl From<ClientMessage> for Any {
    fn from(msg: ClientMessage) -> Self {
        match msg {
            ClientMessage::Header(header) => Any {
                type_url: ROLLKIT_HEADER_TYPE_URL.to_string(),
                value: Protobuf::<Any>::encode_vec(*header),
            },
            ClientMessage::Misbehaviour(misbehaviour) => Any {
                type_url: TENDERMINT_MISBEHAVIOUR_TYPE_URL.to_string(),
                value: Protobuf::<Any>::encode_vec(*misbehaviour),
            },
        }
    }
}
