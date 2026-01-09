use thiserror::Error;

#[derive(Debug, Error)]
pub enum CanonicalError {
    #[error("floating numbers are not allowed in canonical JSON")]
    FloatNotAllowed,
    #[error("invalid unicode normalization")]
    Unicode,
    #[error("serde error: {0}")]
    Serde(String),
}

#[derive(Debug, Error)]
pub enum SealError {
    #[error("canonicalization failed: {0}")]
    Canonical(#[from] CanonicalError),
}

#[derive(Debug, Error)]
pub enum VerifyError {
    #[error("signed fact canonical bytes mismatch (recomputed CID differs)")]
    CanonicalMismatch,
    #[error("signature verification failed")]
    BadSignature,
}
