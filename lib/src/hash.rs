use alloy_primitives::Keccak256 as AlloyKeccak256;
use alloy_primitives::B256;
use starknet_types_core::{felt::Felt, hash::StarkHash};

pub trait Hash {
    type HeaderType;
    type HashOutput;
    type HashInput;

    /// Hash (x, y) -> Output
    fn hash(x: Self::HashInput, y: Self::HashInput) -> Self::HashOutput;

    /// Hash (x) -> Output, x is block header and Output is the element of MMR
    fn hash_key(value: Self::HeaderType) -> Self::HashOutput;
}

pub struct StarkPoseidoen;
impl Hash for StarkPoseidoen {
    type HeaderType = Vec<Felt>;
    type HashOutput = Felt;
    type HashInput = Felt;

    fn hash(x: Self::HashInput, y: Self::HashInput) -> Self::HashOutput {
        starknet_types_core::hash::Poseidon::hash(&x, &y)
    }

    fn hash_key(value: Self::HeaderType) -> Self::HashOutput {
        starknet_types_core::hash::Poseidon::hash_array(&value)
    }
}

pub struct Keccak256;
impl Hash for Keccak256 {
    type HeaderType = Vec<u8>;
    type HashOutput = B256;
    type HashInput = B256;

    fn hash(x: Self::HashInput, y: Self::HashInput) -> Self::HashOutput {
        let mut haser = AlloyKeccak256::new();
        haser.update(x);
        haser.update(y);
        haser.finalize()
    }

    fn hash_key(value: Self::HeaderType) -> Self::HashOutput {
        let mut haser = AlloyKeccak256::new();
        haser.update(value);
        haser.finalize()
    }
}
