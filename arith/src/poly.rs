use ark_std::{end_timer, log2, start_timer};

use crate::{Field, Serde};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
/// Definition for an MLE, with an associated type F.
pub struct MultiLinearPoly<F> {
    /// Number of variables in an MLE
    pub var_num: usize,
    /// MLE Evaluations
    pub evals: Vec<F>,
}

impl<F: Field> MultiLinearPoly<F> {
    /// create a new MLE with evaluations
    #[inline(always)]
    pub fn new(evals: Vec<F>) -> Self {
        let var_num = log2(evals.len()) as usize;
        MultiLinearPoly { var_num, evals }
    }

    /// length of the evaluations
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.evals.len()
    }

    /// length of the evaluations
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.evals.is_empty()
    }

    /// evaluate the mploy at a point x
    pub fn eval_at(&self, x: &[F::BaseField]) -> F {
        let timer = start_timer!(|| format!("eval mle with {} vars", x.len()));
        assert_eq!(x.len(), self.var_num);
        let mut scratch = self.evals.to_vec();
        let mut cur_eval_size = scratch.len() >> 1;
        for r in x.iter() {
            for i in 0..cur_eval_size {
                scratch[i] =
                    scratch[i * 2] + (scratch[i * 2 + 1] - scratch[i * 2]).mul_base_elem(r);
            }
            scratch.truncate(cur_eval_size);
            cur_eval_size >>= 1;
        }
        end_timer!(timer);
        scratch[0]
    }
}

impl<F: Field + Serde> Serde for MultiLinearPoly<F> {
    fn serialize_into(&self, buffer: &mut [u8]) {
        assert!(buffer.len() == self.evals.len() * F::SIZE);
        self.evals
            .iter()
            .zip(buffer.chunks_exact_mut(F::SIZE))
            .for_each(|(f, chunk)| f.serialize_into(chunk))
    }

    fn deserialize_from(buffer: &[u8]) -> Self {
        let poly_vals = buffer
            .chunks_exact(F::SIZE)
            .map(|chunk| F::deserialize_from(chunk))
            .collect::<Vec<_>>();

        let var_num = log2(poly_vals.len()) as usize;
        MultiLinearPoly {
            evals: poly_vals,
            var_num,
        }
    }
}
