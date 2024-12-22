use std::io::Read;
use aprs_logger_rs::{aprsis};
use aprs_logger_rs::aprsis::start_default_aprs_is_stream;
use aprs_logger_rs::stream_processor::{process_stream, StreamIterator, StreamProcessor};

fn main() {
    println!("Hello, world!");
    
    let mut tcp_stream = start_default_aprs_is_stream().unwrap();
    
    println!("tcp stream started");
    
    loop {
        let mut buf: [u8; 1] = [0x00; 1];

        tcp_stream.read_exact(&mut buf).unwrap();

        print!("{}", buf[0] as char);
    }

    //let input_processor = |line: &str| Some(line.trim().to_string());

    //let text_stream = process_stream(tcp_stream, input_processor);
    
    //for line in text_stream {
    //    println!("{}", line.unwrap());
    //}
}
