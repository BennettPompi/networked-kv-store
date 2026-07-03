use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::arg_parse::{Config, ConnectionString};

pub fn listen(config: Config) -> Result<(), String> {
    let listener =
        TcpListener::bind(config.get_connection_string()).map_err(|e| format!("Error: {e}"))?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        }
    }
    Ok(())
}
fn handle_connection(mut stream: TcpStream) {
    println!("Received Connection!");
    let mut buf: Vec<u8> = Vec::new();
    let res = stream.read_to_end(&mut buf);
    match res {
        Ok(byte_count) => {
            let msg: &str = std::str::from_utf8(&buf).unwrap();
            println!("Read {byte_count} bytes. \nReceived: \n{msg}");
            stream.write("ACK\n".as_bytes()).unwrap();
        }
        Err(_) => {
            eprintln!("Error Reading Stream!");
        }
    }
}
