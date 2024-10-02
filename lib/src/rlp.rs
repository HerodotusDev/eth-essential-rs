use alloy_rlp::Decodable;
use alloy_rlp::RlpDecodable;
use alloy_rlp::RlpEncodable;
use reth_primitives::{Header, B256, U256};
use std::error::Error;

pub fn get_state_root(rlp: &mut &[u8]) -> Result<B256, Box<dyn Error>> {
    let decoded = Header::decode(rlp).unwrap();
    Ok(decoded.state_root)
}

#[derive(Debug, RlpDecodable, RlpEncodable, PartialEq)]
pub struct Account {
    pub nonce: u64,
    pub balance: U256,
    pub storage_root: B256,
    pub code_hash: B256,
}

pub fn get_account_info(rlp: &mut &[u8]) -> Result<Account, Box<dyn Error>> {
    let decoded_account = Account::decode(rlp).unwrap();
    Ok(decoded_account)
}

#[cfg(test)]
mod tests {
    use alloy_primitives::private::alloy_rlp::Decodable;
    use reth_primitives::hex;

    use super::*;

    #[test]
    fn test_header_rlp_decode() {
        let mut rlp = hex!("f901fda025a5cc106eea7138acab33231d7160d69cb777ee0c2c553fcddf5138993e6dd9a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347942f14582947e292a2ecd20c430b46f2d27cfe213ca0c91d4ecd59dce3067d340b3aadfc0542974b4fb4db98af39f980a91ea00db9dca056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421b901000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000083020000018401c9c38080846173603a80a0cd039d5508e92723db0f078b5205da89144e3a6fee3a34124c966f53c35ce42c88c7faaf72b456848084342770c0").as_slice();
        let decoded = Header::decode(&mut rlp).unwrap();
        println!("{:?}", decoded);
    }
}
