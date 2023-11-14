use rasn::types::ObjectIdentifier;

use crate::error::Error;
use crate::types::{ObjectIdentifierExt, SNMPDataType};

pub fn parse_snmpwalk_line(line: &str) -> Result<(ObjectIdentifier, SNMPDataType, String), Error> {
    let parts: Vec<&str> = line.splitn(2, '=').map(|s| s.trim()).collect();
    if parts.len() == 2 {
        let oid = parts[0].trim();
        let rest = parts[1];

        // parse to ObjectIdentifier
        let oid = ObjectIdentifier::from_str(oid)?;

        if let Some((datatype, value)) = rest.split_once(':') {
            let datatype_number = match datatype {
                "INTEGER" => SNMPDataType::Integer,
                "STRING" => SNMPDataType::String,
                "NULL" => SNMPDataType::Null,
                "OID" => SNMPDataType::Oid,
                "IpAddress" => SNMPDataType::IpAddress,
                "Counter32" => SNMPDataType::Counter32,
                "Gauge32" => SNMPDataType::Gauge32,
                "Timeticks" => SNMPDataType::Timeticks,
                "Counter64" => SNMPDataType::Counter64,
                "Opaque" => SNMPDataType::Opaque,
                _ => SNMPDataType::Opaque, // unsurpoted data type
            };
            Ok((oid, datatype_number, value.trim().to_string()))
        } else {
            Ok((oid, SNMPDataType::String, rest.to_string()))
        }
    } else {
        Err(Error::InvalidLine(line.into()))
    }
}
