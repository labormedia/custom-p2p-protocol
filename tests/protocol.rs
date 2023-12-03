use p2p_handshake::protocol;

#[test]
fn command_size() {
    let nonce = [0,0,1,0,0,1,0,0];
    let mut command_bytes = protocol::Command::Ping(nonce).to_bytes();
    assert_eq!(command_bytes.len(), 12);
}