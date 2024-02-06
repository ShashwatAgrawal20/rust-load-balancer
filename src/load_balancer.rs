mod server_list_parser;

use std::io::{Read, Write};
use std::net::TcpStream;
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
    mut client_stream: std::net::TcpStream,
    servers: &Arc<Mutex<LoadBalancer>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut lb = servers.lock().unwrap();
    let server_addr = lb.next_server();

    let addr = server_addr.parse::<std::net::SocketAddr>()?;
    let mut server_stream = TcpStream::connect(addr)?;

    let mut client_buffer = [0; 1024];
    let bytes_read = client_stream.read(&mut client_buffer)?;
    server_stream.write(&client_buffer[..bytes_read])?;

    let mut server_buffer = [0; 1024];
    let bytes_read = server_stream.read(&mut server_buffer)?;
    client_stream.write(&server_buffer[..bytes_read])?;

    Ok(())
}
