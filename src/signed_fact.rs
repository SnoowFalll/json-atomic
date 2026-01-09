#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

use ed25519_dalek::{Signature, VerifyingKey};
use hex::ToHex;

#[derive(Clone, Debug)]
pub struct SignedFact {
    /// Canonical bytes JSON✯Atomic
    pub canonical: Vec<u8>,
    /// BLAKE3(content) — 32 bytes
    pub cid: [u8; 32],
    /// Ed25519 signature over CID
    pub signature: [u8; 64],
    /// Public key (Ed25519)
    pub public_key: [u8; 32],
    pub hash_alg: &'static str,  // "blake3"
    pub sig_alg: &'static str,   // "ed25519"
    pub canon_ver: &'static str, // "1"
    pub format_id: &'static str, // "json-atomic/1"
}

impl SignedFact {
    pub fn verifying_key(&self) -> VerifyingKey {
        VerifyingKey::from_bytes(&self.public_key).expect("valid pk")
    }
    pub fn signature_obj(&self) -> Signature {
        Signature::from_bytes(&self.signature)
    }
    pub fn cid_hex(&self) -> String {
        self.cid.encode_hex::<String>()
    }
}
