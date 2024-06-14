use crate::CHECKSUM_SIZE;
use sha2::{Digest, Sha256};

pub fn u32_to_le_bytes(size: u32) -> [u8; 4] {
    let b1 : u8 = ((size >> 24) & 0xff) as u8;
    let b2 : u8 = ((size >> 16) & 0xff) as u8;
    let b3 : u8 = ((size >> 8) & 0xff) as u8;
    let b4 : u8 = (size & 0xff) as u8;
    return [b4, b3, b2, b1]  // Little Endianess
}

pub fn u32_to_be_bytes(size: u32) -> [u8; 4] {
    let b1 : u8 = ((size >> 24) & 0xff) as u8;
    let b2 : u8 = ((size >> 16) & 0xff) as u8;
    let b3 : u8 = ((size >> 8) & 0xff) as u8;
    let b4 : u8 = (size & 0xff) as u8;
    return [b1, b2, b3, b4]  // Big Endianess
}

pub fn long_checksum(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(hash);
    let hash = hasher.finalize();

    let mut hash_vector = hash.to_vec();
    // hash_vector.reverse();
    hash_vector
}

pub fn le_checksum(data: impl AsRef<[u8]>) -> [u8; CHECKSUM_SIZE] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(hash);
    let hash = hasher.finalize();

    let mut buf = [0u8; CHECKSUM_SIZE];
    buf.clone_from_slice(&hash[..CHECKSUM_SIZE]);

    [hash[0], hash[1], hash[2], hash[3]]
}

pub fn be_checksum(data: &[u8]) -> [u8; CHECKSUM_SIZE] {
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
    //empty_checksum.reverse();
    assert_eq!(empty_checksum, [0x5d, 0xf6, 0xe0, 0xe2]) // 0x5df6e0e2
}

#[test]
fn empty_long_checksum() {
    let mut empty_checksum = long_checksum(&[]);
    // empty_checksum.reverse();
    assert_eq!(empty_checksum[0..4], [0x5d, 0xf6, 0xe0, 0xe2]) // 0x5df6e0e2
}

#[test]
fn known_string_checksum() {
    let mut checksum = long_checksum(b"hello");
    // checksum.reverse();
    let hash = to_hex_string_from_slice(&checksum);
    assert_eq!(hash, "9595c9df90075148eb06860365df33584b75bff782a510c6cd4883a419833d50");
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
    // assert_eq!(hex, "00000000000000001e8d6829a8a21adc5d38d0a473b144b6765798e61f98bd1d");
    assert_eq!(hex, "1dbd981fe6985776b644b173a4d0385ddc1aa2a829688d1e0000000000000000");
}

#[test]
fn static_le_checksum() {
    let payload_a  = [113, 17, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 132, 106, 107, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 58, 177, 140, 75, 32, 141, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 0, 0, 0, 0, 0, 0, 62, 254, 14, 194, 215, 88, 65, 159, 12, 114, 117, 115, 116, 45, 101, 120, 97, 109, 112, 108, 101, 0, 0, 0, 0, 0];
    let payload_b = [113, 17, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 211, 85, 107, 102, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 101, 109, 148, 106, 32, 141, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 127, 0, 0, 1, 32, 141, 21, 252, 44, 1, 198, 129, 100, 59, 114, 117, 115, 116, 45, 101, 120, 97, 109, 112, 108, 101, 0, 0, 0, 0, 0];
    let payload_c = [113, 17, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 204, 152, 107, 102, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 182, 69, 118, 149, 32, 141, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 0, 0, 0, 0, 0, 0, 221, 219, 130, 117, 29, 22, 51, 127, 12, 114, 117, 115, 116, 45, 101, 120, 97, 109, 112, 108, 101, 0, 0, 0, 0, 0];
    let payload_d = [113, 17, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 106, 157, 107, 102, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 99, 229, 234, 251, 32, 141, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 0, 0, 0, 0, 0, 0, 245, 72, 116, 114, 102, 5, 226, 49, 12, 114, 117, 115, 116, 45, 101, 120, 97, 109, 112, 108, 101, 0, 0, 0, 0, 0];
    
    let checksum_a = le_checksum(&payload_a);
    let checksum_b = le_checksum(&payload_b);
    let checksum_c = le_checksum(&payload_c);
    let checksum_d = le_checksum(&payload_d);
    
    assert_eq!(checksum_a, [144, 241, 160, 226]);
    assert_eq!(checksum_b, [166, 141, 49, 234]);
    assert_eq!(checksum_c, [2, 101, 15, 154]);
    assert_eq!(checksum_d, [228, 58, 92, 52]);
}