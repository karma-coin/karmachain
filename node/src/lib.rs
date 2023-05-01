//! Responsible for running the blockchain network by coordinating the activities of the network's
//! participants, validating incoming transactions, and creating new blocks based on Runtime.
//!
//! Also provide helpful cli features for running node, syncing node, createing chain specification
//! file, creating keypairs, etc.

mod benchmarking;
pub mod chain_spec;
pub mod cli;
pub mod command;
pub mod rpc;
pub mod service;
