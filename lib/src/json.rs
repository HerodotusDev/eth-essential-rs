use serde::{Deserialize, Serialize};

use crate::{
    account::HdpAccount,
    mmr_keccak::{Header, MmrMeta},
    storage::HdpStorage,
};

#[derive(Serialize, Deserialize)]
pub struct MmrJson {
    pub meta: MmrMeta,
    pub headers: Vec<Header>,
    pub accounts: Vec<HdpAccount>,
    pub storages: Vec<HdpStorage>,
}
