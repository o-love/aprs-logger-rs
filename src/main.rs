use std::io::Read;
use std::string::FromUtf8Error;
use aprs_logger_rs::{aprsis};
use aprs_logger_rs::aprsis::start_default_aprs_is_stream;
use aprs_logger_rs::stream_processor::{process_stream, StreamIterator, StreamProcessor};

fn main() {
    println!("Hello, world!");
    
    let mut tcp_stream = start_default_aprs_is_stream().unwrap();
    
    println!("tcp stream started");
    

    let input_processor = |line: &[u8]| {
        match String::from_utf8(line.to_vec()) {
            Ok(line) => {Some(line)}
            Err(err) => {
                eprint!("Invalid utf-8 line: ");
                
                for c in line {
                    eprint!("{}", *c as char);
                }
                eprintln!();
                
                None
            }
        }
    };

    let text_stream = process_stream(tcp_stream, input_processor);
    
    for line in text_stream {
        println!("{}", line.unwrap());
    }
}
