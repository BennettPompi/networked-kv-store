use crate::arg_parse::{Config, ConnectionString};

use std::io;
use std::io::Write;

use std::net::TcpStream;

pub fn connect(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(config.get_connection_string())
        .map_err(|e| format!("Connection failed: {e}"))?;
    let mut input = String::new();
    loop {
        // read user input
        io::stdin().read_line(&mut input)?;
        if input.starts_with("/quit") {
            break;
        }

        stream.write(input.as_bytes())?;
        input.clear();
        stream.flush()?;
    }
    Ok(())
}
