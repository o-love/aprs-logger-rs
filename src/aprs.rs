use chrono::{DateTime, Utc};

pub struct AprsPacket {
    pub recv_time: DateTime<Utc>,
    pub origin: String,
    pub destination: String,
    pub protocol: String,
    pub payload: Vec<u8>,
}