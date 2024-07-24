
# How to run the programs

## Natively Running client

Run the client with the pre-image server to generate pre-images.json.

```shell
$ cd bin
$ just run-client-native <L2_BLOCK_NUM> <L1_RPC_URL> <L1_BEACON_URL> <L2_RPC_URL> <L2_NODE_RPC_URL> <L1_END_BLOCK_NUM> <VERBOSITY>

# Example
# VERBOSITY: The Rust debug level is determined by the number of "v"s.
$ just run-client-native 4178 http://localhost:8545 http://localhost:5052 http://localhost:9545 http://localhost:7545 485 -vv
$ ls preimages
4178_preimages.json
```

Once the preimages.json file is generated, the client can be run independently.

```shell
$ just run-client-solo <L2_BLOCK_NUM> <L1_END_BLOCK_NUM> <VERBOSITY>

# Example
just run-client-solo 4178 485 -vv
```

## Proving proof with SP1 ZKVM

First, build the guest program, which has the same logic as the client, using the SP1 toolchain.
Originally, the guest program should be automatically built when running the script, but currently, it needs to be built manually.

```shell
$ cd bin/zkvm/program
$ cargo prove build

$ ls elf
riscv32im-succinct-zkvm-elf
```

Run the script to generate the zkvm proof. For this to run, a preimages.json file corresponding to the block number is required.

```shell
$ cd bin/zkvm/script
$ cargo run --release -- --target_l2_height <L2_BLOCK_NUM>

# Example
$ cargo run --release -- --target_l2_height <4178>
```
