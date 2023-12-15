use crate::CHECKSUM_SIZE;
use sha2::{Digest, Sha256};

pub fn u32_to_le_bytes(size: u32) -> [u8; 4] {
    let b1 : u8 = ((size >> 24) & 0xff) as u8;
    let b2 : u8 = ((size >> 16) & 0xff) as u8;
    let b3 : u8 = ((size >> 8) & 0xff) as u8;
    let b4 : u8 = (size & 0xff) as u8;
    return [b4, b3, b2, b1]  // Little Endianess
}

pub fn long_checksum(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(hash);
    let hash = hasher.finalize();

    let mut hash_vector = hash.to_vec();
    hash_vector.reverse();
    hash_vector
}

pub fn le_checksum(data: &[u8]) -> [u8; CHECKSUM_SIZE] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(hash);
    let hash = hasher.finalize();

    let mut buf = [0u8; CHECKSUM_SIZE];
    buf.clone_from_slice(&hash[..CHECKSUM_SIZE]);

    [buf[3], buf[2], buf[1], buf[0]]
}

pub fn to_bytes_from_slice(str_slice: &str) -> Vec<u8> {
    (0..str_slice.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&str_slice[i..i + 2], 16).unwrap() )
        .collect()
}

pub fn to_hex_string_from_slice(data: &[u8]) -> String {
    data.iter()
        .map(|b| format!("{:02x}", b).to_string())
        .collect::<Vec<String>>()
        .join("")
}

pub fn le_concatenate<'a, T>(buffer: &'a [u8], byte_sequence: &[u8], data_sequence: &[u8]) -> &'a [u8] {
    let a = 0;
    let b = byte_sequence[0];
    buffer;
    todo!()
}

#[test]
fn check_u32_to_le_bytes_endianess() {
    let num: u32 = 42;
    assert_eq!(u32_to_le_bytes(num.try_into().unwrap()), num.to_le_bytes());
}

#[test]
fn empty_le_checksum() {
    let mut empty_checksum = le_checksum(&[]);
    empty_checksum.reverse();
    assert_eq!(empty_checksum, [0x5d, 0xf6, 0xe0, 0xe2]) // 0x5df6e0e2
}

#[test]
fn empty_long_checksum() {
    let mut empty_checksum = long_checksum(&[]);
    empty_checksum.reverse();
    assert_eq!(empty_checksum[0..4], [0x5d, 0xf6, 0xe0, 0xe2]) // 0x5df6e0e2
}

#[test]
fn block_125552() { // https://blockchair.com/bitcoin/block/125552
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
    assert_eq!(hex, "00000000000000001e8d6829a8a21adc5d38d0a473b144b6765798e61f98bd1d");
}