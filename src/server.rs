use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use std::env;
use std::error::Error;

use crate::config;

fn parse_data(buf: &Vec<u8>) {
    
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args().nth(1).unwrap_or_else(|| (config::DEFAULT_SSH_ADDR).to_string());
    let listener = TcpListener::bind(&addr).await?;

    println!("Listening on: {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;
        let (mut sock_read, mut sock_write) = socket.into_split();

        // Read thread
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            'socket_handler: loop {
                let output = match sock_read.read(&mut buf).await {
                    Ok(n) => match n {
                        0 => break 'socket_handler,
                        _ => {
                            parse_data(&buf)
                        }
                    },
                    Err(_) => break 'socket_handler
                };
                println!("{:?}", buf);
            }
        });

        // Write thread
        tokio::spawn(async move {
            match sock_write.write(b"Hello there!").await {
                Ok(n) => { dbg!("Wrote {} bytes", n); },
                Err(err) => { dbg!("{:?}", err); }
            };
        });
    }
}