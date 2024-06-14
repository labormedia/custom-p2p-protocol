use std::time::{Duration, SystemTime};
use core::net::Ipv4Addr;
use rand::prelude::*;
use crate::{
    START_STRING_SIZE,
    EMPTY_VERSION_SIZE,
    CUSTOM_VERSION_SIZE,
    USER_AGENT_SIZE,
    message::network_address::{
        self,
        NetworkAddress,
        NETWORK_SERVICES,
        DEFAULT_IPADDR,
        NETWORK_IPvXX,
        NetworkOptions,
    },
    traits::{
        EndianWrite,
        Length,
        Builder,
    },
    errors,
    protocol_builder::PayloadBuilder,
};

// Opaque types

mod version;
mod ping;

pub use version::VersionPayload;
pub use ping::PingPayload;

