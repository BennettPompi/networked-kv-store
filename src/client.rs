use crate::arg_parse::{Config, ConnectionString};

use std::io::{self, BufReader};
use std::io::{BufRead, Write};

use std::net::TcpStream;

pub fn connect(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut write_stream = TcpStream::connect(config.get_connection_string())?;
    let read_stream = write_stream.try_clone()?;
    let mut reader = BufReader::new(read_stream);

    let mut input = String::new();
    let mut output = String::new();
    loop {
        // read user input
        io::stdin().read_line(&mut input)?;
        if input.starts_with("/quit") {
            break;
        }

        write_stream.write(input.as_bytes())?;
        write_stream.flush()?;
        reader.read_line(&mut output)?;
        input.clear();
        println!("{}", output.trim());
        output.clear();
    }
    Ok(())
}
