### Building blocks

- [ ] MMR validation
- [ ] Header verification - MMR proof
- [ ] Account verification - MPT proof
- [ ] Storage verification - MPT proof
- [ ] Tx / Receipt verification - MPT proof

### Build & Run

```sh
cd program && cargo prove build
```

```sh
cd script && cargo run --release -- --execute
```

This will execute the program and display the output.

### Generate a Core Proof

To generate a core proof for your program:

```sh
cd script
cargo run --release -- --prove
```
