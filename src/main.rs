#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
extern crate std;
use std::{
    net::SocketAddr,
    println,
    str
};

use alloc::{
    vec::Vec,
    boxed::Box,
};
use futures::future::select_all;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{
        lookup_host, 
        TcpStream
    },
    // time::timeout,
};
use p2p_handshake::{
    errors,
    MessageHeader,
    COMMAND_SIZE,
    VersionPayload,
    EndianWrite,
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
    let payload: VersionPayload = Default::default();
    let ping_header = MessageHeader::version()?.to_le_bytes_with_payload(&payload.to_be_bytes())?;
    let mut ping_header_with_payload = [0_u8; 114]; // 24 + 90
    ping_header_with_payload[..COMMAND_SIZE].copy_from_slice(&ping_header);
    ping_header_with_payload[COMMAND_SIZE..].copy_from_slice(&payload.to_be_bytes());
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

async fn stream_process(target: SocketAddr) -> Result<Vec<u8>, Box<dyn errors::Error>> {
    println!("Resolving for {:?}", target);
    let mut stream = TcpStream::connect(target).await?;
    let payload: [u8; 8] = [1,1,1,1,1,1,1,1];
    let ping_header = MessageHeader::ping()?.to_le_bytes_with_payload(&payload)?;
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