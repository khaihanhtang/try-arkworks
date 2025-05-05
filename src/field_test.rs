#[cfg(test)]
pub mod field_test {
    use ark_ff::Field;
    use ark_test_curves::bls12_381::Fq2 as F;
    use ark_std::UniformRand;
    use ark_std::One;
    #[test]
    fn test_field() {
        let mut rng = ark_std::test_rng();
        let a = F::rand(&mut rng);
        let b = F::rand(&mut rng);
        let c = a + b;
        let d = a - b;
        let e = c * d;
        assert_eq!(e, a.square() - b.square());
        
    }
}