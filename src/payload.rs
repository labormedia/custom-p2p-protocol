// Opaque types

#[derive(Default, Clone)]
pub struct VersionPayload {
    version: [u8; 4],
    services: [u8; 8],
    timestamp: [u8; 8],
    addr_recv: [u8; 26],
    // Fields below require version ≥ 106
    addr_from: [u8; 26],
    nonce: [u8; 8],
    user_agent: [u8; 1], // This variable size is fixated here for code simplicity.
    start_height: [u8; 8],
    // Fields below require version ≥ 70001
    relay: [u8; 1]
}

#[derive(Default, Clone)]
pub struct PingPayload {
    pub nonce: [u8;8],
}