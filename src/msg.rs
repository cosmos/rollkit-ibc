//! Contains the definition of the messages that can be sent to the CosmWasm contract.

use alloc::vec::Vec;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use cosmwasm_schema::{cw_serde, QueryResponses};
//use ibc::clients::wasm_types::serializer::Base64;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub type Bytes = Vec<u8>;

pub struct Base64;

impl Base64 {
    pub fn serialize<S: Serializer>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
        let encoded = BASE64_STANDARD.encode(bytes);
        String::serialize(&encoded, serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
        let base64 = String::deserialize(deserializer)?;
        let bytes = BASE64_STANDARD
            .decode(base64.as_bytes())
            .map_err(Error::custom)?;

        Ok(bytes)
    }
}

// ------------------------------------------------------------
// Implementation of the InstantiateMsg struct
// ------------------------------------------------------------

#[cw_serde]
pub struct InstantiateMsg {
    #[schemars(with = "String")]
    #[serde(with = "Base64", default)]
    pub client_state: Bytes,
    #[schemars(with = "String")]
    #[serde(with = "Base64", default)]
    pub consensus_state: Bytes,
    #[schemars(with = "String")]
    #[serde(with = "Base64", default)]
    pub checksum: Bytes,
}

// ------------------------------------------------------------
// Implementation of the SudoMsg enum and its variants
// ------------------------------------------------------------

#[cw_serde]
pub enum SudoMsg {}

// ------------------------------------------------------------
// Implementation of the QueryMsg enum and its variants
// ------------------------------------------------------------

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
