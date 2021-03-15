use std::{env, io};
use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};

fn main() -> io::Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8081".to_string());
        
    let mut stream = TcpStream::connect(addr)?;
    
    print!("Enter your name: ");
    io::stdout().flush()?;

    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    
    name.truncate(name.len() - 1);
    stream.write(name.as_bytes())?;
    stream.shutdown(Shutdown::Write)?;

    let mut res = String::new();
    stream.read_to_string(&mut res)?;
    println!("{}", res);
    Ok(())
}