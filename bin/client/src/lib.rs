#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, missing_docs, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![no_std]

extern crate alloc;

pub mod l1;

pub mod l2;

pub mod hint;
pub use hint::HintType;

mod comms;
pub use comms::{CachingOracle, HINT_WRITER, ORACLE_READER};

mod boot;
pub use boot::{
    BootInfo, L1_END_NUMBER_KEY, L1_HEAD_KEY, L2_CHAIN_ID_KEY, L2_CLAIM_BLOCK_NUMBER_KEY,
    L2_CLAIM_KEY, L2_OUTPUT_ROOT_KEY, L2_ROLLUP_CONFIG_KEY,
};
mod devnet_rollup_config;

/// Scenario of the client program.
pub mod scenario;
