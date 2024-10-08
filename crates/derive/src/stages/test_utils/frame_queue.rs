//! Mock types for the [FrameQueue] stage.

use crate::{
    errors::{PipelineError, PipelineResult},
    stages::FrameQueueProvider,
    traits::{OriginAdvancer, OriginProvider, ResettableStage},
};
use alloc::{boxed::Box, vec::Vec};
use alloy_primitives::Bytes;
use async_trait::async_trait;
use op_alloy_genesis::SystemConfig;
use op_alloy_protocol::BlockInfo;

/// A mock [FrameQueueProvider] for testing the [FrameQueue] stage.
#[derive(Debug, Default)]
pub struct MockFrameQueueProvider {
    /// The data to return.
    pub data: Vec<PipelineResult<Bytes>>,
    /// The origin to return.
    pub origin: Option<BlockInfo>,
}

impl MockFrameQueueProvider {
    /// Creates a new [MockFrameQueueProvider] with the given data.
    pub const fn new(data: Vec<PipelineResult<Bytes>>) -> Self {
        Self { data, origin: None }
    }

    /// Sets the origin for the [MockFrameQueueProvider].
    pub fn set_origin(&mut self, origin: BlockInfo) {
        self.origin = Some(origin);
    }
}

impl OriginProvider for MockFrameQueueProvider {
    fn origin(&self) -> Option<BlockInfo> {
        self.origin
    }
}

#[async_trait]
impl OriginAdvancer for MockFrameQueueProvider {
    async fn advance_origin(&mut self) -> PipelineResult<()> {
        Ok(())
    }
}

#[async_trait]
impl FrameQueueProvider for MockFrameQueueProvider {
    type Item = Bytes;

    async fn next_data(&mut self) -> PipelineResult<Self::Item> {
        self.data.pop().unwrap_or(Err(PipelineError::Eof.temp()))
    }
}

#[async_trait]
impl ResettableStage for MockFrameQueueProvider {
    async fn reset(&mut self, _base: BlockInfo, _cfg: &SystemConfig) -> PipelineResult<()> {
        Ok(())
    }
}
