mod load_balancer;

use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:6969")?;
    let servers = Arc::new(Mutex::new(load_balancer::LoadBalancer::new()?));
    println!("Server started at port 6969");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection from: {}", stream.peer_addr()?);
                let servers_clone = Arc::clone(&servers);
                thread::spawn(move || {
                    if let Err(e) = load_balancer::process_request(stream, &servers_clone) {
                        println!("Error: {}", e);
                    }
                    // println!("here we are");
                    // thread::sleep(std::time::Duration::from_secs(1));
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    Ok(())
}
