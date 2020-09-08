use crate::util::dfs_serializer;
use base64::DecodeError;
use core::num::ParseIntError;
use rmp_serde as serde_mgpk;
use serde_cbor;
use serde_json;
use thiserror::Error;
use ursa::CryptoError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error during Serialization: {0}")]
    SerializationError(String),

    #[error("JSON Serialization error")]
    JSONSerializationError {
        #[from]
        source: serde_json::Error,
    },

    #[error("CBOR Serialization error")]
    CBORSerializationError {
        #[from]
        source: serde_cbor::Error,
    },

    #[error("MessagePack Serialization error")]
    MsgPackSerializationError {
        #[from]
        source: serde_mgpk::encode::Error,
    },

    #[error("DFS Serialization error")]
    DFSSerializationError {
        #[from]
        source: dfs_serializer::Error,
    },

    #[error("Error parsing numerical value: {source}")]
    IntegerParseValue {
        #[from]
        source: ParseIntError,
    },

    #[error("Error while applying event: {0}")]
    SemanticError(String),

    #[error("validation error")]
    CryptoError(CryptoError),

    #[error("Deserialization error")]
    DeserializationError,

    #[error("Base64 Decoding error")]
    Base64DecodingError {
        #[from]
        source: DecodeError,
    },

    #[error("Improper Prefix Type")]
    ImproperPrefixType,
}
