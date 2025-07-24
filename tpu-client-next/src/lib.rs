//! # Solana TPU Client Next
//! 
//! Client code to send transactions to TPU (Transaction Processing Unit).
//! 
//! ## Logging Features
//! 
//! This crate supports two mutually exclusive logging backends:
//! 
//! - **`log` feature**: Uses the `log` crate for logging (default)
//! - **`tracing` feature**: Uses the `tracing` crate for logging
//! 
//! By default, the crate uses the `log` feature. To use `tracing` instead:
//! 
//! ```toml
//! # Default behavior (uses log crate)
//! solana-tpu-client-next = "3.0.0"
//! 
//! # Explicitly specify log crate
//! solana-tpu-client-next = { version = "3.0.0", features = ["log"] }
//! 
//! # Use tracing crate instead
//! solana-tpu-client-next = { version = "3.0.0", default-features = false, features = ["tracing"] }
//! ```
//! 
//! The features are mutually exclusive to prevent conflicts between the two logging systems.

pub(crate) mod connection_worker;
pub mod connection_workers_scheduler;
pub mod send_transaction_stats;
pub mod workers_cache;
pub use crate::{
    connection_workers_scheduler::{ConnectionWorkersScheduler, ConnectionWorkersSchedulerError},
    send_transaction_stats::SendTransactionStats,
};
pub(crate) mod quic_networking;
pub(crate) use crate::quic_networking::QuicError;
pub mod leader_updater;
pub mod transaction_batch;

#[cfg(feature = "metrics")]
pub mod metrics;

// Logging abstraction module
pub(crate) mod logging;
