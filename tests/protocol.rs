use p2p_handshake::{
    EndianWrite,
    Command,
    START_STRING_SIZE,
    COMMAND_NAME_SIZE,
    PAYLOAD_SIZE_SIZE,
    CHECKSUM_SIZE,
    COMMAND_SIZE,
};

#[test]
fn check_command_size() {
    let total_size = START_STRING_SIZE + COMMAND_NAME_SIZE + PAYLOAD_SIZE_SIZE + CHECKSUM_SIZE;
    assert_eq!(total_size, COMMAND_SIZE)
}

#[test]
fn ping_command_size() {
    let nonce = [0,0,1,0,0,1,0,0];
    let command_bytes = Command::Ping(nonce).to_le_bytes();
    assert_eq!(command_bytes.len(), 12);
}

#[test]
#[should_panic]
fn command_polymorphism_negative() {
    let nonce = [0,1,0,0,0,0,0,0];
    let a: [u8; 12] = 
        Command::Ping(nonce)
            .to_le_bytes()
            .into_iter()
            // .rev()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
    assert_eq!(
        Command::Ping(nonce)
            .to_be_bytes(), 
        a
    );
}

#[test]
fn command_polymorphism_positive() {
    let nonce = [0,1,0,0,0,0,0,0];
    let a: [u8; 12] = 
        Command::Ping(nonce)
            .to_le_bytes()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
    assert_eq!(
        Command::Ping(nonce)
            .to_be_bytes(), 
        a
    );
}