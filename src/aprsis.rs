use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};

pub fn start_aprs_is_stream<A: ToSocketAddrs>(addr: A, call_sign: &str, passwd: &str) -> std::io::Result<TcpStream> {
    let mut aprsis_stream = TcpStream::connect(addr)?;
    
    let login = format!("user {call_sign} pass {passwd} vers t1 1.2 TCP");
    
    aprsis_stream.write_all(login.as_bytes())?;
    
    Ok(aprsis_stream)
}

pub fn start_default_aprs_is_stream() -> std::io::Result<TcpStream> {
    start_aprs_is_stream(
        "rotate.aprs.net:10152",
        "AE5PL-TS",
        "-1"
    )
}

