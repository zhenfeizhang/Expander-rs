//! RAW commitment refers to the case where the prover does not commit to the witness at all.
//! The prover will send the whole witnesses to the verifier.

use arith::{Field, MultiLinearPoly, Serde};

pub struct RawOpening {}

pub struct RawCommitment<F> {
    pub mpoly: MultiLinearPoly<F>,
}

impl<F: Field + Serde> RawCommitment<F> {
    pub fn size(&self) -> usize {
        self.mpoly.len() * F::SIZE
    }
    pub fn serialize_into(&self, buffer: &mut [u8]) {
        self.mpoly.serialize_into(buffer)
    }
    pub fn deserialize_from(buffer: &[u8], poly_size: usize) -> Self {
        assert_eq!(buffer.len(), poly_size * F::SIZE);
        RawCommitment {
            mpoly: MultiLinearPoly::<F>::deserialize_from(buffer),
        }
    }
}

impl<F: Field + Serde> RawCommitment<F> {
    pub fn new(poly_vals: Vec<F>) -> Self {
        RawCommitment {
            mpoly: MultiLinearPoly::<F>::new(poly_vals),
        }
    }
    pub fn verify(&self, x: &[F::BaseField], y: F) -> bool {
        y == MultiLinearPoly::<F>::eval_multilinear(&self.mpoly.evals, x)
    }
}
