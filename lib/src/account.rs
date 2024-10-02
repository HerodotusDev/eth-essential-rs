use reth_primitives::{Address, Bytes};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct HdpAccount {
    pub address: Address,
    pub account_key: String,
    pub proofs: Vec<ProcessedMPTProof>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct ProcessedMPTProof {
    pub block_number: u64,
    pub proof: Vec<Bytes>,
}
