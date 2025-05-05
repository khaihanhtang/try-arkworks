#[cfg(test)]
pub mod tests {
    use std::ops::Add;
    use ark_ff::AdditiveGroup;
    use ark_test_curves::bls12_381::Fq2 as F;
    use ark_std::{One, UniformRand};

    #[test]
    pub fn test_additive_group() {
        let mut rng = ark_std::test_rng();
        let a = F::rand(&mut rng);
        let b = F::rand(&mut rng);
        // let c = <F as AdditiveGroup>::Scalar::rand(&mut rng);
        
        let c = a + b;
        let d = a - b;
        assert_eq!(c + d, a.double());
    }
}

