use crate::arg_parse::{Config, ConnectionString};
use crate::message_protocol::{Request, Serializable};

use std::io::{self, BufReader};
use std::io::{BufRead, Write};

use std::net::TcpStream;
use std::str::FromStr;

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
        // todo: clean this up it's very gross
        let req = Request::serialize(Request::from_str(&input).unwrap())?;

        write_stream.write(&req)?;
        write_stream.flush()?;
        reader.read_line(&mut output)?;
        input.clear();
        output.clear();
    }
    Ok(())
}
