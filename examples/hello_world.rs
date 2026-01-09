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
use tokio_util::sync::CancellationToken;

use std::error::Error;
use std::thread;
use std::time::{Duration, Instant};

#[tokio::main]
pub async fn main() {
    let token = CancellationToken::new();
    let child = token.child_token();

    // Canceller thread
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        println!("cancelling...");
        child.cancel();
    });

    let mut max_gap = Duration::ZERO;
    let mut last = Instant::now();

    // Tight spin loop
    while !token.is_cancelled() {
        let now = Instant::now();
        let gap = now - last;
        if gap > max_gap {
            max_gap = gap;
        }
        last = now;
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    // for _ in 0..4 {
    //     let t = token.clone();
    //     thread::spawn(move || {
    //         while !t.is_cancelled() {
    //             std::hint::spin_loop();
    //         }
    //     });
    // }

    println!("Max observed gap: {:?}", max_gap);
}
