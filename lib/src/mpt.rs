use alloy_primitives::B256;
use alloy_trie::proof::verify_proof;
use reth_primitives::{hex, Bytes};
use reth_trie::{AccountProof, Nibbles, StorageProof};
use serde::{Deserialize, Serialize};

use crate::{account::HdpAccount, storage::HdpStorage};

/// The account proof with the bytecode.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountProofWithBytecode {
    /// The account proof.
    pub proof: AccountProof,
}

pub fn from_processed_account_to_account_proof(
    account: HdpAccount,
    storage: Option<HdpStorage>,
    state_root: B256,
) -> bool {
    for proof in account.proofs {
        // TODO: verify storage
        let _converted_storage_proof = into_storage_proof(storage.clone());
        let key = Bytes::from(hex::decode(account.account_key.clone()).unwrap());
        let nibbles = Nibbles::unpack(key);
        // TODO: need to get rlp encoded account
        let expected = Bytes::from(hex!(
            "f84a018612309ce54000a069bbf0407f9d5438512c6218768a9581f377fa5dc119ea1409b917b75c242e1ca0eab3448e22d0f75e09ed849b2e87ac6739db4104db4eaeeffcc66cfa819755fd"
        ));
        verify_proof(
            state_root,
            nibbles,
            Some(expected.to_vec()),
            proof.proof.iter(),
        )
        .unwrap();
    }

    true
}

impl AccountProofWithBytecode {
    /// Verifies the account proof against the provided state root.
    pub fn verify(&self, state_root: B256) -> bool {
        self.proof.verify(state_root).is_ok()
    }
}

pub fn into_storage_proof(storage: Option<HdpStorage>) -> Vec<StorageProof> {
    let mut vec_storage_proofs = vec![];
    for proof in storage.clone().unwrap().proofs {
        let mut storage_proof = StorageProof::new(storage.clone().unwrap().storage_key);
        storage_proof.proof = proof.proof;
        vec_storage_proofs.push(storage_proof);
    }
    vec_storage_proofs
}

#[cfg(test)]
mod tests {
    use alloy_primitives::U256;
    use alloy_rpc_types_eth::EIP1186AccountProofResponse;
    use alloy_trie::proof::verify_proof;
    use reth_primitives::{address, b256, bytes, hex, Bytes, B256};
    use reth_trie::Nibbles;

    #[test]
    fn test_eip_1186_account_without_storage_proof() {
        // TEST CASE: account proof of ETHEREUM SEPOLIA 6127485
        let _ = EIP1186AccountProofResponse {
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

        // let state_root =
        //     B256::from_str("0x6f184a0cf582192768fc6c8c697da0e9eb85b623c0cfea2b26034e29cdc88628")
        //         .unwrap();
        // let account_proof = AccountProofWithBytecode::from_eip1186_proof(res);
        // account_proof.verify(state_root).unwrap();
    }

    #[test]
    fn test_mpt() {
        let root_hash = B256::from_slice(&hex!(
            "fe5710ac36eae31f8fd741ec4646295805efde7d5af87f75b6c9f3b478264c03"
        ));
        let encode1 = Bytes::from(hex!(
            "f90211a0dcbd475dd9c177f568ddcdc319c6397b522f9f0477f02c6f5d40f9a676d72817a00bd4f5aa23dff4b6bc82a30b7f06e299a4190e5dc145de56ebccf7b392187a09a0d61af80cd5cb39dc63864ca43eece1e0c75f3921a897e76d9749aebc53c3df8aa0864c73b1fcb6c20c7a3dc987b995a197d3970a60776b69fe86d2e8e47ea94c34a07a968065710e1a0487b9ba027fe8b4e75c48f808fa4df0e8b2e790d68af1c7a1a0415df25f69223af7620a8ae632dd96f0a266de38c7669450ccc1266e6d9f28e1a0169b43699e8685d580bf8544569306755efd3faf8ee9c573d8fc3b765716dff2a04fbb7e84242d6ebc993d1722bd4da3c8f7933c2197dc7e004029d5ecac5472dea00598724879d324152dba334b881e3ec51ab434b0f5160e501f27e41eb081a9faa0b80e76258879b1cc1553ca8fecaa7156a4bbeeae3a46e48af60f16c78b08dcc4a0cc4d6c7a08755dc1a1deeed0705ab80e722b5c26f49484052021c9afcc9f0c8da0e16df8444c1c2b049779688485f147f62074c3f74b3af089c4eda8dd71e45b77a0f3963f892598d2d4c58dc1b26ecc22670f9d6456e36682e63c10f28a6d17c19ea08e0108cf433e3125ac5e6a175171a410f4f16f3fec51752a27b6f0d32d00027fa06142d171f73708046ddc01d7a1303d875ade53e4b354a314ef031bfb2d35374ca093fb99dbe11b40160a862e9c654f6374f6e5c9e24685488a262df35175c83a8780"
        ));
        let encode2 = Bytes::from(hex!(
            "f90211a085874b8d020224e2ef3bd88570ab5c06d45decde1754f26c8d4d8008e0ccd113a003db289fe33b3b8d8c26b21562e42c050d680960ec99ef5bb315c1d636fa46eca013df832fe4094fa9df8f7a13beb8babb9874ec04c11cf0d170ad2b55869314e4a027760fd8d42862a7c2709031af97c6b34b7ad73af02bf7e0e8573da746e5d672a0e5e42a2564de948a79a3584e601c523f2a8d2f63fab6285d58a76ada5b7a94caa0b280c353d94aa1b0a94da9a2c27766cfe15dbf0299aaa0647e24dabd80e42f81a0ed48ca0484a3345f1e84d2f8a7a0d1e8549ff231ff6bffd9a46d6c0d0ab3683ca0635a725706435ff7c28ac67e918a077147acc3a8272989e2f565c00cfe49900aa09813dfd235a1906b75234a4ea7c0a2537bdc6297ece7628f10b92df4f38fccf0a09a1fec84542b2b2ea02e9f664a30b970043ff2ce93db423bc8e671db2acc5942a0e6459861abcab3d672bacfbcd0038cbaf48c2db6ff6168ebc5f32a024ae9414ea0e5da62361bea1124afcfc1908b58ba4171a2ecb9041500df3564eadfae0865c0a01d0ec6d9747a31adac893f02d7dd9e4d4de50423a758a5d0e9005d3c5a4f4e5aa04fbeca911091ce452276d951fa5d75d202c3c39bd8cad76100d40cc192382e4aa0ed06eb4338923d982930fd9ef71f51124e281f3db032094bee2e171ea5c95c69a02574cb976bb8877c1d22451852f470c0561a30ccb0a9f0b4de060d943f25a77a80"
        ));
        let encode3 = Bytes::from(hex!(
            "f90211a0084de79b5e883c52b17995d2e0acc42672f5c655d6af15eefa49383cfeff7f4fa0445910a988ca4f62953b07a03032d4018c686160c48bbcd8b4a90dd69fbde0cda05577225cb64372d7e288773beab518921e402946977279f6ea43f0cc01b134cba0280baa1097cec29878b9f6e3ddd7b6e941d66aa1ae68188e377530f2816a87c2a0c4ff1b28e2f879ec95e217d623e1e7e9a862540306b1702e1b90df84e72c8af9a0c82365017d560783befa31d24472e2239ee9f07c2c96e531e3b71d29834dd87ba08c6c2ce740933f17d0e50730c4a90ce1c44127c0c95fa2668a8177a4cb636130a03c39ea857e88d2038809d04bdd720599f88d713a0749e6fc4aae0c868e4db3a0a028249731b03a575f0676c649411a88cf93684e2990fa3ba3f922df5937fd0239a0d730b73f1924349103f07d7675f031799894108625ad258ab906359f1b45f071a0c9272cb00358e0272f80667ae90348ec7a655df30692e5410ff3cdfdd0c74f04a015b2a43f4ec90d5870821683cf4a2cdc935e6061cc6465ae459682715f9fdea9a09bd6271ddf98dbb976d85e8fdf0c8fef73377a4cb0e312f7a6e17227d0614040a0c772ab406ca4b22d2d25d9fc2e8969b02d1699cafdcacf8a1a44ddb0505019e8a061683a17e7cb9118d7d9083260df11b5d2b4a05986c5b0423a4940bf58c6c2bfa0368010c59a191f3867c91195be126b12cd3a264ae66b3ff235b827c42bc5a37f80"
        ));
        let encode4 = Bytes::from(hex!(
            "f90211a030d16cf8968685e1fe09ecb3512a4693214b06516219522769f5a3bd61b37be0a023ef708630714cf8f1e3f3976be52a5466e4a98499a53ff82601b58ec5757063a0ac9c8d3c1e7a5161fef97bea92515a17ac26e44fd2378921482c106756a800ada0db27b9f7fc9979b2668c32c5c0248198417a4149310f872cd056b4872e59318ca042457bb6d51d099607184742c05e63595f2560f1e19074e2d85057c30a5eef90a09cce0b54c3a682f7fb3f8cff6c8f3ec26db4dc374ef5fd2e264dd1a65e18d75da009cf5dceddf7fdb5e1b9bbd2f98403e7383039d7e3bc1647a92e8d9da54670aca0b876e4ec048c5a62c713da59da80a5b0c5fcb1c2fb42fc27a50ba29f2c44d0bca0660498eab6d6f8af7f5c787083dbcff9c11a680ad7b95c5911ba3eac220fb85da0ce4da32c4c975b5a8c9525457de86141de21c5a10c87f50ac8e7f789f0f7610ba0c6c553a41dc8e0efb4b2861c5629badf4024e783f3e86c64027dacd8a6b81939a0754ff70bc9d39995034b21918c09e82622e1d5454e23f1d536c9da249e251c49a0f70fdd04323bbe0c3b54e8d1bf1cd3486f07806592e3fb709e0d09b3b6215a60a07a15f0c9006c2c39c0104e3bbc6638b46ec97cb247273b305a6cb4e1571406f4a0f245157f960b827c6e16e1cb0a8dc0ec52c308317d5745bbd526ab1132eb7675a09a6fb7cc3bcabe9d69b07b9d642e14fa28e5b0880c0a93687bdebe1e9adbc65a80"
        ));
        let encode5 = Bytes::from(hex!(
            "f90211a0bab0d0089f9b71981ee144eab1eb87260863d7c0e8860fc50baa123cd57e1076a0b32055007377e4ebece59e67dd1bdef2fab8090fe602e426a17b989b24cee030a097213247b21d1775d0e0a9fceb5cd8bf089b565e0b12aeb87c4cf5ea3b9ef647a0fb10491ba8d4b7f3ab3aef2d239e47d05f905989ba886914de453260cca87a25a0fed0c810a8f3bb9060ceaa1e0e3b60bd2cac8aa3925f257821ffd1a6c045d019a02d579a264ff31036f717769ed3f6ee42710abba3ef9c6208fff18e906f941baaa086f017b4aa106b297a4c2ff4d7d2df9b59ab2ec7542b954df44948b3a85e710aa06d8d9c4bea4ba3b6b10dddc3394b4862067b96fc6dfd17828813a5d7472ee55da03b3bcad80eebdb117e2959dedb15f20cbae17bd7c8a2efbd7faba34bc1e0d142a076f365c75ad50cdf2a3576be0650168face12d535126685fece5d2940587d82ba028d9e77aa5602c1751028395ebed9c7f23febd2336acbe30c18b7d30b9449746a0dc0ab69dba2c48c9e72774ecab8bc336cf24ae1114545620ebd497d873d29ab9a0a34fa0b0b9ed3180b42db497dd324bfbd11f76e019a5404b82f94262572c199da01c8267bd384ec055d22e28973cb5e1a6602ad4eb1c6bb2e6012cd3239710fc16a04dbfb0a097f406c15ab7947441fb164796a12932b4d88676fd1b2cd7b1d95bc2a0fa35a1f36a579056bed3cf33830d34938d92199a27b487c3cd6c7206234a0f4480"
        ));
        let encode6 = Bytes::from(hex!(
            "f901d1a06d6223af2401971b5d3667a3a58a872ea759e76582fb375e4f2de0e420df275ea0f158252d20b99f3aa36e5b97c87644eaabc50f582e769ea41cf18f7ae3602227a0a4faeacc33284fdd0eafce32e0776543e3ac349de68dfcb7abcc40b0ae82df5fa0245f6fda91c1d0dd6036c8799a9338cbf85cbbca8a3a45f35a27bb076d10cb65a080d306d21c5efccfa655b98f48b2811000fe6f60a9aebd8fdcbde7589c748e96a077499f3ba879737a73d6628cbe5d5b8ad598430031ca879cdcb3a2509d3f7d5fa0c91ebaef1a0e560845ba673efd813a313e8b9e870524adc4aa4cb6ce4eb47358a078db9a4d7a85f223a7e7b0b4e22c8f0b0c1e976d6197f0ab565b16d7d2143852a02aaaa42933c19eec648bef646eda705a1e84cffbe4ecd62a31862aee16e05241a06e516cdf1f81d33ffae52ca5bf785219501710b5302738b2c405127406ef3c94a0c8ed1799c413fefe7f902fd41911193db6378456ac10eb218c8f7a137b7b50b4a0e412c32035edec4058b02f8137c18a403f5d0274e1ca1f0beff3257f61788af8a0be49c166207007fd651f379fdd6a16bea5854e57e6fcf0109551e5d7f28f883680a017f79411b196fbea4295e681196191c969174d02a467bfd6699ef4c3c6d4fb2a8080"
        ));
        let encode7 = Bytes::from(hex!(
            "f8518080808080a01922ad14def89076bde0011d514a50cae7632d617136bb83c1b2fcbed3383c7380808080808080a0e81a4320e846af94db949f1a5298f425864e8eecbe8b72342b0aea33c0ea6e3c808080"
        ));
        let encode8 = Bytes::from(hex!(
            "f86c9d3fc8476432660877b666f653759ea69189b60d2f4a7008e70555746ad1b84cf84a018612309ce54000a069bbf0407f9d5438512c6218768a9581f377fa5dc119ea1409b917b75c242e1ca0eab3448e22d0f75e09ed849b2e87ac6739db4104db4eaeeffcc66cfa819755fd"
        ));

        let proof = vec![
            &encode1, &encode2, &encode3, &encode4, &encode5, &encode6, &encode7, &encode8,
        ];

        let key = Bytes::from(hex!(
            "962f445fc8476432660877b666f653759ea69189b60d2f4a7008e70555746ad1"
        ));

        let nibbles = Nibbles::unpack(key);

        let expected = Bytes::from(hex!(
            "f84a018612309ce54000a069bbf0407f9d5438512c6218768a9581f377fa5dc119ea1409b917b75c242e1ca0eab3448e22d0f75e09ed849b2e87ac6739db4104db4eaeeffcc66cfa819755fd"
        ));

        verify_proof(root_hash, nibbles, Some(expected.to_vec()), proof.clone()).unwrap();
    }
}
