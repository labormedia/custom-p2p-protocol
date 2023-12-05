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
    io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader},
    net::{
        lookup_host, 
        TcpStream
    },
    time::timeout,
};
use p2p_handshake::{
    errors,
    protocol,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn errors::Error>> {
    let resolved_addrs: Vec<_> = lookup_host(("seed.bitcoin.sipa.be", 8333)).await?.collect();
    let mut streams: Vec<_> = resolved_addrs
        .into_iter()
        .map(stream_process)
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
    // Err(errors::ErrorSide)?
}

async fn stream_process(target: SocketAddr) -> Result<Vec<u8>, Box<dyn errors::Error>> {
    println!("Resolving for {:?}", target);
    let mut stream = TcpStream::connect(target).await?;
    let payload: [u8; 8] = [1,1,1,1,1,1,1,1];
    let ping_header = protocol::MessageHeader::ping()?.to_bytes_with_payload(&payload)?;
    let mut ping_header_with_payload = [0_u8; 32];
    ping_header_with_payload[..protocol::COMMAND_SIZE].copy_from_slice(&ping_header);
    ping_header_with_payload[protocol::COMMAND_SIZE..].copy_from_slice(&payload);
    #[cfg(debug_assertions)]
    println!("Bytes to send {:?}", ping_header_with_payload);
    let _ = stream.write_all(&ping_header_with_payload).await?;

    // writes a verack message header to stream buffer.
    // let verack_header = protocol::MessageHeader::verack()?.to_bytes()?;
    // let _ = stream.write_all(&verack_header).await?;

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