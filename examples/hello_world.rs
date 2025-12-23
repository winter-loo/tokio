//! A simple client that opens a TCP stream, writes "hello world\n", and closes
//! the connection.
//!
//! To start a server that this client can talk to on port 6142, you can use this command:
//!
//!     ncat -l 6142
//!
//! And then in another terminal run:
//!
//!     cargo run --example hello_world

#![warn(rust_2018_idioms)]

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::runtime::Builder;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let rt = Builder::new_current_thread().enable_io().build().unwrap();
    println!("start run loop...1");
    rt.block_on(async {
        println!("async started...1");
        // let mut stream = TcpStream::connect("10.0.0.2:6142").await?;
        let mut stream = TcpStream::connect("127.0.0.1:6142").await.unwrap();
        println!("created stream");

        let result = stream.write_all(b"hello world\n").await;
        println!("wrote to stream; success={:?}", result.is_ok());
    });

    // the following future demostrates we have one more TOKEN_WAKEUP event
    rt.block_on(async {
        let mut i = 0;
        std::future::poll_fn(|cx| {
            // This is a computation task and once the Pending returns and
            // tokio runtime has no way to poll this future again.
            println!("i = {i}");
            if i == 1 {
                println!("...ready");
                std::task::Poll::Ready(())
            } else {
                i += 1;
                println!("...pending...");
                std::task::Poll::Pending
            }
        }).await;
    });

    Ok(())
}
