use std::env;
use std::fs;
use std::io::prelude::*;
use std::io;
use std::net::TcpListener;


fn main() -> io::Result<()> {
    let entry_path = match env::args().nth(1) {
        Some(p) => p,
        None    => panic!("Usage: potd-server <file>")
    };

    let entry_data = fs::read(entry_path)?;

    let listener = TcpListener::bind("0.0.0.0:17")?;
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => { stream.write(&entry_data)?; },
            Err(_e)        => { /* connection failed */ }
        }
    }

    Ok(())
}

