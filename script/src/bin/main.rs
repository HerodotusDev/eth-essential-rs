//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use clap::Parser;
use hdp_lib::mmr::Header;
use hdp_lib::mmr::HeaderInclusionProof;
use hdp_lib::mmr::MmrMeta;
use hdp_lib::mmr_keccak::MmrJson;
use sp1_sdk::{ProverClient, SP1Stdin};
use starknet_types_core::felt::Felt;

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const FIBONACCI_ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");
pub const MMR_KECCAK_FIXTURE: &str = include_str!("../../keccak-test.json");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    #[clap(long, default_value = "20")]
    n: u32,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::new();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    let fixture: MmrJson = serde_json::from_str(MMR_KECCAK_FIXTURE).unwrap();

    // let test_mmr_meta: MmrMeta = MmrMeta {
    //     root: Felt::from_hex_unchecked(
    //         "0x492627ffa5084ec078f4d461408dfaa50b504a022c5471452d598da0040c066",
    //     ),
    //     size: 13024091,
    //     peaks: vec![
    //         Felt::from_hex_unchecked(
    //             "0x262c4c9b1cb2a036924aecf563dc9952e5f8b41004310adde86f22abb793eb1",
    //         ),
    //         Felt::from_hex_unchecked(
    //             "0x10b39aed56c8f244a1df559c944ada6f12b7238f8c06a2c243ba4276b8059b0",
    //         ),
    //         Felt::from_hex_unchecked(
    //             "0x46f45f218ea3aec481f350cda528a6f9f926a2dd53dae302e2cb610e5f152c7",
    //         ),
    //         Felt::from_hex_unchecked(
    //             "0x1d52a06e6d02569893a1d842c00bb67c044be541c614e88613d7fc7187e18c1",
    //         ),
    //         Felt::from_hex_unchecked(
    //             "0x770ebf618a589c17e3dc05bda7121acbedc0b48cd25f2943dc43f395f8bf0db",
    //         ),
    //         Felt::from_hex_unchecked(
    //             "0x7263e878f7deafdc49b47da57f8594d477e572d3ac2bec27bb73860a35b1899",
    //         ),
    //         Felt::from_hex_unchecked(
    //             "0x7b9e99f008949f9ee33d2965708ac6773a57965514df6383d55de104a39ab8c",
    //         ),
    //         Felt::from_hex_unchecked(
    //             "0x28f6ccdcd38f6be6c437d100fcd62604c3293e31342a777dc37c712869ab08c",
    //         ),
    //         Felt::from_hex_unchecked(
    //             "0x13d87197fe5d6f646a57dc918dcbef210737020dca9b89537fd8718ac69da3e",
    //         ),
    //         Felt::from_hex_unchecked(
    //             "0x7eef4b790b56858c0232b494034d4c8699112d88f358209f71f02d5e93a7084",
    //         ),
    //         Felt::from_hex_unchecked(
    //             "0x25cd2f0b579c902c41ac26df96ed5b21e16a3127dce2b471973dc86eb4c099f",
    //         ),
    //         Felt::from_hex_unchecked(
    //             "0x5fdedfd0123b7461d5b3162fe82f7f3172c42fda6209415367870086f7c7918",
    //         ),
    //         Felt::from_hex_unchecked(
    //             "0x7c0a415d5a6c4c90fd2dde1b340c3be305a72aa3b758dd26b8d7b4a78b53681",
    //         ),
    //     ],
    // };

    // let test_header = Header {
    //     rlp: vec![
    //         Felt::from_hex_unchecked("0x167e6bf7a06502f9"),
    //         Felt::from_hex_unchecked("0xdca4f95cfb1ccd40"),
    //         Felt::from_hex_unchecked("0x5a13c40acf7e78d2"),
    //         Felt::from_hex_unchecked("0xacd3fd41f7a63a9f"),
    //         Felt::from_hex_unchecked("0x4dcc1da0cf7a78fc"),
    //         Felt::from_hex_unchecked("0xb585ab7a5dc7dee8"),
    //         Felt::from_hex_unchecked("0x4512d31ad4ccb667"),
    //         Felt::from_hex_unchecked("0x42a1f013748a941b"),
    //         Felt::from_hex_unchecked("0x9ff2944793d440fd"),
    //         Felt::from_hex_unchecked("0xa8fba1c9a6ae6af9"),
    //         Felt::from_hex_unchecked("0xd469c0f33747f751"),
    //         Felt::from_hex_unchecked("0xb1476d73eca0a9f1"),
    //         Felt::from_hex_unchecked("0x874d5147a3a0ecc4"),
    //         Felt::from_hex_unchecked("0x27d9569c6ab671c0"),
    //         Felt::from_hex_unchecked("0x68e67dfce96bee20"),
    //         Felt::from_hex_unchecked("0x5b109054a0f5f9e7"),
    //         Felt::from_hex_unchecked("0xabce3cd2ad9d178e"),
    //         Felt::from_hex_unchecked("0x10e5ed6dee95a472"),
    //         Felt::from_hex_unchecked("0xfbb391640a35f4e8"),
    //         Felt::from_hex_unchecked("0x7a1947a042d82f76"),
    //         Felt::from_hex_unchecked("0x6314311ffdf5cf8c"),
    //         Felt::from_hex_unchecked("0x9463a963ebe94e44"),
    //         Felt::from_hex_unchecked("0xc7f760050e6a5057"),
    //         Felt::from_hex_unchecked("0x1b9b02c41672c"),
    //         Felt::from_hex_unchecked("0x4906c46f88361c"),
    //         Felt::from_hex_unchecked("0x6c911371fa12b805"),
    //         Felt::from_hex_unchecked("0x4c1c03320051c7a2"),
    //         Felt::from_hex_unchecked("0x980481c194c40809"),
    //         Felt::from_hex_unchecked("0x7c800a28414069c0"),
    //         Felt::from_hex_unchecked("0xe298b78301017240"),
    //         Felt::from_hex_unchecked("0x7573200b25021338"),
    //         Felt::from_hex_unchecked("0x2213d6020368106"),
    //         Felt::from_hex_unchecked("0x8beb9e585402601e"),
    //         Felt::from_hex_unchecked("0xc34c08813a122656"),
    //         Felt::from_hex_unchecked("0xe04450e0418116"),
    //         Felt::from_hex_unchecked("0x27ca8e8d35900660"),
    //         Felt::from_hex_unchecked("0x49220bab610260d"),
    //         Felt::from_hex_unchecked("0x804b129191418eb8"),
    //         Felt::from_hex_unchecked("0x5a647b00c0a40be8"),
    //         Felt::from_hex_unchecked("0xe128901306e0201"),
    //         Felt::from_hex_unchecked("0x40c72b023c044626"),
    //         Felt::from_hex_unchecked("0xa0e1e228ab300827"),
    //         Felt::from_hex_unchecked("0x420f21290342200c"),
    //         Felt::from_hex_unchecked("0x8a060605538001e3"),
    //         Felt::from_hex_unchecked("0x94c211f02a7ada02"),
    //         Felt::from_hex_unchecked("0x8514e295a15d542"),
    //         Felt::from_hex_unchecked("0x288345a43586720a"),
    //         Felt::from_hex_unchecked("0xf80029022246a480"),
    //         Felt::from_hex_unchecked("0xa94cb33462985683"),
    //         Felt::from_hex_unchecked("0x4052129f03d73b01"),
    //         Felt::from_hex_unchecked("0x136a4280411314e8"),
    //         Felt::from_hex_unchecked("0x246165a86250b186"),
    //         Felt::from_hex_unchecked("0x902b4201410416d2"),
    //         Felt::from_hex_unchecked("0x742bb43302a4638a"),
    //         Felt::from_hex_unchecked("0x6322be6a48524029"),
    //         Felt::from_hex_unchecked("0x8566c6808b04068"),
    //         Felt::from_hex_unchecked("0xc90184ab66598380"),
    //         Felt::from_hex_unchecked("0x66841ce6c18380c3"),
    //         Felt::from_hex_unchecked("0xd0183d899bcf93a"),
    //         Felt::from_hex_unchecked("0x678868746567840b"),
    //         Felt::from_hex_unchecked("0x85362e31322e316f"),
    //         Felt::from_hex_unchecked("0xd51ca078756e696c"),
    //         Felt::from_hex_unchecked("0x83074fe811b1197c"),
    //         Felt::from_hex_unchecked("0x1e66107c617bfe2f"),
    //         Felt::from_hex_unchecked("0xa187759811e589a1"),
    //         Felt::from_hex_unchecked("0x887c6cadc69f7f"),
    //         Felt::from_hex_unchecked("0x8500000000000000"),
    //         Felt::from_hex_unchecked("0xedba0783f9bee02"),
    //         Felt::from_hex_unchecked("0x95a9360b3e8975ca"),
    //         Felt::from_hex_unchecked("0xc9d730d907230f1c"),
    //         Felt::from_hex_unchecked("0xe2a3470255fe1187"),
    //         Felt::from_hex_unchecked("0x883a6a29d4b69e7"),
    //         Felt::from_hex_unchecked("0xa000009204840000"),
    //         Felt::from_hex_unchecked("0xb2a26a2215758e0"),
    //         Felt::from_hex_unchecked("0x6558068195ed9b2d"),
    //         Felt::from_hex_unchecked("0x2b4b100c94d228cf"),
    //         Felt::from_hex_unchecked("0x81fd704a5ab0c188"),
    //     ],
    //     proof: HeaderInclusionProof {
    //         leaf_idx: 175968,
    //         mmr_path: vec![
    //             Felt::from_hex_unchecked(
    //                 "0x3e7f1315cad8591f8c695da7be6422314eb901b61d8987d3a4e8ea0d6d55986",
    //             ),
    //             Felt::from_hex_unchecked(
    //                 "0x727965b30883a87bf412ad16fc1cdf3e0f5ca04cfba6e12a183c9e00b9a42f4",
    //             ),
    //             Felt::from_hex_unchecked(
    //                 "0x2bb4be090166054388850e72daf0981ea183a58caf1fe168df34e3dbe8ee43f",
    //             ),
    //             Felt::from_hex_unchecked(
    //                 "0x1b4d43d2a01745806ed7164aa59bae7f658bce691245618f0c376d2bdf4b8bd",
    //             ),
    //             Felt::from_hex_unchecked(
    //                 "0x361ccc6bfc25ebc64fdfdd407970da357373985578cceb677f0714cdf7bdf87",
    //             ),
    //             Felt::from_hex_unchecked(
    //                 "0x2757794640969772a98ffabb417a53fb2df19791f98a61037064dad0d994ee8",
    //             ),
    //             Felt::from_hex_unchecked(
    //                 "0x196baba4ecbf0dc319a187e8b045966d3cd320663856a2ccd6dceee31a29308",
    //             ),
    //             Felt::from_hex_unchecked(
    //                 "0x75a6c78c085b058e8e9dbbc08707225adebd4f2b3acb67703b7feb85ddceae4",
    //             ),
    //             Felt::from_hex_unchecked(
    //                 "0x1e565ec1da0650a15d4f46f2c0eb3d4549d191f330a6f9bebed76e40e5b4ba3",
    //             ),
    //             Felt::from_hex_unchecked(
    //                 "0x00bc88770e38112723046e2718585dad9b69b271dad851212379e6acc969de6",
    //             ),
    //             Felt::from_hex_unchecked(
    //                 "0x387cf7cedc350945487fd4ee111706c44bd5a815b57f5974ce440e6dde43e24",
    //             ),
    //             Felt::from_hex_unchecked(
    //                 "0x298f99d09fc847f82a9512a7b39ccf73746e078314dbe734b8f269e1286637c",
    //             ),
    //             Felt::from_hex_unchecked(
    //                 "0x725f669ddd64e3864cf53f3e8d16b888bc2835a2bf062fa032ab0b6ba15028a",
    //             ),
    //             Felt::from_hex_unchecked(
    //                 "0x49ab2b3574a65dd6648d976b5fe990131d6e7b4b7d41a2be5366710dd17b94f",
    //             ),
    //             Felt::from_hex_unchecked(
    //                 "0x33716f3f24c1835811c678186589eb5551639a40d94e47147f57c5753c7f12d",
    //             ),
    //             Felt::from_hex_unchecked(
    //                 "0x715c7749b605209670cc17e41fe1d82e97ad445bda2a3eeca74eed3b17fd886",
    //             ),
    //             Felt::from_hex_unchecked(
    //                 "0x48ce8656d39a85f29a6493eb3881a62e965e36165537bc70b225dd5f982ca23",
    //             ),
    //             Felt::from_hex_unchecked(
    //                 "0x11ecc517bc5be2a48922a0136824d6983c3e1a95c1cd32cac135ed69b54d364",
    //             ),
    //             Felt::from_hex_unchecked(
    //                 "0x5eab4c28e04e28d5056042dd8a8cb9810cb44711e7ee6da394d2e4d2b84a680",
    //             ),
    //             Felt::from_hex_unchecked(
    //                 "0x59bbae3d1cfd74546dfe68c264341cee9f95c3b3b61eeacead3bd4cb3ae232a",
    //             ),
    //             Felt::from_hex_unchecked(
    //                 "0x546455f57f4ee848d3952148e3b94700f387ee2c36730bfeda09379ce8fa509",
    //             ),
    //             Felt::from_hex_unchecked(
    //                 "0x08808a106dc9e09c29afd24be7cee31edd9f0d27ce0a3469839ef3d09ddfb43",
    //             ),
    //         ],
    //     },
    // };

    stdin.write(&fixture.headers[0]);
    stdin.write(&fixture.meta);

    // println!("n: {}", args.n);

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(FIBONACCI_ELF, stdin).run().unwrap();
        println!("Program executed successfully.");

        println!("result: {:?}", output);

        // let (expected_a, expected_b) = fibonacci_lib::fibonacci(n);
        // assert_eq!(a, expected_a);
        // assert_eq!(b, expected_b);
        // println!("Values are correct!");

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(FIBONACCI_ELF);

        // Generate the proof
        let proof = client
            .prove(&pk, stdin)
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
