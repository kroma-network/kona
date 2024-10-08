//! Test Utilities for derive traits

use crate::{
    block::OpBlock,
    errors::{BlobProviderError, PipelineError, PipelineResult},
    traits::{
        AsyncIterator, BlobProvider, ChainProvider, DataAvailabilityProvider, L2ChainProvider,
    },
};
use alloc::{boxed::Box, sync::Arc, vec, vec::Vec};
use alloy_consensus::{Header, Receipt, TxEnvelope};
use alloy_eips::eip4844::Blob;
use alloy_primitives::{Address, Bytes, B256};
use anyhow::Result;
use async_trait::async_trait;
use core::fmt::Debug;
use hashbrown::HashMap;
use kona_primitives::IndexedBlobHash;
use op_alloy_genesis::{RollupConfig, SystemConfig};
use op_alloy_protocol::{BlockInfo, L2BlockInfo};

/// Mock data iterator
#[derive(Debug, Default, PartialEq)]
pub struct TestIter {
    /// Holds open data calls with args for assertions.
    pub(crate) open_data_calls: Vec<(BlockInfo, Address)>,
    /// A queue of results to return as the next iterated data.
    pub(crate) results: Vec<PipelineResult<Bytes>>,
}

#[async_trait]
impl AsyncIterator for TestIter {
    type Item = Bytes;

    async fn next(&mut self) -> PipelineResult<Self::Item> {
        self.results.pop().unwrap_or(Err(PipelineError::Eof.temp()))
    }
}

/// Mock data availability provider
#[derive(Debug, Default)]
pub struct TestDAP {
    /// The batch inbox address.
    pub batch_inbox_address: Address,
    /// Specifies the stage results the test iter returns as data.
    pub(crate) results: Vec<PipelineResult<Bytes>>,
}

#[async_trait]
impl DataAvailabilityProvider for TestDAP {
    type Item = Bytes;
    type DataIter = TestIter;

    async fn open_data(&self, block_ref: &BlockInfo) -> PipelineResult<Self::DataIter> {
        // Construct a new vec of results to return.
        let results = self
            .results
            .iter()
            .map(|i| i.as_ref().map_or_else(|_| Err(PipelineError::Eof.temp()), |r| Ok(r.clone())))
            .collect::<Vec<PipelineResult<Bytes>>>();
        Ok(TestIter { open_data_calls: vec![(*block_ref, self.batch_inbox_address)], results })
    }
}

/// A mock chain provider for testing.
#[derive(Debug, Clone, Default)]
pub struct TestChainProvider {
    /// Maps block numbers to block information using a tuple list.
    pub blocks: Vec<(u64, BlockInfo)>,
    /// Maps block hashes to header information using a tuple list.
    pub headers: Vec<(B256, Header)>,
    /// Maps block hashes to receipts using a tuple list.
    pub receipts: Vec<(B256, Vec<Receipt>)>,
    /// Maps block hashes to transactions using a tuple list.
    pub transactions: Vec<(B256, Vec<TxEnvelope>)>,
}

impl TestChainProvider {
    /// Insert a block into the mock chain provider.
    pub fn insert_block(&mut self, number: u64, block: BlockInfo) {
        self.blocks.push((number, block));
    }

    /// Insert a block with transactions into the mock chain provider.
    pub fn insert_block_with_transactions(
        &mut self,
        number: u64,
        block: BlockInfo,
        txs: Vec<TxEnvelope>,
    ) {
        self.blocks.push((number, block));
        self.transactions.push((block.hash, txs));
    }

    /// Insert receipts into the mock chain provider.
    pub fn insert_receipts(&mut self, hash: B256, receipts: Vec<Receipt>) {
        self.receipts.push((hash, receipts));
    }

    /// Insert a header into the mock chain provider.
    pub fn insert_header(&mut self, hash: B256, header: Header) {
        self.headers.push((hash, header));
    }

    /// Clears headers from the mock chain provider.
    pub fn clear_headers(&mut self) {
        self.headers.clear();
    }

    /// Clears blocks from the mock chain provider.
    pub fn clear_blocks(&mut self) {
        self.blocks.clear();
    }

    /// Clears receipts from the mock chain provider.
    pub fn clear_receipts(&mut self) {
        self.receipts.clear();
    }

    /// Clears all blocks and receipts from the mock chain provider.
    pub fn clear(&mut self) {
        self.clear_blocks();
        self.clear_receipts();
        self.clear_headers();
    }
}

#[async_trait]
impl ChainProvider for TestChainProvider {
    type Error = anyhow::Error;

    async fn header_by_hash(&mut self, hash: B256) -> Result<Header> {
        if let Some((_, header)) = self.headers.iter().find(|(_, b)| b.hash_slow() == hash) {
            Ok(header.clone())
        } else {
            Err(anyhow::anyhow!("Header not found"))
        }
    }

    async fn block_info_by_number(&mut self, _number: u64) -> Result<BlockInfo> {
        if let Some((_, block)) = self.blocks.iter().find(|(n, _)| *n == _number) {
            Ok(*block)
        } else {
            Err(anyhow::anyhow!("Block not found"))
        }
    }

    async fn receipts_by_hash(&mut self, _hash: B256) -> Result<Vec<Receipt>> {
        if let Some((_, receipts)) = self.receipts.iter().find(|(h, _)| *h == _hash) {
            Ok(receipts.clone())
        } else {
            Err(anyhow::anyhow!("Receipts not found"))
        }
    }

    async fn block_info_and_transactions_by_hash(
        &mut self,
        hash: B256,
    ) -> Result<(BlockInfo, Vec<TxEnvelope>)> {
        let block = self
            .blocks
            .iter()
            .find(|(_, b)| b.hash == hash)
            .map(|(_, b)| *b)
            .ok_or_else(|| anyhow::anyhow!("Block not found"))?;
        let txs = self
            .transactions
            .iter()
            .find(|(h, _)| *h == hash)
            .map(|(_, txs)| txs.clone())
            .unwrap_or_default();
        Ok((block, txs))
    }
}

/// A mock blob provider for testing.
#[derive(Debug, Clone, Default)]
pub struct TestBlobProvider {
    /// Maps block hashes to blob data.
    pub blobs: HashMap<B256, Blob>,
}

impl TestBlobProvider {
    /// Insert a blob into the mock blob provider.
    pub fn insert_blob(&mut self, hash: B256, blob: Blob) {
        self.blobs.insert(hash, blob);
    }

    /// Clears blobs from the mock blob provider.
    pub fn clear(&mut self) {
        self.blobs.clear();
    }
}

#[async_trait]
impl BlobProvider for TestBlobProvider {
    type Error = BlobProviderError;

    async fn get_blobs(
        &mut self,
        _block_ref: &BlockInfo,
        blob_hashes: &[IndexedBlobHash],
    ) -> Result<Vec<Blob>, Self::Error> {
        let mut blobs = Vec::new();
        for blob_hash in blob_hashes {
            if let Some(data) = self.blobs.get(&blob_hash.hash) {
                blobs.push(*data);
            }
        }
        Ok(blobs)
    }
}

/// An [L2ChainProvider] implementation for testing.
#[derive(Debug, Default)]
pub struct TestL2ChainProvider {
    /// Blocks
    pub blocks: Vec<L2BlockInfo>,
    /// Short circuit the block return to be the first block.
    pub short_circuit: bool,
    /// Blocks
    pub op_blocks: Vec<OpBlock>,
    /// System configs
    pub system_configs: HashMap<u64, SystemConfig>,
}

impl TestL2ChainProvider {
    /// Creates a new [MockBlockFetcher] with the given origin and batches.
    pub const fn new(
        blocks: Vec<L2BlockInfo>,
        op_blocks: Vec<OpBlock>,
        system_configs: HashMap<u64, SystemConfig>,
    ) -> Self {
        Self { blocks, short_circuit: false, op_blocks, system_configs }
    }
}

#[async_trait]
impl L2ChainProvider for TestL2ChainProvider {
    type Error = anyhow::Error;

    async fn l2_block_info_by_number(&mut self, number: u64) -> Result<L2BlockInfo> {
        if self.short_circuit {
            return self.blocks.first().copied().ok_or_else(|| anyhow::anyhow!("Block not found"));
        }
        self.blocks
            .iter()
            .find(|b| b.block_info.number == number)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Block not found"))
    }

    async fn block_by_number(&mut self, number: u64) -> Result<OpBlock> {
        self.op_blocks
            .iter()
            .find(|p| p.header.number == number)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("L2 Block not found"))
    }

    async fn system_config_by_number(
        &mut self,
        number: u64,
        _: Arc<RollupConfig>,
    ) -> Result<SystemConfig> {
        self.system_configs
            .get(&number)
            .ok_or_else(|| anyhow::anyhow!("System config not found"))
            .cloned()
    }
}
