use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

use crate::arg_parse::{Config, ConnectionString};

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(config.get_connection_string())?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream)?,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        }
    }
    Ok(())
}
fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    println!("Received Connection!");
    let mut write_stream = stream.try_clone()?;
    let mut reader = BufReader::new(&mut stream);
    let mut line = String::new();
    loop {
        line.clear();
        let res = reader.read_line(&mut line);
        match res {
            Ok(byte_count) => {
                if byte_count == 0 {
                    break;
                };
                let reply = format!("(server): Read {byte_count} bytes. Received: {line}");
                println!("{}", reply);
                write_stream.write(reply.as_bytes())?;
            }
            Err(_) => {
                eprintln!("Error Reading Stream!");
            }
        }
    }
    Ok(())
}
