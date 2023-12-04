#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
extern crate std;
use std::{
    net::SocketAddr,
    println
};

use alloc::{
    vec::Vec,
    boxed::Box,
};
use futures::future::join_all;

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
    let streams = resolved_addrs.iter().map(|x| {
        stream_process(*x)
    });
    let results = join_all(streams).await;

    Ok(())
    // Err(errors::ErrorSide)?
}

async fn stream_process(target: SocketAddr) -> Result<(), Box<dyn errors::Error>> {
    println!("Resolving for {:?}", target);
    let mut stream = TcpStream::connect(target).await?;
    // let (reader, mut writer) = stream.into_split();
    // let mut command_bytes = protocol::Command::Ping([1,0,0,0,0,0,1,0]).to_bytes();
    // let ping_header = protocol::MessageHeader::ping()?.to_bytes()?;
    let verack_header = protocol::MessageHeader::verack()?.to_bytes()?;
    let _ = stream.write_all(&verack_header).await?;
    // read data from IO
    let mut buf_reader = BufReader::new(stream);
    let mut rx = buf_reader.fill_buf().await?;
    let rx_len = rx.len();
    println!("Received {} bytes", rx_len);
    println!("Payload {:?}", rx);
    drop(buf_reader);
    Ok(())
}