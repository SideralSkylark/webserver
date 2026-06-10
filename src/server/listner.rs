use anyhow::Result;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::http::parser;
use crate::server::router;

//maybe use a dynamic buffer to avoid overflowing
fn handle_stream(mut stream: TcpStream) -> Result<()> {
    let mut buff = [0u8; 1024];
    let size: usize = stream.read(&mut buff)?;

    let parsed_request = parser::parse_request(&mut buff[..size])?;
    let response = router::resolve(parsed_request)?;

    let response_buffer = parser::serialize_response(response);

    stream.write_all(&response_buffer)?;

    Ok(())
}

pub fn listen(port: &str) -> Result<()> {
    let port = sanitize_port(port);

    let listner = TcpListener::bind(get_address(&port))?;

    for stream in listner.incoming() {
        match stream {
            Ok(stream) => {
                handle_stream(stream)?;
            }
            Err(e) => {
                println!("could not establish connection. reason: {}", e);
            }
        }
    }

    Ok(())
}

fn get_address(port: &str) -> String {
    let loopback_address = String::from("127.0.0.1");

    format!("{}:{}", loopback_address, port)
}

fn sanitize_port(port: &str) -> String {
    if port.is_empty() || !port.chars().all(|c| c.is_ascii_digit()) {
        return String::from("80");
    }

    port.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_should_default_on_bad_input() {
        let invalid_port = "sdf";
        let empty_port = "";

        assert_eq!(sanitize_port(invalid_port), "80");
        assert_eq!(sanitize_port(empty_port), "80");
    }

    #[test]
    fn sanitize_returns_same_port_if_valid() {
        let valid_port = "1024";

        assert_eq!(sanitize_port(valid_port), "1024");
    }
}
