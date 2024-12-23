use std::string::FromUtf8Error;
use chrono::Utc;
use crate::aprs::AprsPacket;

pub fn parse_aprs_tnc2_line(line: &[u8]) -> Result<AprsPacket, FromUtf8Error> {
    let now = Utc::now();
    
    let (origin, remainder) = split_at_char(line, b'>');
    let (destination, remainder) = split_at_char(remainder, b',');
    let (protocol, remainder) = split_at_char(remainder, b',');
    let payload = remainder;
    
    
    Ok(AprsPacket {
        recv_time: now,
        origin: String::from_utf8(origin.to_vec())?,
        destination: String::from_utf8(destination.to_vec())?,
        protocol: String::from_utf8(protocol.to_vec())?,
        payload: payload.to_vec(),
    })
}

fn split_at_char(line: &[u8], split_char: u8) -> (&[u8], &[u8]) {
    let mut index = 0;
    while index < line.len() {
        if line[index] == split_char {
            return (&line[0..index], &line[index + 1..]);
        }
        
        index += 1;
    }
    
    (line, &[])
}