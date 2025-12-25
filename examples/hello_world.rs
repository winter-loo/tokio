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
use std::time::Duration;

async fn handle(addr: &str) {
    match TcpStream::connect(addr).await {
        Ok(mut _stream) => {
            // read/write loop
        }
        Err(e) => eprintln!("{} failed: {}", addr, e),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let addrs = ["127.0.0.1:8080", "127.0.0.1:8081", "127.0.0.1:8082"];
    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    // let futures = addrs.iter().map(|addr| TcpStream::connect(addr));
    //
    // rt.block_on(async {
    //     let streams: Vec<TcpStream> = futures::future::try_join_all(futures).await.unwrap();
    //     println!("connected to {} servers", streams.len());
    // });

    let _guard = rt.enter();

    // these tasks will be pushed into the remote queue
    for addr in addrs {
        tokio::spawn(handle(addr));
    }

    let rt_handle = rt.handle().clone();
    std::thread::spawn(move || {
        rt_handle.spawn(async {
            println!("----- Task from thread 1");
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("----- Task 1 completed");
        });
        println!("this block_on uses CondVar to wait/notify");
        rt_handle.block_on(async {
            println!("----- runtime will sleep 4 seconds in this thread");
            println!("----- tokio:sleep will wake up through time::Driver");
            tokio::time::sleep(Duration::from_secs(4)).await;
            println!("----- runtime wakes up from 4 second sleep");
        });
    });

    println!("main thread sleeps 1 second so that the above thread gets the chance to run");
    std::thread::sleep(Duration::from_secs(1));

    rt.block_on(async {
        for i in 0..=10 {
            // these tasks will be pushed into the local queue as when `block_on`
            // runs this current thread gets a runtime(`enter_runtime`).
            tokio::spawn(async move {
                println!("+++++ in block_on {i} +++++");
            });
        }

        // when this future returns Poll::Pending, the scheduler will first retrieve
        // tasks from the local queue and then the remote queue.
        tokio::signal::ctrl_c().await.unwrap();
    });
    Ok(())
}
