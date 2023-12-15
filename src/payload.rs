use crate::{
    network_address::NetworkAddress,
    traits::{
        EndianWrite,
        Length
    },
};

// Opaque types

#[derive(Default, Clone)]
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
    start_height: [u8; 8],
    // Fields below require version ≥ 70001
    relay: [u8; 1]
}

impl EndianWrite for VersionPayload {
    type Output = [u8;90];
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
        let mut buf = [0;90];
        buf[0..byte_sequence[0]].copy_from_slice(&self.version);
        buf[byte_sequence[0]..byte_sequence[1]].copy_from_slice(&self.services);
        buf[byte_sequence[1]..byte_sequence[2]].copy_from_slice(&self.timestamp);
        buf[byte_sequence[2]..byte_sequence[3]].copy_from_slice(&self.addr_recv.to_be_bytes());
        buf[byte_sequence[3]..byte_sequence[4]].copy_from_slice(&self.addr_from);
        buf[byte_sequence[4]..byte_sequence[5]].copy_from_slice(&self.nonce);
        buf[byte_sequence[5]..byte_sequence[6]].copy_from_slice(&self.user_agent);
        buf[byte_sequence[6]..byte_sequence[7]].copy_from_slice(&self.start_height);
        buf[byte_sequence[7]..byte_sequence[8]].copy_from_slice(&self.relay);
        buf
    }
}

#[derive(Default, Clone)]
pub struct PingPayload {
    pub nonce: [u8;8],
}