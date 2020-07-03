use std::env;
use std::fs;
use std::io::prelude::*;
use std::io;
use std::net::TcpListener;


fn main() -> io::Result<()> {
    let host = env::args().nth(1).expect("Usage: potd-server <host> <file>");
    let entry_path = env::args().nth(2).expect("Usage: potd-server <host> <file>");

    let listener = TcpListener::bind(format!("{}:17", host))
        .expect("Socket listening failed");
    println!("Listening on {} port 17...", host);
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

