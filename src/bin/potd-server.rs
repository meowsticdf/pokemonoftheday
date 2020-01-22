use std::env;
use std::fs;
use std::io::prelude::*;
use std::io;
use std::net::TcpListener;


fn main() -> io::Result<()> {
    let entry_path = env::args().nth(1).expect("Usage: potd-server <file>");

    let listener = TcpListener::bind("0.0.0.0:17")
        .expect("Socket listening failed");
    println!("Listening on 0.0.0.0 port 17...");
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let entry_data = fs::read(&entry_path)
                    .expect("Unable to open file");
                stream.write(&entry_data)?;
            },
            Err(_e) => { /* connection failed */ }
        }
    }

    Ok(())
}

