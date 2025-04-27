//! Context interface for Optimism's execution environment.
//!
//! This crate defines the interface and trait boundaries for Optimism's execution context:
//!
//! - Block: Interface for block environment data (number, timestamp, etc.)
//! - Config: Interface for chain configuration (chain ID, enabled EIPs, etc.)
//! - Context: Core trait for EVM context management
//! - Journaled state: Interface for state tracking with checkpoint/rollback capability
//! - Transaction: Interface for transaction data and execution parameters
//! - Result types: Common result and error types for the execution environment
//!
//! These interfaces establish a clear boundary between the execution environment
//! implementation and consumers, enabling modularity and testing.
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc as std;

pub mod block;
pub mod cfg;
pub mod context;
pub mod journaled_state;
pub mod local;
pub mod result;
pub mod transaction;

pub use block::Block;
pub use cfg::{Cfg, CreateScheme, TransactTo};
pub use context::{ContextSetters, ContextTr};
pub use database_interface::{DBErrorMarker, Database};
pub use either;
pub use journaled_state::JournalTr;
pub use local::LocalContextTr;
pub use transaction::{Transaction, TransactionType};
