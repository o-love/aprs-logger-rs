use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

pub fn start_aprs_is_stream<A: ToSocketAddrs>(addr: A, call_sign: &str, passwd: &str) -> std::io::Result<TcpStream> {
    let mut aprsis_stream = TcpStream::connect(addr)?;

    initialize_aprs_is_stream(&mut aprsis_stream, call_sign, passwd)?;

    Ok(aprsis_stream)
}

pub fn start_default_aprs_is_stream() -> std::io::Result<TcpStream> {
    start_aprs_is_stream(
        "rotate.aprs.net:10152",
        "AE5PL-TS",
        "-1"
    )
}

fn initialize_aprs_is_stream(tcp_stream: &mut TcpStream, call_sign: &str, passwd: &str) -> std::io::Result<()> {
    let login = format!("user {call_sign} pass {passwd} vers t1 1.2 TCP\r\n");

    let mut count_nl = 1;
    while count_nl > 0 {
        let mut buf: [u8; 1] = [0x00; 1];

        tcp_stream.read_exact(&mut buf)?;

        print!("{}", buf[0] as char);

        if buf[0] == b'\n' {
            count_nl -= 1;
        }
    }

    tcp_stream.write_all(login.as_bytes())?;
    tcp_stream.flush()?;

    Ok(())
}