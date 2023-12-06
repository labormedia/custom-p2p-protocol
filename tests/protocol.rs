use p2p_handshake::protocol;

#[test]
fn check_command_size() {
    let total_size = protocol::START_STRING_SIZE + protocol::COMMAND_NAME_SIZE + protocol::PAYLOAD_SIZE_SIZE + protocol::CHECKSUM_SIZE;
    assert_eq!(total_size, protocol::COMMAND_SIZE)
}

#[test]
fn ping_command_size() {
    let nonce = [0,0,1,0,0,1,0,0];
    let command_bytes = protocol::Command::Ping(nonce).to_le_bytes();
    assert_eq!(command_bytes.len(), 12);
}

#[test]
#[should_panic]
fn command_poly_negative() {
    let a: [u8; 12] = 
        protocol::Command::Ping([0,0,0,0,0,0,0,0])
            .to_le_bytes()
            .into_iter()
            // .rev()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
    assert_eq!(
        protocol::Command::Ping([0,0,0,0,0,0,0,0])
            .to_be_bytes(), 
        a
    );
}

#[test]
fn command_poly_positive() {
    let a: [u8; 12] = 
        protocol::Command::Ping([0,0,0,0,0,0,0,0])
            .to_le_bytes()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
    assert_eq!(
        protocol::Command::Ping([0,0,0,0,0,0,0,0])
            .to_be_bytes(), 
        a
    );
}