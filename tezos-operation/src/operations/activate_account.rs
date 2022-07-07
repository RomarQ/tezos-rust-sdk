use tezos_core::types::{encoded::Ed25519PublicKeyHash, hex_string::HexString};

use super::{OperationContentTag, TraitOperationContent};

pub struct ActivateAccount {
    pkh: Ed25519PublicKeyHash,
    secret: HexString,
}

impl ActivateAccount {
    pub fn pkh(&self) -> &Ed25519PublicKeyHash {
        &self.pkh
    }

    pub fn secret(&self) -> &HexString {
        &self.secret
    }

    pub fn new(pkh: Ed25519PublicKeyHash, secret: HexString) -> Self {
        Self { pkh, secret }
    }
}

impl TraitOperationContent for ActivateAccount {
    fn tag() -> &'static [u8] {
        &[OperationContentTag::ActivateAccount as u8]
    }
}
