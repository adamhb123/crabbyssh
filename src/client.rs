use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::env;
use std::error::Error;

use crate::config;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let server_addr = env::args().nth(1).unwrap_or_else(|| config::DEFAULT_SSH_ADDR.to_string());
    let mut stream = TcpStream::connect(server_addr).await?;
    // Write some data.
    stream.write_all(b"hello world!").await?;
    Ok(())
}