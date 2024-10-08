//! Contains "online" implementations for providers.

// Re-export types for the online pipeline construction.
pub use crate::{
    attributes::StatefulAttributesBuilder,
    pipeline::{DerivationPipeline, PipelineBuilder},
    sources::EthereumDataSource,
    traits::{ChainProvider, L2ChainProvider, OriginProvider, Pipeline, StepResult},
};

mod pipeline;
pub use pipeline::{
    new_online_pipeline, OnlineAttributesBuilder, OnlineAttributesQueue, OnlineDataProvider,
    OnlinePipeline,
};

mod beacon_client;
pub use beacon_client::{BeaconClient, OnlineBeaconClient};

mod alloy_providers;
pub use alloy_providers::{AlloyChainProvider, AlloyL2ChainProvider};

mod blob_provider;
pub use blob_provider::{
    BlobSidecarProvider, OnlineBlobProvider, OnlineBlobProviderBuilder,
    OnlineBlobProviderWithFallback, SimpleSlotDerivation, SlotDerivation,
};

#[cfg(any(test, feature = "test-utils"))]
pub mod test_utils;
