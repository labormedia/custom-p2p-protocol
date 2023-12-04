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
    let mut streams: Vec<_> = resolved_addrs.iter().map(|x| {
        Box::pin(stream_process(*x))
    })
    .collect();
    // let results = join_all(streams).await.iter().map(|result| { 
    //     match result {
    //         Ok(payload) => println!("Payload {:?}", payload),
    //         Err(e) => println!("Error {}", e),
    //     };
    //     result
    // });

    while !streams.is_empty() {
        match select_all(streams).await {
            (Ok(payload), _index, remaining) => {
                println!("Payload : {:?}", payload);
                streams = remaining;
            },
            (Err(e), _index, remaining) => {
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
    let ping_header = protocol::MessageHeader::ping()?.to_bytes()?;
    // let verack_header = protocol::MessageHeader::verack()?.to_bytes()?;
    let _ = stream.write_all(&ping_header).await?;
    // read data from stream
    let mut buf_reader = BufReader::new(stream);
    let rx = buf_reader.fill_buf().await?;
    let rx_len = rx.len();
    println!("Received {} bytes", rx_len);
    // println!("Payload {:?}", rx);
    let result = rx.into();
    drop(buf_reader);
    Ok(result)
}