use std::time::{Duration, SystemTime};
use rand::prelude::*;
use crate::{
    message::network_address::{
        self,
        NetworkAddress,
        NETWORK_SERVICES,
    },
    traits::{
        EndianWrite,
        Length
    },
};

// Opaque types

#[derive(Clone)]
pub struct VersionPayload {
    version: [u8; 4],
    services: [u8; 8],
    timestamp: [u8; 8],
    // addr_recv: [u8; 26],
    // The three datum that composes addr_recv
    // are defined in NetworkAddress enum.
    addr_recv: NetworkAddress,
    // Fields below require version ≥ 106
    addr_from: [u8; 26],
    nonce: [u8; 8],
    user_agent: [u8; 1], // This variable size is fixated here for code simplicity.
    start_height: [u8; 4],
    // Fields below require version ≥ 70001
    relay: [u8; 1]
}

impl Default for VersionPayload {
    fn default() -> VersionPayload {
        let multi_address = match NetworkAddress::default() {
            NetworkAddress::Version(multi_address) => multi_address,
            NetworkAddress::NonVersion(multi_address) => multi_address,
        };
        let version = 70015_u32.to_le_bytes();
        let services: [u8; NETWORK_SERVICES] = multi_address[1].to_be_bytes().into_iter().collect::<Vec<u8>>().try_into().expect("Default not well defined.");
        let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Time System.").as_secs().to_le_bytes();
        let nonce: [u8; 8] = rand::thread_rng().gen::<u64>().to_le_bytes();
        let height = 845_684_u32.to_le_bytes();
        VersionPayload {
            version,
            services,
            timestamp,
            addr_recv: NetworkAddress::default(),
            addr_from: NetworkAddress::default().to_be_bytes().try_into().unwrap(),
            nonce,
            user_agent: [0_u8; 1],
            start_height: height,
            relay: [0_u8; 1],
        }
    }
}

impl EndianWrite for VersionPayload {
    type Output = [u8;86];
    fn to_le_bytes(&self) -> Self::Output {
        let mut buf = self.to_be_bytes();
        buf.reverse();
        buf
    }
    fn to_be_bytes(&self) -> Self::Output {
        let byte_sequence = [
            self.version.len(), 
            self.services.len(), 
            self.timestamp.len(), 
            self.addr_recv.len(),
            self.addr_from.len(),
            self.nonce.len(),
            self.user_agent.len(),
            self.start_height.len(),
            self.relay.len(),
        ];     
        let total_sequence: usize = byte_sequence.iter().sum();
        assert_eq!(total_sequence, 86); // This is hardcoded at this stage for convenience. TODO: implement dynamic size
        let mut buf = [0;86];
        let mut start = 0;
        let mut end = start + byte_sequence[0];
        buf[start..end].copy_from_slice(&self.version);
        start = start + byte_sequence[0];
        end = start + byte_sequence[1];
        buf[start..end].copy_from_slice(&self.services);
        start = start + byte_sequence[1];
        end = start + byte_sequence[2];
        buf[start..end].copy_from_slice(&self.timestamp);
        start = start + byte_sequence[2];
        end = start + byte_sequence[3];
        buf[start..end].copy_from_slice(&self.addr_recv.to_be_bytes());
        start = start + byte_sequence[3];
        end = start + byte_sequence[4];
        buf[start..end].copy_from_slice(&self.addr_from);
        start = start + byte_sequence[4];
        end = start + byte_sequence[5];
        buf[start..end].copy_from_slice(&self.nonce);
        start = start + byte_sequence[5];
        end = start + byte_sequence[6];
        buf[start..end].copy_from_slice(&self.user_agent);
        start = start + byte_sequence[6];
        end = start + byte_sequence[7];
        buf[start..end].copy_from_slice(&self.start_height);
        start = start + byte_sequence[7];
        end = start + byte_sequence[8];
        buf[start..end].copy_from_slice(&self.relay);
        buf
    }
}

#[derive(Default, Clone)]
pub struct PingPayload {
    pub nonce: [u8;8],
}