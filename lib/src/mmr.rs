use std::{collections::VecDeque, error::Error};

#[derive(Debug, thiserror::Error)]
pub enum MmrError {
    #[error("PeaksError")]
    PeaksError,

    #[error("InvalidElementIndex")]
    InvalidElementIndex,
}

use serde::{Deserialize, Serialize};
use starknet_types_core::{
    felt::Felt,
    hash::{Poseidon, StarkHash},
};

#[derive(Serialize, Deserialize)]
pub struct MmrMeta {
    root: Felt,
    size: u128,
    peaks: Vec<Felt>,
}

impl MmrMeta {
    pub fn new(root: Felt, size: u128, peaks: Vec<Felt>) -> Self {
        Self { root, size, peaks }
    }

    pub fn verify_proof(
        &self,
        element_index: u128,
        element_value: Felt,
        proof: Vec<Felt>,
    ) -> Result<bool, Box<dyn Error>> {
        // Ensure the root hash matches the calculated root from the peaks
        let root = self.bag_peaks()?;
        assert_eq!(root, self.root);

        let leaf_count = mmr_size_to_leaf_count(self.size as usize);
        let peaks_count = leaf_count_to_peaks_count(leaf_count);

        assert_eq!(peaks_count, self.peaks.len() as u32);

        let mut hash = element_value;
        let mut leaf_index = element_index_to_leaf_index(element_index as usize)?;

        // Process the proof hashes
        for proof_hash in proof {
            let is_right = leaf_index % 2 == 1;

            // Hashing logic based on the position
            hash = if is_right {
                Poseidon::hash(&proof_hash, &hash)
            } else {
                Poseidon::hash(&hash, &proof_hash)
            };

            // Update the leaf index
            leaf_index /= 2; // Move to the parent index for the next iteration

            println!("hash: {:?}", hash.to_hex_string());
            println!("leaf_index_mut: {:?}", leaf_index);
        }

        // Get the peak information
        let (peak_index, _) = get_peak_info(self.size as usize, element_index as usize);
        let peak_hashes = self.peaks.clone();

        // Verify the final hash matches the peak hash
        Ok(peak_hashes[peak_index] == hash)
    }

    /// P = Poseidon(N | Poseidon(N | Node(p1) | Node(p2) | Node(p3))), N = size, p = peaks
    fn bag_peaks(&self) -> Result<Felt, Box<dyn Error>> {
        let final_top_peak = self.final_top_peak()?;
        let size = Felt::from(self.size);

        Ok(Poseidon::hash(&size, &final_top_peak))
    }

    fn final_top_peak(&self) -> Result<Felt, Box<dyn Error>> {
        let mut peaks_hashes: VecDeque<Felt> = self.peaks.clone().into();

        match peaks_hashes.len() {
            0 => Err(Box::new(MmrError::PeaksError)),
            1 => Ok(peaks_hashes[0]),
            _ => {
                let last = peaks_hashes.pop_back().unwrap();
                let second_last = peaks_hashes.pop_back().unwrap();
                let root0 = Poseidon::hash(&second_last, &last);

                Ok(peaks_hashes
                    .into_iter()
                    .rev()
                    .fold(root0, |prev, cur| Poseidon::hash(&cur, &prev)))
            }
        }
    }
}

fn bit_length(num: usize) -> usize {
    (std::mem::size_of::<usize>() * 8) - num.leading_zeros() as usize
}

pub fn get_peak_info(mut elements_count: usize, mut element_index: usize) -> (usize, usize) {
    let mut mountain_height = bit_length(elements_count);
    let mut mountain_elements_count = (1 << mountain_height) - 1;
    let mut mountain_index = 0;

    loop {
        if mountain_elements_count <= elements_count {
            if element_index <= mountain_elements_count {
                return (mountain_index, mountain_height - 1);
            }
            elements_count -= mountain_elements_count;
            element_index -= mountain_elements_count;
            mountain_index += 1;
        }
        mountain_elements_count >>= 1;
        mountain_height -= 1;
    }
}

pub fn leaf_count_to_peaks_count(leaf_count: usize) -> u32 {
    count_ones(leaf_count) as u32
}

pub(crate) fn count_ones(mut value: usize) -> usize {
    let mut ones_count = 0;
    while value > 0 {
        value &= value - 1;
        ones_count += 1;
    }
    ones_count
}

pub fn mmr_size_to_leaf_count(mmr_size: usize) -> usize {
    let mut remaining_size = mmr_size;
    let bits = bit_length(remaining_size + 1);
    let mut mountain_tips = 1 << (bits - 1); // Using bitwise shift to calculate 2^(bits-1)
    let mut leaf_count = 0;

    while mountain_tips != 0 {
        let mountain_size = 2 * mountain_tips - 1;
        if mountain_size <= remaining_size {
            remaining_size -= mountain_size;
            leaf_count += mountain_tips;
        }
        mountain_tips >>= 1; // Using bitwise shift for division by 2
    }

    leaf_count
}

pub fn element_index_to_leaf_index(element_index: usize) -> Result<usize, Box<dyn Error>> {
    assert!(element_index > 0);
    elements_count_to_leaf_count(element_index - 1)
}

pub fn elements_count_to_leaf_count(elements_count: usize) -> Result<usize, Box<dyn Error>> {
    let mut leaf_count = 0;
    let mut mountain_leaf_count = 1 << bit_length(elements_count);
    let mut current_elements_count = elements_count;

    while mountain_leaf_count > 0 {
        let mountain_elements_count = 2 * mountain_leaf_count - 1;
        if mountain_elements_count <= current_elements_count {
            leaf_count += mountain_leaf_count;
            current_elements_count -= mountain_elements_count;
        }
        mountain_leaf_count >>= 1;
    }

    if current_elements_count > 0 {
        Err(Box::new(MmrError::InvalidElementIndex))
    } else {
        Ok(leaf_count)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Header {
    rlp: Vec<Felt>,
    proof: HeaderInclusionProof,
}

#[derive(Serialize, Deserialize)]
pub struct HeaderInclusionProof {
    leaf_idx: u128,
    mmr_path: Vec<Felt>,
}

pub fn verify_headers_with_mmr_peaks(mmr: MmrMeta, header: Header) -> Result<bool, Box<dyn Error>> {
    let element_value = Poseidon::hash_array(&header.rlp);
    mmr.verify_proof(header.proof.leaf_idx, element_value, header.proof.mmr_path)
}

pub fn validate_mmr(mmr: MmrMeta) {
    assert!(validate_mmr_size(mmr.size));
}

// Asserts that the MMR size is valid given:
// - our condition on size (1 <= x <= 2^126)
// - the specific way the MMR is constructed, ie : a list of balanced merkle trees.
// For example,
// 0 is not a valid MMR size.
// 1 is a valid MMR size.
// 2 is not a valid MMR size.
// 3 is a valid MMR size.
// 4 is a valid MMR size.
// 5 is not a valid MMR size.
// 6 is not a valid MMR size.
// 7 is a valid MMR size.
// 8 is a valid MMR size.
// 9 is not a valid MMR size.
// 10 is a valid MMR size.
// etc.
// Params:
// - x: felt - MMR size.
// Fails if the MMR size is not valid given the above conditions.
fn validate_mmr_size(size: u128) -> bool {
    // range check (1 <= x <= 2^126)
    assert!(size >= 1 && size <= 2_u128.pow(126));
    // TODO : validate size
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bag_peaks() {
        let test_mmr_meta: MmrMeta = MmrMeta {
            root: Felt::from_hex_unchecked(
                "0x153a3aa25125c2f853eeb90f74112e893c895f1ae0cff947094f4e1b6381034",
            ),
            size: 35,
            peaks: vec![
                Felt::from_hex_unchecked(
                    "0x33883305ab0df1ab7610153578a4d510b845841b84d90ed993133ce4ce8f827",
                ),
                Felt::from_hex_unchecked(
                    "0x293d3e8a80f400daaaffdd5932e2bcc8814bab8f414a75dcacf87318f8b14c5",
                ),
                Felt::from_hex_unchecked("0x0"),
            ],
        };

        let bag = test_mmr_meta.bag_peaks().unwrap();
        assert_eq!(bag, test_mmr_meta.root);
    }

    #[test]
    fn verify_proof() {
        let test_mmr_meta: MmrMeta = MmrMeta {
            root: Felt::from_hex_unchecked(
                "0x2196def0d3c0944f72f22abc485401aed3c948f02691ec54292b89172f79f9d",
            ),
            size: 11,
            peaks: vec![
                Felt::from_hex_unchecked(
                    "0x106cab81b95b8f3d61b89db2b5e5aea8fd0bbc800f1f7930ba786db8c5340c1",
                ),
                Felt::from_hex_unchecked(
                    "0x384f427301be8e1113e6dd91088cb46e25a8f6426a997b2f842a39596bf45f4",
                ),
                Felt::from_hex_unchecked("0x6"),
            ],
        };

        assert!(test_mmr_meta
            .verify_proof(
                5,
                Felt::from_hex_unchecked("0x3"),
                vec![
                    Felt::from_hex_unchecked("0x2"),
                    Felt::from_hex_unchecked(
                        "0x5134197931125e849424475aa20cd6ca0ce8603b79177c3f76e2119c8f98c53",
                    ),
                ],
            )
            .unwrap());
    }

    #[test]
    fn test_verify_headers_with_mmr_peaks() {
        let test_mmr_meta: MmrMeta = MmrMeta {
            root: Felt::from_hex_unchecked(
                "0x492627ffa5084ec078f4d461408dfaa50b504a022c5471452d598da0040c066",
            ),
            size: 13024091,
            peaks: vec![
                Felt::from_hex_unchecked(
                    "0x262c4c9b1cb2a036924aecf563dc9952e5f8b41004310adde86f22abb793eb1",
                ),
                Felt::from_hex_unchecked(
                    "0x10b39aed56c8f244a1df559c944ada6f12b7238f8c06a2c243ba4276b8059b0",
                ),
                Felt::from_hex_unchecked(
                    "0x46f45f218ea3aec481f350cda528a6f9f926a2dd53dae302e2cb610e5f152c7",
                ),
                Felt::from_hex_unchecked(
                    "0x1d52a06e6d02569893a1d842c00bb67c044be541c614e88613d7fc7187e18c1",
                ),
                Felt::from_hex_unchecked(
                    "0x770ebf618a589c17e3dc05bda7121acbedc0b48cd25f2943dc43f395f8bf0db",
                ),
                Felt::from_hex_unchecked(
                    "0x7263e878f7deafdc49b47da57f8594d477e572d3ac2bec27bb73860a35b1899",
                ),
                Felt::from_hex_unchecked(
                    "0x7b9e99f008949f9ee33d2965708ac6773a57965514df6383d55de104a39ab8c",
                ),
                Felt::from_hex_unchecked(
                    "0x28f6ccdcd38f6be6c437d100fcd62604c3293e31342a777dc37c712869ab08c",
                ),
                Felt::from_hex_unchecked(
                    "0x13d87197fe5d6f646a57dc918dcbef210737020dca9b89537fd8718ac69da3e",
                ),
                Felt::from_hex_unchecked(
                    "0x7eef4b790b56858c0232b494034d4c8699112d88f358209f71f02d5e93a7084",
                ),
                Felt::from_hex_unchecked(
                    "0x25cd2f0b579c902c41ac26df96ed5b21e16a3127dce2b471973dc86eb4c099f",
                ),
                Felt::from_hex_unchecked(
                    "0x5fdedfd0123b7461d5b3162fe82f7f3172c42fda6209415367870086f7c7918",
                ),
                Felt::from_hex_unchecked(
                    "0x7c0a415d5a6c4c90fd2dde1b340c3be305a72aa3b758dd26b8d7b4a78b53681",
                ),
            ],
        };

        let test_header = Header {
            rlp: vec![
                Felt::from_hex_unchecked("0x167e6bf7a06502f9"),
                Felt::from_hex_unchecked("0xdca4f95cfb1ccd40"),
                Felt::from_hex_unchecked("0x5a13c40acf7e78d2"),
                Felt::from_hex_unchecked("0xacd3fd41f7a63a9f"),
                Felt::from_hex_unchecked("0x4dcc1da0cf7a78fc"),
                Felt::from_hex_unchecked("0xb585ab7a5dc7dee8"),
                Felt::from_hex_unchecked("0x4512d31ad4ccb667"),
                Felt::from_hex_unchecked("0x42a1f013748a941b"),
                Felt::from_hex_unchecked("0x9ff2944793d440fd"),
                Felt::from_hex_unchecked("0xa8fba1c9a6ae6af9"),
                Felt::from_hex_unchecked("0xd469c0f33747f751"),
                Felt::from_hex_unchecked("0xb1476d73eca0a9f1"),
                Felt::from_hex_unchecked("0x874d5147a3a0ecc4"),
                Felt::from_hex_unchecked("0x27d9569c6ab671c0"),
                Felt::from_hex_unchecked("0x68e67dfce96bee20"),
                Felt::from_hex_unchecked("0x5b109054a0f5f9e7"),
                Felt::from_hex_unchecked("0xabce3cd2ad9d178e"),
                Felt::from_hex_unchecked("0x10e5ed6dee95a472"),
                Felt::from_hex_unchecked("0xfbb391640a35f4e8"),
                Felt::from_hex_unchecked("0x7a1947a042d82f76"),
                Felt::from_hex_unchecked("0x6314311ffdf5cf8c"),
                Felt::from_hex_unchecked("0x9463a963ebe94e44"),
                Felt::from_hex_unchecked("0xc7f760050e6a5057"),
                Felt::from_hex_unchecked("0x1b9b02c41672c"),
                Felt::from_hex_unchecked("0x4906c46f88361c"),
                Felt::from_hex_unchecked("0x6c911371fa12b805"),
                Felt::from_hex_unchecked("0x4c1c03320051c7a2"),
                Felt::from_hex_unchecked("0x980481c194c40809"),
                Felt::from_hex_unchecked("0x7c800a28414069c0"),
                Felt::from_hex_unchecked("0xe298b78301017240"),
                Felt::from_hex_unchecked("0x7573200b25021338"),
                Felt::from_hex_unchecked("0x2213d6020368106"),
                Felt::from_hex_unchecked("0x8beb9e585402601e"),
                Felt::from_hex_unchecked("0xc34c08813a122656"),
                Felt::from_hex_unchecked("0xe04450e0418116"),
                Felt::from_hex_unchecked("0x27ca8e8d35900660"),
                Felt::from_hex_unchecked("0x49220bab610260d"),
                Felt::from_hex_unchecked("0x804b129191418eb8"),
                Felt::from_hex_unchecked("0x5a647b00c0a40be8"),
                Felt::from_hex_unchecked("0xe128901306e0201"),
                Felt::from_hex_unchecked("0x40c72b023c044626"),
                Felt::from_hex_unchecked("0xa0e1e228ab300827"),
                Felt::from_hex_unchecked("0x420f21290342200c"),
                Felt::from_hex_unchecked("0x8a060605538001e3"),
                Felt::from_hex_unchecked("0x94c211f02a7ada02"),
                Felt::from_hex_unchecked("0x8514e295a15d542"),
                Felt::from_hex_unchecked("0x288345a43586720a"),
                Felt::from_hex_unchecked("0xf80029022246a480"),
                Felt::from_hex_unchecked("0xa94cb33462985683"),
                Felt::from_hex_unchecked("0x4052129f03d73b01"),
                Felt::from_hex_unchecked("0x136a4280411314e8"),
                Felt::from_hex_unchecked("0x246165a86250b186"),
                Felt::from_hex_unchecked("0x902b4201410416d2"),
                Felt::from_hex_unchecked("0x742bb43302a4638a"),
                Felt::from_hex_unchecked("0x6322be6a48524029"),
                Felt::from_hex_unchecked("0x8566c6808b04068"),
                Felt::from_hex_unchecked("0xc90184ab66598380"),
                Felt::from_hex_unchecked("0x66841ce6c18380c3"),
                Felt::from_hex_unchecked("0xd0183d899bcf93a"),
                Felt::from_hex_unchecked("0x678868746567840b"),
                Felt::from_hex_unchecked("0x85362e31322e316f"),
                Felt::from_hex_unchecked("0xd51ca078756e696c"),
                Felt::from_hex_unchecked("0x83074fe811b1197c"),
                Felt::from_hex_unchecked("0x1e66107c617bfe2f"),
                Felt::from_hex_unchecked("0xa187759811e589a1"),
                Felt::from_hex_unchecked("0x887c6cadc69f7f"),
                Felt::from_hex_unchecked("0x8500000000000000"),
                Felt::from_hex_unchecked("0xedba0783f9bee02"),
                Felt::from_hex_unchecked("0x95a9360b3e8975ca"),
                Felt::from_hex_unchecked("0xc9d730d907230f1c"),
                Felt::from_hex_unchecked("0xe2a3470255fe1187"),
                Felt::from_hex_unchecked("0x883a6a29d4b69e7"),
                Felt::from_hex_unchecked("0xa000009204840000"),
                Felt::from_hex_unchecked("0xb2a26a2215758e0"),
                Felt::from_hex_unchecked("0x6558068195ed9b2d"),
                Felt::from_hex_unchecked("0x2b4b100c94d228cf"),
                Felt::from_hex_unchecked("0x81fd704a5ab0c188"),
            ],
            proof: HeaderInclusionProof {
                leaf_idx: 175968,
                mmr_path: vec![
                    Felt::from_hex_unchecked(
                        "0x3e7f1315cad8591f8c695da7be6422314eb901b61d8987d3a4e8ea0d6d55986",
                    ),
                    Felt::from_hex_unchecked(
                        "0x727965b30883a87bf412ad16fc1cdf3e0f5ca04cfba6e12a183c9e00b9a42f4",
                    ),
                    Felt::from_hex_unchecked(
                        "0x2bb4be090166054388850e72daf0981ea183a58caf1fe168df34e3dbe8ee43f",
                    ),
                    Felt::from_hex_unchecked(
                        "0x1b4d43d2a01745806ed7164aa59bae7f658bce691245618f0c376d2bdf4b8bd",
                    ),
                    Felt::from_hex_unchecked(
                        "0x361ccc6bfc25ebc64fdfdd407970da357373985578cceb677f0714cdf7bdf87",
                    ),
                    Felt::from_hex_unchecked(
                        "0x2757794640969772a98ffabb417a53fb2df19791f98a61037064dad0d994ee8",
                    ),
                    Felt::from_hex_unchecked(
                        "0x196baba4ecbf0dc319a187e8b045966d3cd320663856a2ccd6dceee31a29308",
                    ),
                    Felt::from_hex_unchecked(
                        "0x75a6c78c085b058e8e9dbbc08707225adebd4f2b3acb67703b7feb85ddceae4",
                    ),
                    Felt::from_hex_unchecked(
                        "0x1e565ec1da0650a15d4f46f2c0eb3d4549d191f330a6f9bebed76e40e5b4ba3",
                    ),
                    Felt::from_hex_unchecked(
                        "0x00bc88770e38112723046e2718585dad9b69b271dad851212379e6acc969de6",
                    ),
                    Felt::from_hex_unchecked(
                        "0x387cf7cedc350945487fd4ee111706c44bd5a815b57f5974ce440e6dde43e24",
                    ),
                    Felt::from_hex_unchecked(
                        "0x298f99d09fc847f82a9512a7b39ccf73746e078314dbe734b8f269e1286637c",
                    ),
                    Felt::from_hex_unchecked(
                        "0x725f669ddd64e3864cf53f3e8d16b888bc2835a2bf062fa032ab0b6ba15028a",
                    ),
                    Felt::from_hex_unchecked(
                        "0x49ab2b3574a65dd6648d976b5fe990131d6e7b4b7d41a2be5366710dd17b94f",
                    ),
                    Felt::from_hex_unchecked(
                        "0x33716f3f24c1835811c678186589eb5551639a40d94e47147f57c5753c7f12d",
                    ),
                    Felt::from_hex_unchecked(
                        "0x715c7749b605209670cc17e41fe1d82e97ad445bda2a3eeca74eed3b17fd886",
                    ),
                    Felt::from_hex_unchecked(
                        "0x48ce8656d39a85f29a6493eb3881a62e965e36165537bc70b225dd5f982ca23",
                    ),
                    Felt::from_hex_unchecked(
                        "0x11ecc517bc5be2a48922a0136824d6983c3e1a95c1cd32cac135ed69b54d364",
                    ),
                    Felt::from_hex_unchecked(
                        "0x5eab4c28e04e28d5056042dd8a8cb9810cb44711e7ee6da394d2e4d2b84a680",
                    ),
                    Felt::from_hex_unchecked(
                        "0x59bbae3d1cfd74546dfe68c264341cee9f95c3b3b61eeacead3bd4cb3ae232a",
                    ),
                    Felt::from_hex_unchecked(
                        "0x546455f57f4ee848d3952148e3b94700f387ee2c36730bfeda09379ce8fa509",
                    ),
                    Felt::from_hex_unchecked(
                        "0x08808a106dc9e09c29afd24be7cee31edd9f0d27ce0a3469839ef3d09ddfb43",
                    ),
                ],
            },
        };

        assert!(verify_headers_with_mmr_peaks(test_mmr_meta, test_header).unwrap());
    }
}
