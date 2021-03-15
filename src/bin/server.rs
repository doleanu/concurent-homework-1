use std::io::{self, Read, Write};
use std::net::TcpListener;
use std::{env, thread};

fn main() -> io::Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8081".to_string());
    let listener = TcpListener::bind(&addr)?;
    println!("Listening on: {}", addr);

    for stream in listener.incoming() {
        thread::spawn(|| -> io::Result<()> {
            let mut stream = stream?;
            let mut name = String::new();
            stream.read_to_string(&mut name)?;
            stream.write(format!("Hello, {}!", name).as_bytes())?;
            Ok(())
        });
    }

    Ok(())
}
