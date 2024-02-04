mod server_list_parser;

use std::sync::{Arc, Mutex};

pub struct LoadBalancer {
    servers: Vec<String>,
    index: usize,
}

impl LoadBalancer {
    pub fn new() -> Result<LoadBalancer, Box<dyn std::error::Error>> {
        let servers = server_list_parser::parse()?;
        Ok(LoadBalancer { servers, index: 0 })
    }

    fn next_server(&mut self) -> &String {
        let server = &self.servers[self.index];
        self.index = (self.index + 1) % self.servers.len();
        server
    }
}

pub fn process_request(
    stream: std::net::TcpStream,
    servers: &Arc<Mutex<LoadBalancer>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut lb = servers.lock().unwrap();
    println!("forwarding to server: {}", lb.next_server());

    Ok(())
}
