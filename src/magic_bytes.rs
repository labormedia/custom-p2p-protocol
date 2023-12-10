use crate::EndianWrite;

pub enum Network {
    Mainnet,
    Testnet3,
    Regtest,
    Signet,
    Namecoin,

}

// as documented in https://en.bitcoin.it/wiki/Protocol_documentation#Common_structures
const MAINNET: [u8;4] = [0xd9, 0xb4, 0xbe, 0xf9]; 
const TESTNET3: [u8;4] = [0x07, 0x09, 0x11, 0x0b]; 
const REGTEST: [u8; 4] = [0xda, 0xb5, 0xbf, 0xfa];
const SIGNET: [u8; 4] = [0x40, 0xcf, 0x03, 0x0a];
const NAMECOIN: [u8;4] = [0xfe, 0xb4, 0xbe, 0xf9];

impl EndianWrite for Network {
    type Output = [u8;4];
    fn to_le_bytes(&self) -> Self::Output {
        let mut buf = self.to_be_bytes().clone();
        buf.reverse();
        buf
    }
    fn to_be_bytes(&self) -> Self::Output {
        match self {
            Network::Mainnet => {
                MAINNET
            },
            Network::Testnet3 => {
                TESTNET3
            },
            Network::Regtest => {
                REGTEST
            },
            Network::Signet => {
                SIGNET
            },
            Network::Namecoin => {
                NAMECOIN
            }
        }
    }
}

// const ALL_NETWORKS_LIST = [ Network::Mainnet, Network::Testnet3, Network::Regtest, Network::Signet, Network::Namecoin];

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