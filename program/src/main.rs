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
use hdp_lib::mmr_keccak::{verify_headers_with_mmr_peaks, Header, MmrMeta};

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    let header = sp1_zkvm::io::read::<Header>();
    let mmr = sp1_zkvm::io::read::<MmrMeta>();

    let is_valid = verify_headers_with_mmr_peaks(mmr, header).unwrap();
    if is_valid {
        sp1_zkvm::io::commit_slice(&[1]);
    }
}
