// #![feature(ip_in_core)]

pub mod errors;
pub mod traits;
pub mod helpers;
pub mod message;


use message::magic_bytes::Network;

// Size constants for version 70015
pub const COMMAND_SIZE: usize = 24;
pub const COMMAND_NAME_SIZE: usize = 12;
pub const START_STRING_SIZE: usize = 4;
pub const PAYLOAD_SIZE_SIZE: usize = 4;
pub const CHECKSUM_SIZE: usize = 4;
pub const MAX_PAYLOAD_SIZE: usize = 32 * 1024 * 1024;

pub const NETWORK: Network = Network::Mainnet;