enum MessageId {
    KeepAlive = -1,
    Choke = 0,
    Unchoke = 1,
    Interested = 2,
    NotInterested = 3,
    Have = 4,
    BitField = 5,
    Request = 6,
    Piece = 7,
    Cancel = 8,
    Port = 9,
}

trait BitTorrentMessageMethods {
    fn new(id: u8, payload: String) -> Self;
    fn to_string(&self) -> String;
    fn get_message_id(&self) -> u8;
    fn get_payload(&self) -> String;
}

#[derive(Default, Debug)]
pub struct BitTorrentMessage {
    message_length: u32,
    pub id: u8,
    pub payload: String,
}

impl BitTorrentMessageMethods for BitTorrentMessage {
    fn new(id: u8, payload: String) -> Self {
        let message_length = payload.len() as u32 + 1;
        Self {
            id,
            payload,
            message_length,
        }
    }

    fn to_string(&self) -> String {
        // this function transforms into big_endian assuming
        // that locally it is in little_endian
        // TODO: Corroborar que el formato que se maneja localmente sea little endian
        format!(
            "{}{:02X?}{}",
            self.id,
            self.message_length.to_be_bytes(),
            self.payload
        )
    }

    /// Returns the id of the BitTorrentMessage instance.
    fn get_message_id(&self) -> u8 {
        self.id
    }

    /// Returns the payload of the BitTorrentMessage instance.
    fn get_payload(&self) -> String {
        self.payload.clone()
    }
}
