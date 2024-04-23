use std::env;
mod tests;
mod config;
mod server;
mod client;
mod ssh;

fn main() {
    if env::args().nth(1).unwrap().to_lowercase().contains("server") {
        println!("Running server...");
        server::main().expect("Failed to run server");
    } else {
        println!("Running client...");
        client::main().expect("Failed to run client");
    }
}