use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::arg_parse::{Config, ConnectionString};
use crate::message_protocol::{Request, Serializable};

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
    // let mut write_stream = stream.try_clone()?;
    let mut reader = BufReader::new(&mut stream);
    // let mut line = String::new();
    loop {
        // line.clear();
        // let res = reader.read_line(&mut line);
        let mut length_buf = [0u8; 8];
        reader.read_exact(&mut length_buf)?;
        let content_length: u64 = u64::from_be_bytes(length_buf);
        let mut content_buf = vec![0u8; content_length as usize];
        reader.read_exact(&mut content_buf);

        Request::deserialize(&content_buf);
        // TODO: Fix IO loop to work with actual app
        //
        // match res {
        //     Ok(byte_count) => {
        //         if byte_count == 0 {
        //             break;
        //         };
        //         let reply = format!("Read {byte_count} bytes. Received: {line}");
        //         println!("{}", reply);
        //         write_stream.write(format!("(server): {reply}").as_bytes())?;
        //     }
        //     Err(_) => {
        //         eprintln!("Error Reading Stream!");
        //     }
        // }
    }
    // Ok(())
}
