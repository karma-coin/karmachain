//! Responsible for running the blockchain network by coordinating the activities of the network's
//! participants, validating incoming transactions, and creating new blocks based on Runtime.
//!
//! Also provide helpful cli features for running node, syncing node, createing chain specification
//! file, creating keypairs, etc.

pub mod chain_spec;
pub mod rpc;
pub mod service;
