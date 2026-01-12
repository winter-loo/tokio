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

use tokio_util::sync::CancellationToken;

use std::thread;
use std::time::{Duration, Instant};

#[tokio::main]
pub async fn main() {
    let token = CancellationToken::new();
    let token1 = token.clone();

    // Canceller thread
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        println!("cancelling...");
        token1.cancel();
    });

    // Tight spin loop
    while !token.is_cancelled() {}

    let bg = (0..8)
        .map(|_| {
            let t = token.clone();
            thread::spawn(move || {
                let mut max_gap = Duration::ZERO;
                let mut last = Instant::now();
                while !t.is_cancelled() {
                    let now = Instant::now();
                    let gap = now - last;
                    if gap > max_gap {
                        max_gap = gap;
                    }
                    last = now;
                }
                max_gap
            })
        })
        .collect::<Vec<_>>();

    let max_gap = bg.into_iter().map(|t| t.join().unwrap()).max().unwrap();

    println!("Max observed gap: {:?}", max_gap);
}
