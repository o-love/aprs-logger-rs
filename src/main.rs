use std::fs::File;
use std::io::{BufWriter, Write};
use chrono::Utc;
use aprs_logger::aprsis::cnx::start_default_aprs_is_stream;
use aprs_logger::aprsis::processor::parse_aprs_tnc2_line;
use aprs_logger::stream_processor::{process_stream};

fn aprs_pipeline() {
    let tcp_stream = start_default_aprs_is_stream().unwrap();

    println!("tcp stream started");


    let input_processor = |line: &[u8]| {
        match parse_aprs_tnc2_line(line) {
            Ok(packet) => {Some(packet)}
            Err(err) => {
                eprint!("Invalid utf-8 line: ");

                for c in line {
                    eprint!("{}", *c as char);
                }
                eprintln!("{}", err);

                None
            }
        }
    };

    let packet_stream = process_stream(tcp_stream, input_processor);

    let filename = format!("/data/{}", Utc::now().timestamp());
    let file = File::create(filename).unwrap();
    let mut writer = BufWriter::new(file);

    for result in packet_stream {
        match result {
            Ok(packet) => {
                let packet_str = format!("\n>>{},{},{},{}", packet.recv_time, packet.origin, packet.destination, packet.protocol);

                match writer.write_all(packet_str.as_bytes()) {
                    Ok(_) => {}
                    Err(err) => {
                        eprint!("Error writing to file: {err}");
                    }
                }

                match writer.write_all(packet.payload.as_slice()) {
                    Ok(_) => {}
                    Err(err) => {
                        eprint!("Error writing to file: {err}");
                    }
                }
            }
            Err(err) => {
                eprint!("Error while processing packet: {}", err);
            }
        }
    }
}

fn main() {
    println!("Hello, world!");

    aprs_pipeline()
}
