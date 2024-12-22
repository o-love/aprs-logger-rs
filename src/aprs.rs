use std::time::Instant;

pub struct AprsPacket {
    pub recv_time: Instant,
    pub origin: String,
    pub destination: String,
    pub protocol: String,
    pub payload: Vec<u8>,
}