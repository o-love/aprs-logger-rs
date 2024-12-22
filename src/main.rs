use tokio::io::BufReader;
use aprs_logger::aprsis::cnx::start_default_aprs_is_stream;
use aprs_logger::aprsis::processor::parse_aprs_tnc2_line;
use aprs_logger::stream_processor::{process_stream};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    
    let tcp_stream = start_default_aprs_is_stream().await.unwrap();
    let tcp_stream = BufReader::new(tcp_stream);
    
    println!("tcp stream started");
   
    

    let input_processor = |line: &[u8]| {
        match parse_aprs_tnc2_line(line) {
            Ok(packet) => {Some(packet)}
            Err(err) => {
                eprint!("Invalid line: ");
                
                for c in line {
                    eprint!("{}", *c as char);
                }
                eprintln!();
                
                None
            }
        }
    };

    let packet_stream = process_stream(tcp_stream, input_processor);
    
    for result in packet_stream {
        match result {
            Ok(packet) => {
                print!("From: {}; To: {}", packet.origin, packet.destination);
                for c in packet.payload{
                    print!("{}", c as char);
                }
            }
            Err(err) => {
                eprint!("Error while processing packet: {}", err);
            }
        }
    }
}
