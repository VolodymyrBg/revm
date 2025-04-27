//! Optimism context implementation for REVM.
//!
//! This crate provides Optimism-specific context for EVM execution, containing essential
//! execution environment variables that influence transaction processing:
//!
//! - Block environment (timestamp, blockhash, etc.)
//! - Configuration environment (chain ID, specifications, etc.)
//! - Transaction environment (gas limit, calldata, addresses, etc.)
//! - EVM context for internal state tracking during execution
//! - Journal for tracking state changes with rollback capabilities
//!
//! The context module helps Optimism's execution layer handle its unique state transition
//! requirements while remaining compatible with Ethereum's execution semantics.
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc as std;

pub use context_interface::*;

pub mod block;
pub mod cfg;
pub mod context;
pub mod evm;
pub mod journal;
pub mod local;
pub mod tx;

pub use block::BlockEnv;
pub use cfg::{Cfg, CfgEnv};
pub use context::*;
pub use evm::Evm;
pub use journal::*;
pub use local::LocalContext;
pub use tx::TxEnv;
