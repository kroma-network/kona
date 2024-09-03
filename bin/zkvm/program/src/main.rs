//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::{sol, SolValue};
use kona_client::scenario::Scenario;
use kona_preimage::PreimageKey;
use revm::primitives::HashMap;
use alloy_primitives::B256;

extern crate alloc;
use alloc::vec::Vec;

/// The public values encoded as a tuple that can be easily deserialized inside Solidity.
type PublicValuesTuple = sol! {
    tuple(B256, B256, B256)
};

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    let prebuilt_preimage = sp1_zkvm::io::read::<HashMap<PreimageKey, Vec<u8>>>();

    let (parent_output_root, output_root, l1_end_block_hash): (B256, B256, B256) = kona_common::block_on(async move {
        let mut client = Scenario::new(Some(prebuilt_preimage)).await.unwrap();
        let (attributes, l2_safe_head_header, l1_origin_block) = client.derive().await.unwrap();
        let l1_end_block_hash = client
            .check_l1_connectivity(
                l1_origin_block.hash,
                l1_origin_block.number,
                client.boot.l1_end_number,
            )
            .await.unwrap();
        let number = client.execute_block(attributes, l2_safe_head_header.clone()).await.unwrap();

        let parent_output_root = client.compute_output_root_of(l2_safe_head_header).await?;
        let output_root = client.compute_output_root().await.unwrap();

        assert_eq!(number, client.boot.l2_claim_block);
        assert_eq!(output_root, client.boot.l2_claim);

        Ok::<(B256, B256, B256), anyhow::Error>((parent_output_root, output_root, l1_end_block_hash))
    }).unwrap();

    // Encode the public values of the program.
    let bytes = PublicValuesTuple::abi_encode(&(parent_output_root, output_root, l1_end_block_hash));

    // Commit to the public values of the program.
    sp1_zkvm::io::commit_slice(&bytes);
}