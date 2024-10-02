use alloy_primitives::Address;
use reth_primitives::{StorageKey, B256};
use serde::{Deserialize, Serialize};

use crate::account::ProcessedMPTProof;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct HdpStorage {
    pub address: Address,
    pub slot: B256,
    pub storage_key: StorageKey,
    pub proofs: Vec<ProcessedMPTProof>,
}
