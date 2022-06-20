use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

pub struct Secp256K1EncryptedScalar {
    base58: String,
}

impl Encoded for Secp256K1EncryptedScalar {
    type Coder = EncodedBytesCoder;

    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(Secp256K1EncryptedScalar { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "seesk",
    base58_length: 93,
    bytes_prefix: &[1, 131, 36, 86, 248],
    bytes_length: 60,
};

impl TryFrom<&Vec<u8>> for Secp256K1EncryptedScalar {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for Secp256K1EncryptedScalar {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for Secp256K1EncryptedScalar {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Secp256K1EncryptedScalar::new(value)
    }
}

impl TryFrom<&str> for Secp256K1EncryptedScalar {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Secp256K1EncryptedScalar::new(value.to_string())
    }
}

impl TryFrom<&Secp256K1EncryptedScalar> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &Secp256K1EncryptedScalar) -> Result<Self> {
        value.to_bytes()
    }
}
