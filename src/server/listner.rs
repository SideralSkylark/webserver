use anyhow::Result;
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

fn handle_stream(stream: TcpStream) {
    stream.bytes().for_each(|x| println!("{:?}", x));
}

// Todo: add other checks for port (is only numbers and is not empty, then default to 80 if
// parameter is wrong)
pub fn listen(port: &str) -> Result<()> {
    let port = sanitize_port(port);

    let listner = TcpListener::bind(get_address(&port))?;

    for stream in listner.incoming() {
        handle_stream(stream?);
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
