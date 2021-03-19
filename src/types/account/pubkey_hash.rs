use crate::params;
use anyhow::ensure;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryInto;

/// Hash of the account's owner public key.
///
/// This is an essential type used within Fluidex network to authorize transaction author
/// to perform an operation.
///
/// `PubKeyHash` is calculated as the Rescue hash of the public key byte sequence.
#[derive(Copy, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
pub struct PubKeyHash {
    pub data: [u8; params::ADDRESS_LEN],
}

impl std::fmt::Debug for PubKeyHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl PubKeyHash {
    /// Creates an uninitialized `PubkeyHash` object.
    /// This value is used for new accounts to signalize that `PubKeyHash` was not yet
    /// set for the corresponding account.
    /// Accounts with unset `PubKeyHash` are unable to execute L2 transactions.
    pub fn zero() -> Self {
        PubKeyHash {
            data: [0; params::ADDRESS_LEN],
        }
    }

    /// Converts the `PubKeyHash` object into its hexadecimal representation.
    /// `PubKeyHash` hexadecimal form is prepended with the `fluidex:` prefix.
    ///
    pub fn to_hex(&self) -> String {
        format!("fluidex:{}", hex::encode(&self.data))
    }

    /// Decodes `PubKeyHash` from its hexadecimal form.
    /// Input string must have a `fluidex:` prefix.
    ///
    pub fn from_hex(s: &str) -> Result<Self, anyhow::Error> {
        ensure!(s.starts_with("fluidex:"), "PubKeyHash should start with fluidex:");
        let bytes = hex::decode(&s[8..])?;
        Self::from_bytes(&bytes)
    }

    /// Decodes `PubKeyHash` from the byte sequence.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
        ensure!(bytes.len() == params::ADDRESS_LEN, "Size mismatch");
        Ok(PubKeyHash {
            data: bytes.try_into().unwrap(),
        })
    }
}

impl Serialize for PubKeyHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_hex())
    }
}

impl<'de> Deserialize<'de> for PubKeyHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        PubKeyHash::from_hex(&string).map_err(serde::de::Error::custom)
    }
}
