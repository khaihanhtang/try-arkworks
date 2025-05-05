// use ark_crypto_primitives::merkle_tree::{Config, MerkleTree, Path};
// #[derive(Clone)]
// pub struct MerkleConfig;
// impl Config for MerkleConfig {
//     type Leaf = ();
//     type LeafDigest = ();
//     type LeafInnerDigestConverter = ();
//     type InnerDigest = ();
//     // Our Merkle tree relies on two hashes: one to hash leaves, and one to hash pairs
//     // of internal nodes.
//     type LeafHash = LeafHash;
//     type TwoToOneHash = TwoToOneHash;
// }
//
// #[cfg(test)]
// mod tests {
//     use ark_crypto_primitives::merkle_tree;
//     use ark_crypto_primitives::crh::CRHScheme;
//     use ark_crypto_primitives::crh::{TwoToOneCRHScheme};
//
//     // pub type RootVar = <TwoToOneHashGadget as TwoToOneCRHGadget<TwoToOneHash, ConstraintF>>::OutputVar;
//     #[test]
//     fn test_merkle_tree_example() {
//         let mut rng = ark_std::test_rng();
//         let leaf_crh_params = <LeafHash as CRHScheme>
//     }
// }