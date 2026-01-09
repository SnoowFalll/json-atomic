#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Cosine similarity mapeada para [0,1] como "confidence".
/// 1.0 = idênticos, ~0.5 = ortogonais, ~0.0 = opostos.
pub fn trajectory_confidence(a: &[f32], b: &[f32]) -> f32 {
    debug_assert_eq!(a.len(), b.len());
    let mut dot = 0.0f32;
    let mut na = 0.0f32;
    let mut nb = 0.0f32;
    for i in 0..a.len() {
        dot += a[i] * b[i];
        na += a[i] * a[i];
        nb += b[i] * b[i];
    }
    if na == 0.0 || nb == 0.0 {
        return 0.0;
    }
    let cos = dot / (na.sqrt() * nb.sqrt());
    (cos + 1.0) * 0.5
}
