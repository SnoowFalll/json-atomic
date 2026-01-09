#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use ed25519_dalek::{Signer, SigningKey};
use logline_core::LogLine;

use crate::errors::{SealError, VerifyError};
use crate::{
    canonize,
    version::{CANON_VERSION, FORMAT_ID},
    SignedFact,
};

pub fn seal_value<T: serde::Serialize>(
    value: &T,
    sk: &SigningKey,
) -> Result<SignedFact, SealError> {
    let canonical = canonize(value)?;
    let cid = blake3::hash(&canonical);
    let sig = sk.sign(cid.as_bytes());
    let vk = sk.verifying_key();

    Ok(SignedFact {
        canonical,
        cid: *cid.as_bytes(),
        signature: sig.to_bytes(),
        public_key: vk.to_bytes(),
        hash_alg: "blake3",
        sig_alg: "ed25519",
        canon_ver: CANON_VERSION,
        format_id: FORMAT_ID,
    })
}

pub fn verify_seal(f: &SignedFact) -> Result<(), VerifyError> {
    // Recalcula CID do canonical e compara
    let recomputed = blake3::hash(&f.canonical);
    if recomputed.as_bytes() != &f.cid {
        return Err(VerifyError::CanonicalMismatch);
    }
    let vk = f.verifying_key();
    vk.verify_strict(recomputed.as_bytes(), &f.signature_obj())
        .map_err(|_| VerifyError::BadSignature)
}

/// Sela um LogLine completo como Signed Fact (Paper II: Signed Fact de ação verificada)
pub fn seal_logline(line: &LogLine, sk: &SigningKey) -> Result<SignedFact, SealError> {
    // Reaproveita `serde` derivado do logline-core
    seal_value(line, sk)
}
