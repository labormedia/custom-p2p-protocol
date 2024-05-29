use p2p_handshake::{
    EndianWrite,
    Command,
    START_STRING_SIZE,
    COMMAND_NAME_SIZE,
    PAYLOAD_SIZE_SIZE,
    CHECKSUM_SIZE,
    COMMAND_SIZE,
    payload::{
        PingPayload,
        VersionPayload,
    },
    helpers::to_bytes_from_slice,
    helpers::to_hex_string_from_slice,
    helpers::long_checksum,
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
        nonce: [0,1,0,0,0,0,0,0] // non palindromic
    };
    let a: [u8; COMMAND_NAME_SIZE] = 
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
        nonce: [0,1,0,0,0,0,0,0] // non palindromic
    };
    let a: [u8; COMMAND_NAME_SIZE] = 
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

#[test]
#[should_panic]
fn ping_message_header_polymorphism_negative() {
    let ping_payload = PingPayload {
        nonce: [0,1,0,0,0,0,0,0] // non palidromic
    };
    let a: [u8; COMMAND_NAME_SIZE] = 
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
fn ping_message_header_polymorphism_positive() {
    let ping_payload = PingPayload {
        nonce: [0,1,0,0,0,0,0,0] // non palindromic
    };
    let a: [u8; COMMAND_NAME_SIZE] = 
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

#[test]
fn hard_coded_message() {
    let binding = "01000000".to_owned() +
        "81cd02ab7e569e8bcd9317e2fe99f2de44d49ab2b8851ba4a308000000000000" +
        "e320b6c2fffc8d750423db8b1eb942ae710e951ed797f7affc8892b0f1fc122b" +
        "c7f5d74d" +
        "f2b9441a" +
        "42a14695";
    let header_bytes: Vec<u8> = to_bytes_from_slice(&binding);
    let binding_as_bytes: &[u8] = binding.as_bytes();
    assert_eq!(binding_as_bytes.len(), 160);
    assert_eq!(header_bytes.len(), 80);

    let long_hash = long_checksum(&header_bytes);
    assert_eq!(long_hash.len(), 32);

    let hex : String = to_hex_string_from_slice(&long_hash);
    assert_eq!(hex, "1dbd981fe6985776b644b173a4d0385ddc1aa2a829688d1e0000000000000000");
}
