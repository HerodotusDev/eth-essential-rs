use std::error::Error;

use alloy_primitives::B256;
use alloy_rpc_types_eth::EIP1186AccountProofResponse;
use reth_primitives::Account;
use reth_trie::{AccountProof, StorageProof, EMPTY_ROOT_HASH};
use serde::{Deserialize, Serialize};

/// The account proof with the bytecode.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountProofWithBytecode {
    /// The account proof.
    pub proof: AccountProof,
}

impl AccountProofWithBytecode {
    pub fn from_eip1186_proof(proof: EIP1186AccountProofResponse) -> Self {
        Self {
            proof: eip1186_proof_to_account_proof(proof),
        }
    }

    /// Verifies the account proof against the provided state root.
    pub fn verify(&self, state_root: B256) -> Result<(), Box<dyn Error>> {
        self.proof.verify(state_root).unwrap();

        Ok(())
    }
}

/// Converts an [EIP1186AccountProofResponse] to an [AccountProof].
pub fn eip1186_proof_to_account_proof(proof: EIP1186AccountProofResponse) -> AccountProof {
    let address = proof.address;
    let balance = proof.balance;
    let code_hash = proof.code_hash;
    let nonce = proof.nonce;
    let storage_root = proof.storage_hash;
    let account_proof = proof.account_proof;
    let storage_proofs = proof
        .storage_proof
        .into_iter()
        .map(|storage_proof| {
            let key = storage_proof.key;
            let value = storage_proof.value;
            let proof = storage_proof.proof;
            let mut sp = StorageProof::new(key.0);
            sp.value = value;
            sp.proof = proof;
            sp
        })
        .collect();

    let (storage_root, info) =
        if nonce == 0 && balance.is_zero() && storage_root.is_zero() && code_hash.is_zero() {
            // Account does not exist in state. Return `None` here to prevent proof verification.
            (EMPTY_ROOT_HASH, None)
        } else {
            (
                storage_root,
                Some(Account {
                    nonce,
                    balance,
                    bytecode_hash: code_hash.into(),
                }),
            )
        };

    AccountProof {
        address,
        info,
        proof: account_proof,
        storage_root,
        storage_proofs,
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use alloy_primitives::U256;
    use alloy_rpc_types_eth::EIP1186AccountProofResponse;
    use reth_primitives::{address, b256, bytes};

    #[test]
    fn test_eip_1186_account_without_storage_proof() {
        // TEST CASE: account proof of ETHEREUM SEPOLIA 6127485
        let res = EIP1186AccountProofResponse {
            address: address!("7f2c6f930306d3aa736b3a6c6a98f512f74036d4"),
            balance: U256::from_str_radix("21422802379747620244", 10).unwrap(),
            code_hash: b256!("c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"),
            nonce: 119083,
            storage_hash: b256!("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"),
            account_proof: vec![
                bytes!("f90211a01e8517593e5731bcaaeb0a94eee58ae8cc264a81895f79f26b43c7e91987f6a7a0a73f2f15ad0826ba1bf8439e7a92d4267de561eedd4d3de4b296eab0ec93e9f4a02960f7f0658bc943940f201085ea766c7e687c165e89e17063ba6488f2ca76c1a07a418097f39ecfcfc501d6efea460ef4478b91a1f3f7d45235913404c80a4183a053c30d89f0400089b4e19f6c35f444a23cc89eaf687a53fcd49594dbd6823dcfa08cd70e0dfe6734ea038808cf7813abfa68aa16e55401901b2d0ccc1d87f41630a090279611ca153ab82849d98a6dcf26d363dd4fbe59c936cc7ce11cd2427a3801a08ca2b8975758d64a208d5f9e86aef246a69dcd9f44513f218bf2de60f45c67aca043d5c66de53c117518ed35a772579b7db27ad78d64bd3d5ac86162723d6e65bea028a34cefc2ec25df8dc123c63c7fb2296e61c9c83dbea6339131922e7e364454a0a1696c566d2fefdd0ecd750307b68c9af34f17d062d7350574253be384190a2ca06c1e4b449b268ee86d208eae65bc6afbf08547a0dd39476aed6f0ac7cefa6fa7a031aac4a34e48d57bcad57c6db091b9e3cfc1479953232043bc836312355b2e03a0812dae259c4f1aa59523ca357345551e0f08d52d6b42836365369c5a03ab0c96a036a1390811c64ca1bd6982c4085697298575ec9926b6819cd70f3508ac66f09ba0446cc89ca20f1d4aa9f05da82706e69bcc52ea3f5acea1b98361d4a720b9558c80"),
                bytes!("f90211a023f0c8b9f297db60f404abce353ecb64e5223c0a5d30640e2d3d53f42e2f0322a01bc463af9aebbce588d2cca301f723a30d19926dc1147d1757eb6487a5a40235a0baa1ef92f3bd0d5679283dae9c350da5f21792b3c26d3dfe1172d65d052044faa04423a23f8344dfb024f4049c4b76ad57b4ed0b792e3a12eee0eafffdb52f2ca8a09caddd8472250a8f23062a5fe5a2fb007f298dfb1412d265fe017bb28eaf9bcaa0512b89e0b5f744873679d06cb2d2478f427bf446d85e34efe01fef2bc7817ea7a01c18a73af4d342039a4e1ca9bf9c4af706bb6b7ebc773251769b98b09141fee9a0a934aaf81216ecc6b42fadffd7afa1ff9c599d985810ff642a2769be8b6f780ea085f72fd60635a26e2bdf4bd1133a326c05b4114ec0bf1a7235cf6d03715b2754a0a5208b98fb9ddfd71453845bac9f296a38d997447a341d1edfd0c4d1b7e8e9afa05a4cd8392343c1be3f158b8340efa6dddad0fdffd1f799e8fdcbc741b135d591a0b1d3463f67c1193c3aae0c3cff7d23d9344f996c10c0bc9178160b6a4279115ea0dfd3d84c1cb23b4df9a92b83c57d37378797e2e7ddffd11e753fa2676ce50373a0a8e647d9a60d570fdbd6164ce0486e10ec6b546977648a383e6440e0431a1605a0457c6a89ea970d7b656a5df66ab4686bf118c3db4c082f01eeafacc67435a477a0cb28a9915315b06ec49a97350741af109eea6949c7b9cbf385061ae6a8d94edf80"), 
                bytes!("f90211a0d9d5b23b0ead6d685bed917eae93085e8b20644b1fd2cff8891870729654e3f1a07424e76dd735fcb528fb24b8c684bcfd3ee3de68e3420e9cd8f3bdc6f304f773a007c0b6e61d04f72f0203a7a640ad9d11bbf8fc4364d3c412b51915f070dd38efa041b3a21516d7a7f009bfdab2925d08fd51bc2ce14a6e70e8b9df6e8d0c93426da06b660c80b562761b2514fc121abf41b9ede6acf6238dd1e4df9043c2cc048f03a05ca2fb8fd637f4b1aea5f093b99c126e6993f0cb15a5b415467009369ff9477fa0fbc3fb3d734c50aab170c9667d9013194ba84b064f38d47ade4d8c7b2b7ec3eea08da0495fd21cefaa68d023b8eaf67104535dffd707d00b7d28b1abd2c825c48ea06426daa14457013ba6e27b807600fdb52dc566866e6eba8ac7ad8b963f8306fda05705b464cb941a76238d161d8e90f7d6b38609b6cc3150118db47bee82aeca4ba0c8c3665ff8589f69da2e7d1c2916395b241d2a19e2d33f8f0c16c26c26467b37a0ae18179410122e6e4d74aa9621b11b9d0c5bb55b61585bcc2a0b6e72ce1e44c1a057710a1ebf121e99821c63bd8e4d7776ca11203dd06fc853506d48f72299054da07388988dbc4599e2ebf670002371ce453db5e88733ecc49bc200e028b32fd637a06853b90f6f8e6f881fb2a414e19aa7cfaacf0f776a95385825e5ac9544db0c7aa089cb81336cf8fede518e50396dbb6134c3070ac36f7b7ff911615f861e1d800d80"), 
                bytes!("f90211a09b90d05b3edd4235b372d89288d2e3eb0502b18cd045d4b8869189825e7f33d2a07ca5b9ea0ac77941e195aba57eedd2a8a3154e47225e7604fbe183c7ab9fb7c6a088c1720243eb0104e85ee8cf57760b7ac59edc7ea85834ae45912fc77227aca4a0b9e5ac4a195f03e58096812d486817f13e402f5d057f1310c6187b0d630b1636a0cd56ec82c7c68c3916015c7ca24ecdc7f88126fd63a8424421e1b65305439ac7a03b573e4c4d26b282d384800fa04bb7d805d50099cc0f0779f4ea6b06466888dca07d8223dac6c3b1b35e5ee5c9aab965126988bd654194ec6c7b8427ae8c3ed1fba04b80000e279df41416799bc0145624172c30f35466817087d3e0dca3d2fc6cdca04900170cf0ab29833b6db80a9764965a7af3f6bb3cafdcf07c157e01ef664ba7a00cc2e2d6b778728933f80385d81df6de8b0bda7b0cdd16feb10f9169104aa18da0d78e818dfb0aa2d2c8ba251e0dec6941d65160a9bf66d0aa3d0bdd19177a87b4a0fca8f4aa95e47faf8eb197e24e9c6be2af0e3a7c6525d344b7bbdc6fc6cfa64fa0f2fad58bf3e61d53cfed82ece34b6f7392d5d8c3dc3acb038bec099393b0eba1a0a9014bd11e2dfd980f0c71d7a397823088b5dfb4f1554878e8d396ce5528e534a0cd8496abe3d7644adaf79da1cb69f1b8332b0c2c5d9b052cc91a1aea909cc641a0424a769dbe98a20891a02da93ca2ed91194f2c0bfe9fcdf53b62223caf8be9a280"),
                bytes!("f90211a01155a7b563ab116e315cb84ea63f1bcdf209508d01072216f4987d30de35da1aa0411d71b4514191ae7974ea55c7579a164c81bcb5fcec64bb13d71a36a11d772ca08c9a17c69345609c46e60526b1b4bcacf347fd3b903d70f77f9481ed6c082527a0f0608605a1f693a9e20d09452c0c1399dc1787bd4691a9379fbd984176f5b3b5a0b48a1df4d35182ba12a612e7c014193ec407d3f8e816133b8f996bc7b20e78cea0db4058e768cbe8bf18bf7c602fcf766d5566b7c9ad07904b975d53f340322f99a096904899d76215181ef3a496ba64110507468d0b273463612286dcd904eaf1c9a0ae5c71169ca135fe084f8d157c481d6528650bee44b27f9c09b0bd3322263b0ea0fe4f65bd44313324b447471a58a47a2d51c219513fab84d5c11774c59ff5440da003e50aacb3f4cbb8b1b8afdcb9f5f604e95a5329f2ef03f2b2990ad3af137aa0a08b03204f5ea5616a229b21dae26c978485e95f530bcf4e766029d082a20f56a1a02a177496bb4a6a48fa7fc92b0eb5645bfa657de0f3389a91d6881f5354b88efaa0325b5fdcad051ed9100dd18ae375dc569fea27eb6f675ae915c22020d019b408a076b9673f1f77e632bbe27953abec69825980b0b8939a59104acd943dbaa12c5ea01ef19106e6d3ccfd2dfb47e9479570c17fec2399c61b106452bb09883c65522fa0878cc5831a6dcf054e3b227c18748f3853cd66c1674f649e2f184684842b9efe80"),
                bytes!("f901f1a012eebf080b2bc5f43a4e26ea405b974066a98aabb57520b0bdc60d2a8d1b2d6d80a05ad8f58c5eeb611212582513105c3967f2be0c437c163e0a500f3e99376a1d0da0b782d012da1b51544d6928a73327110ae12cf6fe922c8efcf9402c0a3d26e864a0ac6f169af9d2c18286a6d14e0104c028f1ef1334027025cec6fd00b1d42ae0d3a07039ebe73cd8e4a725451e6a9353314ec4b7a0899b79c192404bae1f13d8b8f9a08ad26df1b15582ec4dc4f08efdf9df9302f1a61ca688a217b93832269c7bee81a020a06be73b79a2293abc6e08b110a2dfe44804f705ff5c1220c170c927515b19a0a3a01d3ab546f5bc3eda8c5ea8d0c60b92ee1c6c513e08032f422b4114325522a0cf60aebe174d15925f304a8226f6162b32a4f817f3153c9db02b359f95496bd6a0729d651316546407ccdcd3b05358318105bd5c7ee6efa3fbc3c8cf3afe9f0363a01ec3512523b23d000cfd0ff5eaf77787223de5fc55dd3cda8d3792634a037977a08bc0a97065188cd5310ebceb9a17200651e6c767c47f047735404284c09b8c58a0864ff5e9ad3b6a76602e5575c52f9ec123679f7509451fa98cb2e152081a21b5a0275a300b4cb1ad1c3404cb863df2cd12b855bdf13895a464cb778ac53e1fa11ca046039551ae1681a75f809cd1a7fe6e759ffb5599d103263747a9b753b6cb725580"), 
                bytes!("f89180a0c71b5c99cbe57d80e00197f49afb41c3c81f829fce67b4889607b19fec839528808080a058d90c97d5325ac87fdf362e1f3460000534c81a1ed807d19c5f3532d9d65bbb808080a07b3c71cc818328815c79bcd344c717789bde929b23bf30bfe28a36ca3cad72cd80808080a0399fc00ebdefd164330b05393631464d5cfe06bfde27c928ff0f5a48356f91158080"), 
                bytes!("f8729d3d41ff168cfccb34c4efa2db7e4f369c363cf9480dc12886f2b6fb82a5b852f8508301d12b8901294d1542f23ff594a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a0c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470")
            ],
            storage_proof: vec![],
        };

        let state_root =
            B256::from_str("0x6f184a0cf582192768fc6c8c697da0e9eb85b623c0cfea2b26034e29cdc88628")
                .unwrap();
        let account_proof = AccountProofWithBytecode::from_eip1186_proof(res);
        account_proof.verify(state_root).unwrap();
    }
}
