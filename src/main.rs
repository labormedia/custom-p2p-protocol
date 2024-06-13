// #![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
extern crate std;
extern crate tokio;
use std::{
    net::SocketAddr,
    println,
    str
};
use core::net::{
    Ipv4Addr,
    Ipv6Addr,
};

use alloc::{
    vec::Vec,
    boxed::Box,
};
use futures::future::select_all;

use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::{
        lookup_host, 
        TcpStream
    },
    // time::timeout,
};
use p2p_handshake::{
    errors,
    message::{
        payload::{
            VersionPayload,
            VersionPayloadBuilder,
        },
        header::MessageHeader
    },
    COMMAND_SIZE, EMPTY_VERSION_SIZE, CUSTOM_VERSION_SIZE,
    traits::EndianWrite,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn errors::Error>> {
    let resolved_addrs: Vec<_> = lookup_host(("seed.bitcoin.sipa.be", 8333)).await?.collect();
    let mut streams: Vec<_> = resolved_addrs
        .into_iter()
        .map(version_handshake)
        .map(Box::pin)
        .collect();

    while !streams.is_empty() {
        match select_all(streams).await {
            (Ok(payload), _index, remaining) => {
                #[cfg(debug_assertions)]
                println!("Payload : {:?}", str::from_utf8(&payload));
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
    let target = match target {
        SocketAddr::V4(v4_address) => {
            v4_address.ip().to_ipv6_mapped().octets()
        },
        SocketAddr::V6(v6_address) => {
            v6_address.ip().octets()
        }
    };
    let payload = VersionPayloadBuilder::init()
        .with_addr_recv(&target)?
        .with_addr_from(&Ipv4Addr::new(0,0,0,0).to_ipv6_mapped().octets())?
        .with_addr_from_port(0)?
        .build();
    #[cfg(debug_assertions)]
    println!("Default Payload {:?}", payload);
    let version_header = MessageHeader::version(payload.to_be_bytes())?.to_be_bytes_with_payload(&payload.to_be_bytes())?;
    let mut version_header_with_payload = [0_u8; 122]; // 24 + 98
    version_header_with_payload[..COMMAND_SIZE].copy_from_slice(&version_header);
    assert_eq!(payload.to_be_bytes().len(), CUSTOM_VERSION_SIZE);
    #[cfg(debug_assertions)]
    println!("Payload {:?}", payload.to_be_bytes());
    version_header_with_payload[COMMAND_SIZE..].copy_from_slice(&payload.to_be_bytes());
    //#[cfg(debug_assertions)]
    println!("Bytes to send {:?}", version_header_with_payload);
    println!("Bytes to send size {:?}", version_header_with_payload.len());
    let _ = stream.write_all(&version_header_with_payload).await?;
    // read data from stream
    let mut buf_reader = BufReader::new(stream);
    //let rx = buf_reader.fill_buf().await?;
    let mut rx: Vec<u8> = Vec::from([0]); 
    let checked = check_bufread(buf_reader).await?;
    let rx_len = checked.len();
    // #[cfg(debug_assertions)]
    println!("Received {} bytes", rx_len);
    println!("Payload {:?}", checked);
    //drop(buf_reader);
    Ok(rx.to_vec())
}

async fn check_bufread(mut payload: BufReader<TcpStream>) -> Result<Vec<u8>, Box<dyn errors::Error>> {
    let start_string = &mut [0u8; 4];
    let command_name = &mut [0u8; 12];
    payload.read_exact(start_string).await?;
    payload.read_exact(command_name).await?;
    let payload_size = payload.read_u32_le().await?;
    let checksum = &mut [0u8; 4];
    payload.read_exact(checksum).await?;

    let start_string = start_string.to_vec();

    let checksum = checksum.to_vec();
    let mut payload_vec: Vec<u8> = Vec::new();
    // Read payload bytes
    for _ in 0..payload_size {
        payload_vec.push(payload.read_u8().await?);
    }
    Ok(checksum)
}

async fn stream_process(target: SocketAddr) -> Result<Vec<u8>, Box<dyn errors::Error>> {
    println!("Resolving for {:?}", target);
    let mut stream = TcpStream::connect(target).await?;
    let payload: [u8; 8] = [1,1,1,1,1,1,1,1];
    let ping_header = MessageHeader::ping()?.to_be_bytes_with_payload(&payload)?;
    let mut ping_header_with_payload = [0_u8; 32];
    ping_header_with_payload[..COMMAND_SIZE].copy_from_slice(&ping_header);
    ping_header_with_payload[COMMAND_SIZE..].copy_from_slice(&payload);
    #[cfg(debug_assertions)]
    println!("Bytes to send {:?}", ping_header_with_payload);
    let _ = stream.write_all(&ping_header_with_payload).await?;
    // read data from stream
    let mut buf_reader = BufReader::new(stream);
    let rx = buf_reader.fill_buf().await?;
    let rx_len = rx.len();
    // #[cfg(debug_assertions)]
    println!("Received {} bytes", rx_len);
    // println!("Payload {:?}", rx);
    let result = rx.into();
    drop(buf_reader);
    Ok(result)
}