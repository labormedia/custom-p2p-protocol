use crate::EndianWrite;

pub enum Network {
    Mainnet,
    Testnet,
}

const MAINNET_LE: [u8;4] = [0xf9, 0xbe, 0xb4, 0xd9];
const TESTNET_LE: [u8;4] = [0x0b, 0x11, 0x09, 0x07];

impl EndianWrite for Network {
    type Array = [u8;4];
    fn to_le_bytes(&self) -> Self::Array {
        match self {
            Network::Mainnet => {
                MAINNET_LE
            },
            Network::Testnet => {
                TESTNET_LE
            }
        }
    }
    fn to_be_bytes(&self) -> Self::Array {
        let mut reversed = self.to_le_bytes().clone();
        reversed.reverse();
        reversed
    }
}

#[test]
#[should_panic]
fn magic_bytes_polymorphism_negative() {
    let a: [u8;4] = Network::Mainnet
        .to_le_bytes()
        .into_iter()
        // .rev()
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    assert_eq!(
        Network::Mainnet
            .to_be_bytes(), 
        a
    );
}

#[test]
fn magic_bytes_polymorphism_positive() {
    let a: [u8;4] = Network::Mainnet
        .to_le_bytes()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    assert_eq!(
        Network::Mainnet
            .to_be_bytes(), 
        a
    );
}