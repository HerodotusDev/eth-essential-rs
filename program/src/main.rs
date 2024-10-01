//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

// turn on off hashser for now
// use hdp_lib::mmr::{verify_headers_with_mmr_peaks, Header, MmrMeta};
use hdp_lib::{
    account::HdpAccount,
    mmr_keccak::{verify_headers_with_mmr_peaks, Header, MmrMeta},
    mpt::from_processed_account_to_account_proof,
    rlp::get_state_root,
    storage::HdpStorage,
};

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.

    let account = sp1_zkvm::io::read::<HdpAccount>();
    let storage = sp1_zkvm::io::read::<HdpStorage>();
    let headers = sp1_zkvm::io::read::<Vec<Header>>();
    let mmr = sp1_zkvm::io::read::<MmrMeta>();

    // verify all the given headers are valid against the given mmr
    let is_valid = verify_headers_with_mmr_peaks(mmr, &headers).unwrap();
    if is_valid {
        let mut is_valid_acc = true;
        for header in headers {
            let state_root = get_state_root(&mut header.rlp.as_bytes()).unwrap();
            let accounts =
                from_processed_account_to_account_proof(account.clone(), None, state_root);
            for one_account in accounts {
                is_valid_acc = one_account.verify(state_root);
            }
        }
        if is_valid_acc {
            sp1_zkvm::io::commit_slice(&[1]);
        } else {
            sp1_zkvm::io::commit_slice(&[0]);
        }
    } else {
        sp1_zkvm::io::commit_slice(&[0]);
    }
}
