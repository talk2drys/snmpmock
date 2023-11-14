use rasn::types::ObjectIdentifier;

use crate::error::Error;

#[derive(Debug)]
pub enum SNMPDataType {
    Integer,
    String,
    Null,
    Oid,
    IpAddress,
    Counter32,
    Gauge32,
    Timeticks,
    Opaque,
    Counter64,
}

pub trait ObjectIdentifierExt {
    fn to_string(&self) -> String;
    fn from_str(oid: &str) -> Result<ObjectIdentifier, Error>;
}

impl ObjectIdentifierExt for ObjectIdentifier {
    fn to_string(&self) -> String {
        let mut vector = self
            .as_ref()
            .iter()
            .map(|element| element.to_string())
            .collect::<Vec<String>>();
        vector.insert(0, ".".into());

        vector.join(".")
    }

    fn from_str(oid: &str) -> Result<ObjectIdentifier, Error> {
        let mut oid_arr = vec![];
        let splitted_str = oid.split('.');
        let mut first_iteration = true;
        for value in splitted_str {
            // oid values can start with a `.` so this handles that
            if first_iteration && value.is_empty() {
                first_iteration = false;
                continue;
            }

            let value = value.parse::<u32>();
            if value.is_err() {
                return Err(Error::ParseOidError(oid.into()));
            };
            first_iteration = false;
            oid_arr.push(value.unwrap())
        }

        Ok(ObjectIdentifier::new(oid_arr).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error;

    use super::ObjectIdentifierExt;
    use rasn::types::ObjectIdentifier;

    #[test]
    fn can_parse_valid_oid_from_string() {
        let oids = [
            "1.3.6.1.2.1.1.1",
            "1.3.6.1.2.1.1.2",
            "1.3.6.1.2.1.1.9.1.4",
            ".1.3.6.1.4.1.29671.1.1.3",
        ];

        for oid in oids {
            let expected_oid = oid
                .split('.')
                .filter(|element| !element.is_empty())
                .map(|element| element.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let expected_oid = ObjectIdentifier::new(expected_oid).unwrap();

            let parsed_oid = ObjectIdentifier::from_str(oid).unwrap();
            assert_eq!(parsed_oid, expected_oid)
        }
    }

    #[test]
    fn return_parse_error_for_invalid_oid_string() {
        let invalid_oids = [
            "1.3.uu.1.2.1.1.1",
            "1.3.6.1.2.1.1.2.",
            "1.3.6..1.2.1.1.9.1.4",
            ".1.3.6..1.4.1.29671.1.1.3.",
        ];

        for oid in invalid_oids {
            let parsed_oid = ObjectIdentifier::from_str(oid);
            assert!(parsed_oid.is_err());
            assert_eq!(parsed_oid, Err(Error::ParseOidError(oid.into())));
        }
    }
}
