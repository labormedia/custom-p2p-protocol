use p2p_handshake::{
    EndianWrite,
    Command,
    START_STRING_SIZE,
    COMMAND_NAME_SIZE,
    PAYLOAD_SIZE_SIZE,
    CHECKSUM_SIZE,
    COMMAND_SIZE,
    payload::{
        PingPayload
    },
};

#[test]
fn check_command_size() {
    let total_size = START_STRING_SIZE + COMMAND_NAME_SIZE + PAYLOAD_SIZE_SIZE + CHECKSUM_SIZE;
    assert_eq!(total_size, COMMAND_SIZE)
}

#[test]
fn default_ping_command_size() {
    let ping_payload: PingPayload = Default::default();
    let command_bytes = Command::Ping(ping_payload).to_le_bytes();
    assert_eq!(command_bytes.len(), COMMAND_NAME_SIZE);
}

#[test]
#[should_panic]
fn command_polymorphism_negative() {
    let ping_payload = PingPayload {
        nonce: [0,1,0,0,0,0,0,0]
    };
    let a: [u8; 12] = 
        Command::Ping(ping_payload.clone())
            .to_le_bytes()
            .into_iter()
            // .rev()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
    assert_eq!(
        Command::Ping(ping_payload)
            .to_be_bytes(), 
        a
    );
}

#[test]
fn command_polymorphism_positive() {
    let ping_payload = PingPayload {
        nonce: [0,1,0,0,0,0,0,0]
    };
    let a: [u8; 12] = 
        Command::Ping(ping_payload.clone())
            .to_le_bytes()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
    assert_eq!(
        Command::Ping(ping_payload)
            .to_be_bytes(), 
        a
    );
}