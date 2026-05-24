use anyhow::Result;
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

fn handle_stream(stream: TcpStream) {
    stream.bytes().for_each(|x| println!("{:?}", x));
}

pub fn listen() -> Result<()> {
    let listner = TcpListener::bind("127.0.0.0:80")?;

    for stream in listner.incoming() {
        handle_stream(stream?);
    }

    Ok(())
}
