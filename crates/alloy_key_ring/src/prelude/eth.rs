use crate::prelude::*;

pub use alloy::primitives::Address;
pub use alloy::signers::{
    Signature,
    k256::ecdsa::signature::rand_core::OsRng,
    k256::ecdsa::signature::rand_core::RngCore,
    SignerSync,
    Signer,
    local::coins_bip39::Mnemonic,
    local::MnemonicBuilder,
    local::coins_bip39::English,
};


#[cfg(feature = "trezor")]
pub use alloy::signers::{
    trezor::TrezorSigner,
    trezor::HDPath::TrezorLive,
};


pub mod eth_address {
    use super::*;

    pub fn serialize<S>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let address = Address::from_slice(bytes.as_slice());

        serializer.serialize_str(&address.to_checksum(None))
        // serializer.serialize_str(&address.to_checksum(Some(0)))
            // let hex_string = format!("0x{}", hex::encode(bytes));
            // serializer.serialize_str(&hex_string)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let req: String = serde::Deserialize::deserialize(deserializer)?;

        // let address = Address::parse_checksummed(req, Some(1))
        let address = Address::parse_checksummed(req, None)
            .map_err(|err| serde::de::Error::custom(err.to_string()))?;

        let res = address.into_array().to_vec();

        Ok(res)
    }
}

