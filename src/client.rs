use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server_addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:22".to_string());
    let mut stream = TcpStream::connect(server_addr).await?;

    // Write some data.
    stream.write_all(b"hello world!").await?;

    Ok(())
}