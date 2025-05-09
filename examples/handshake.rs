// #![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
extern crate std;
extern crate tokio;
use std::{
    net::SocketAddr,
    println,
    str,
};
use core::net::{
    IpAddr,
    Ipv4Addr,
    Ipv6Addr,
};

use alloc::{
    vec::Vec,
    boxed::Box,
};
use futures::future::select_all;

use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter, self},
    net::{
        lookup_host, 
        TcpStream
    },
    // time::timeout,
};
use p2p_handshake::{
    protocol_builder::PayloadBuilder,
    errors::{
        self,
        ErrorSide::*
    },
    message::{
        payload::{
            VersionPayload,
        },
        header::{
            MessageHeader,
            HEADER_SIZE,
        },
    },
    COMMAND_SIZE, EMPTY_VERSION_SIZE, CUSTOM_VERSION_SIZE,
    traits::{
        EndianWrite,
        EndianRead,
        Builder,
    },
    helpers::{
        le_checksum,
        be_checksum,
        long_checksum,
    },
};

#[tokio::main]
async fn main() -> Result<(), errors::ErrorSide> {
    let resolved_addrs: Vec<_> = lookup_host(("seed.bitcoin.sipa.be", 8333)).await?.collect();
    let mut streams: Vec<_> = resolved_addrs
        .into_iter()
        .take(20)
        .map(|x| version_handshake(x))
        .map(Box::pin)
        .collect();

    while !streams.is_empty() {
        match select_all(streams).await {
            (Ok(payload), _index, remaining) => {
                #[cfg(debug_assertions)]
                println!("Received Payload Length: Index {:?} Size {:?} \n {:?}", _index, payload.len(), payload);
                if payload.len() > 0 {
                    return Err(PayloadSizeMismatch(payload.len()));
                }
                streams = remaining;
            },
            (Err(e), _index, remaining) => {
                #[cfg(debug_assertions)]
                println!("Error : {}", e);
                streams = remaining;
            },
        };
    }

    Ok(())
}

async fn version_handshake(target: SocketAddr) -> Result<Vec<u8>, Box<dyn errors::Error>> {
    println!("Resolving for {:?}", target);
    let mut stream = TcpStream::connect(target).await?;
    println!("From {:?}", stream.local_addr()?.ip());
    let (mut rx, mut tx) = stream.split();
    let target = match target {
        SocketAddr::V4(v4_address) => {
            v4_address.ip().to_ipv6_mapped().octets()
        },
        SocketAddr::V6(v6_address) => {
            v6_address.ip().octets()
        }
    };
    let payload = PayloadBuilder::<VersionPayload>::init()
        .with_addr_recv(&target)?
        .with_addr_from(&Ipv4Addr::new(0,0,0,0).to_ipv6_mapped().octets())?
        .with_addr_from_port(0)?
        .build();
    #[cfg(debug_assertions)]
    println!("Default Payload {:?}", payload);
    let version_header = MessageHeader::version(payload.to_be_bytes())?.to_be_bytes_with_payload(&payload.to_be_bytes())?;
    let verack_header = MessageHeader::verack();
    let mut version_header_with_payload = [0_u8; 122]; // 24 + 98
    version_header_with_payload[..COMMAND_SIZE].copy_from_slice(&version_header);
    assert_eq!(payload.to_be_bytes().len(), CUSTOM_VERSION_SIZE);
    version_header_with_payload[COMMAND_SIZE..].copy_from_slice(&payload.to_be_bytes());
    //#[cfg(debug_assertions)]
    println!("Bytes to send {:?}", version_header_with_payload);
    println!("Bytes to send size {:?}", version_header_with_payload.len());
    let _ = stream.write_all(&version_header_with_payload).await?;
    // read data from stream
    let mut buffer = BufWriter::new(BufReader::new(stream));
    let checked = check_bufread("first round", &mut buffer).await?;
    //println!("-----------------------------------------------------");
    //println!("to write {:?}", verack_header.to_be_bytes());
    //let _ = buffer.get_mut().write_all(&verack_header.to_be_bytes()).await?;
    //let checked_b = check_bufread("second round", &mut buffer).await?;
    Ok(checked)
}

async fn check_bufread(label: &str, mut payload: &mut BufWriter<BufReader<TcpStream>>) -> Result<Vec<u8>, Box<dyn errors::Error>> {
    println!("Incoming payload ... : {:?}", payload);
    let mut header: [u8; HEADER_SIZE] = [0u8; HEADER_SIZE];
    payload.read_exact(&mut header).await?;
    println!("Header : {:?}", header);
    
    /*
    let message_header = MessageHeader::from_le_bytes(header);
    let payload_size = u32::from_le_bytes(message_header.payload_size);
    #[cfg(debug_assertions)]
    println!("Check payload_size {:?} {:?}", payload_size, payload);
    #[cfg(debug_assertions)]
    println!("Received Header for label {} {:?}", label, message_header);
    */
    
    let mut buf = Vec::new(); //with_capacity(payload_size as usize);
    payload.read_to_end(&mut buf).await?;
    println!("Check buffer size {:?} {:?}", buf.len(), buf);
    let long_checksum: Vec<u8> = long_checksum(&buf);
    
    #[cfg(debug_assertions)]
    println!("With payload for label {} {:?}", label, buf);
    #[cfg(debug_assertions)]
    println!("With checksum for label {} {:?}", label, long_checksum);

    let my_bytes = payload.fill_buf().await?;
    Ok(my_bytes.to_vec())
}