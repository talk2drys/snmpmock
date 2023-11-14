use std::string::FromUtf8Error;

use rasn::types::ObjectIdentifier;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum NetGraphError {
    #[error("IO Error")]
    IoError(String),
    #[error("Fatal Error")]
    UnknownError,
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("Unspecified")]
    Unspecified,
    #[error("No such object")]
    NoSuchObject(ObjectIdentifier),
    #[error("No such instance")]
    NoSuchInstance,
    #[error("End of Mib view")]
    EndOfMibView,
    #[error("error parsing `{0}` Object Identifier")]
    ParseOidError(String),
    #[error("error parsing received octet to String")]
    ParseOctectError(#[from] FromUtf8Error),
    #[error("Invalid Packet")]
    InvalidPacket,
    #[error("io Error")]
    IoError(#[from] NetGraphError),
    #[error("Does not support SNMP or divice unreachable")]
    Unreachable,
    #[error("Invalide Snmp walk line")]
    InvalidLine(String),
}
