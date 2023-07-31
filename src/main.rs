use std::net::TcpListener;

use newsltr::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listner = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let address = listner.local_addr().unwrap();
    println!("Server launched on address: {}", address);
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    run(listner)?.await
}
